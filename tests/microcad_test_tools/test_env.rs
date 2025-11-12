// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Markdown test environment

use crate::output::Output;

/// Markdown test environment
pub struct TestEnv {
    orig_name: String,
    name: String,
    path: std::path::PathBuf,
    mode: String,
    params: Option<String>,
    code: String,
    start_no: usize,
    log_file: Option<std::fs::File>,
}

/// Markdown test result
pub enum TestResult {
    /// Ok
    Ok,
    /// Ok to fail
    FailOk,
    /// Marked as todo but is ok
    NotTodo,
    /// Todo but fail intentionally
    NotTodoFail,
    /// Fails
    Fail,
    /// Fails with wrong errors
    FailWrong,
    /// s ok but was meant to fail
    OkFail,
    /// Work in progress
    Todo,
    /// Work in progress (should fail)
    TodoFail,
}

impl std::fmt::Display for TestEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"microcad_test_tools::test_env::TestEnv::new({path:?}, {orig_name:?}, {code:?}, {start_no:?})"#,
            path = self.path,
            orig_name = self.orig_name,
            code = self.code,
            start_no = self.start_no
        )
    }
}

impl std::fmt::Debug for TestEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "   Test name: {}", self.name())?;
        writeln!(f, "        Mode: {}", self.mode())?;
        writeln!(f, "      Params: {}", self.params())?;
        writeln!(f, " Source file: {:?}", self.source_path())?;
        writeln!(f, "   Test path: {:?}", self.test_path())?;
        writeln!(f, " Banner file: {:?}", self.banner_file())?;
        writeln!(f, "    Log file: {:?}", self.log_file())?;
        writeln!(f, "Output files: {:?}", self.out_file_path_stem())
    }
}

impl TestEnv {
    /// Create new test environment
    /// # Arguments
    /// - `path`: Path of the markdown file which includes the source file.
    /// - `name`: Name of the test (including mode and params, e.g. `my_test#ok(hires)`).
    /// - `code`: Source code of the test.
    /// - `reference`: Description of the position in the source file (e.g. `path_to/file.md:5:0`).
    pub fn new(
        path: impl AsRef<std::path::Path>,
        name: &str,
        code: &str,
        start_no: usize,
    ) -> Option<Self> {
        let orig_name = name.to_string();
        // split name into `name` and optional `mode`
        let (name, mode) = if let Some((name, mode)) = name.split_once('#') {
            (name, Some(mode))
        } else {
            (name, None)
        };

        if mode == Some("no_test") {
            None
        } else {
            let (name, params) = if let Some((name, params)) = name.split_once('(') {
                if params.ends_with(")") {
                    (name, Some(&params[0..params.len() - 1]))
                } else {
                    (name, None)
                }
            } else {
                (name, None)
            };

            Some(Self {
                orig_name,
                name: name.to_string(),
                path: path.as_ref().to_path_buf(),
                mode: mode.unwrap_or("ok").to_string(),
                params: params.map(|p| p.to_string()),
                code: code.into(),
                start_no,
                log_file: None,
            })
        }
    }

    /// Generate the test call.
    /// - `output`: A string to append the test output to.
    pub fn generate(&mut self, output: &mut String) -> Output {
        output.push_str(&format!(
            r##"
        #[test]
        #[allow(non_snake_case)]
        fn r#{name}() {{
            crate::markdown_test::run_test({self});
        }}"##,
            name = self.name
        ));

        std::fs::create_dir_all(self.test_path()).expect("cant create dir");

        Output::new(
            self.name().into(),
            self.path.clone(),
            self.banner_file(),
            self.out_file_path_stem(),
            self.log_file(),
            &["svg", "stl"],
        )
    }

    /// Create log file for he test.
    pub fn start_log(&mut self) {
        // create log file
        self.log_file = Some(
            std::fs::File::create(self.log_file())
                .unwrap_or_else(|_| panic!("{:?}", self.log_file())),
        );
    }

    /// Return test name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return test source code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return test parameters.
    fn params(&self) -> String {
        self.params.clone().unwrap_or_default()
    }

    /// Return test mode (ok, fail, todo, etc.).
    pub fn mode(&self) -> &str {
        &self.mode
    }

    /// Return path where to store any test output.
    pub fn source_path(&self) -> std::path::PathBuf {
        self.path.clone()
    }

    /// Return path where to store any test output.
    pub fn test_path(&self) -> std::path::PathBuf {
        self.path.parent().unwrap().join(".test")
    }

    /// Return test banner filename as string.
    pub fn banner(&self) -> String {
        self.banner_file()
            .to_string_lossy()
            .escape_default()
            .to_string()
    }

    /// Return test banner filename as path.
    pub fn banner_file(&self) -> std::path::PathBuf {
        self.test_path().join(format!("{}.svg", self.name()))
    }

    /// Return log file path.
    pub fn log_file(&self) -> std::path::PathBuf {
        self.test_path().join(format!("{}.log", self.name()))
    }

    /// Return output file path (without any file extension).
    pub fn out_file_path_stem(&self) -> std::path::PathBuf {
        self.test_path().join(format!("{}-out", self.name()))
    }

    /// Return output file path with given extension.
    pub fn out_file(&self, ext: &str) -> std::path::PathBuf {
        self.out_file_path_stem().with_extension(ext)
    }

    /// Return if test mode is todo.
    pub fn todo(&self) -> bool {
        matches!(self.mode(), "todo" | "todo_fail")
    }

    /// Return if parameter `hires` is set.
    pub fn hires(&self) -> bool {
        self.params() == "hires"
    }

    /// Return markdown file reference of the test.
    pub fn reference(&self) -> String {
        format!(
            "{}:{}",
            self.source_path().to_str().expect("valid path"),
            self.start_no
        )
    }

    /// Map line number into MD-line number.
    pub fn offset_line(&self, line_no: usize) -> usize {
        line_no + self.start_no
    }

    /// Map line number into MD-line number.
    pub fn offset(&self) -> usize {
        self.start_no
    }

    /// Write into test log (end line with LF).
    pub fn log_ln(&mut self, text: &str) {
        if let Some(mut log_file) = self.log_file.as_mut() {
            let log_out = &mut std::io::BufWriter::new(&mut log_file);
            use std::io::Write;
            writeln!(log_out, "{}", text).expect("output error")
        }
    }

    /// Write into test log (no LF at end).
    pub fn log(&mut self, text: &str) {
        if let Some(mut log_file) = self.log_file.as_mut() {
            let log_out = &mut std::io::BufWriter::new(&mut log_file);
            use std::io::Write;
            write!(log_out, "{}", text).expect("output error")
        }
    }

    /// Report output into log file.
    pub fn report_output(&mut self, output: Option<String>) {
        self.log_ln(&format!(
            "-- Output --\n{}",
            output.unwrap_or("output error".into())
        ));
    }

    /// Report errors into log file.
    pub fn report_errors(&mut self, diagnosis: String) {
        self.log(&format!("-- Errors --\n{diagnosis}"));
    }

    fn diff(
        &mut self,
        left: &std::collections::HashSet<usize>,
        right: &std::collections::HashSet<usize>,
        message: &str,
    ) -> bool {
        let mut diff = left.difference(right).collect::<Vec<_>>();
        if diff.is_empty() {
            true
        } else {
            diff.sort();
            let diff = diff.iter().map(|line| line.to_string()).collect::<Vec<_>>();
            let message = format!("{message}: {}", diff.join(", "));
            log::trace!("{message}");
            self.log_ln(&message);
            false
        }
    }

    /// Return if code includes error or warning marker comments
    pub fn has_error_markers(&self) -> bool {
        self.code.lines().any(|line| line.contains("// error"))
            || self.code.lines().any(|line| line.contains("// warning"))
    }

    /// Report wrong errors into log file.
    pub fn report_wrong_errors(
        &mut self,
        error_lines: &std::collections::HashSet<usize>,
        warning_lines: &std::collections::HashSet<usize>,
    ) -> bool {
        fn lines_with(code: &str, marker: &str, offset: usize) -> std::collections::HashSet<usize> {
            code.lines()
                .enumerate()
                .filter_map(|line| {
                    if line.1.contains(marker) {
                        Some(line.0 + offset)
                    } else {
                        None
                    }
                })
                .collect()
        }

        let lines_with_error = lines_with(self.code(), "// error", self.offset());
        let lines_with_warning = lines_with(self.code(), "// warning", self.offset());

        let errors_ok = self.diff(
            &lines_with_error,
            error_lines,
            "Expected error(s) which did not occur in line(s)",
        ) && self.diff(
            error_lines,
            &lines_with_error,
            "Unexpected error(s) which did occur in line(s)",
        );

        let warnings_ok = self.diff(
            &lines_with_warning,
            warning_lines,
            "Expected warnings(s) which did not occur in line(s)",
        ) && self.diff(
            warning_lines,
            &lines_with_warning,
            "Unexpected warnings(s) which did occur in line(s)",
        );

        !errors_ok || !warnings_ok
    }

    /// Report result into log file.
    pub fn result(&mut self, result: TestResult) {
        let (res, res_long) = match result {
            TestResult::Ok => ("ok", "OK"),
            TestResult::Todo => ("todo", "TODO"),
            TestResult::NotTodo => ("not_todo", "OK BUT IS TODO"),
            TestResult::Fail => ("fail", "FAIL"),
            TestResult::FailWrong => ("fail_wrong", "FAILED WITH WRONG ERRORS/WARNINGS"),
            TestResult::FailOk => ("fail_ok", "FAILED AS EXPECTED"),
            TestResult::NotTodoFail => ("not_todo_fail", "FAILED AS EXPECTED BUT IS TODO"),
            TestResult::TodoFail => ("todo_fail", "FAIL (TODO)"),
            TestResult::OkFail => ("ok_fail", "OK BUT SHOULD FAIL"),
        };
        let _ = std::fs::remove_file(self.banner_file());
        let _ = std::fs::hard_link(format!("images/{res}.svg"), self.banner_file());
        self.log_ln(&format!("-- Test Result --\n{res_long}"));
    }
}
