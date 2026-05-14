// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Markdown test environment

use std::path::PathBuf;

use microcad_driver::{base::ResourceLocation, *};

use crate::output::TestOutput;

/// Markdown test environment
pub struct TestEnv {
    orig_name: String,
    name: String,
    mode: String,
    params: Option<String>,
    /// Source to be tested
    pub source: base::Source,
}

/// Markdown test result
pub enum TestResult {
    /// Ok
    Ok,
    /// Ok but has warning(s)
    OkWarn,
    /// Ok, but with wrong warning(s)
    OkWrong,
    /// Ok to fail
    FailOk,
    /// Fails
    Fail,
    /// Fails with wrong errors
    FailWrong,
    /// s ok but was meant to fail
    OkFail,
    /// Work in progress
    Todo,
}

impl std::fmt::Display for TestEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use base::ResourceLocation;
        write!(
            f,
            r#"microcad_test_tools::test_env::TestEnv::new({path:?}, {orig_name:?}, {code:?}, {line_offset:?})"#,
            path = self.source.to_file_path().unwrap(),
            orig_name = self.orig_name,
            code = self.source.code.value(),
            line_offset = self.source.line_offset
        )
    }
}

impl std::fmt::Debug for TestEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "        Test name: {}", self.name())?;
        writeln!(f, "  Expected result: {}", self.mode())?;
        if !self.params().is_empty() {
            writeln!(f, "           Params: {}", self.params())?;
        }
        let line_offset = self.source.line_offset + 1;
        writeln!(
            f,
            "      Source file: {}:{line_offset}",
            self.source_path().display()
        )?;
        writeln!(f, "        Test path: {}", self.test_path().display())
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
        line_offset: u32,
    ) -> Self {
        let orig_name = name.to_string();
        // split name into `name` and optional `mode`
        let (name, mode) = if let Some((name, mode)) = name.split_once('#') {
            (name, Some(mode))
        } else {
            (name, None)
        };

        let (name, params) = if let Some((name, params)) = name.split_once('(') {
            if params.ends_with(")") {
                (name, Some(&params[0..params.len() - 1]))
            } else {
                (name, None)
            }
        } else {
            (name, None)
        };

        let url = Url::from_file_path(path.as_ref().canonicalize().expect("Existing file"))
            .expect("A valid file URL");

        Self {
            orig_name,
            name: name.to_string(),
            mode: mode.unwrap_or("ok").to_string(),
            params: params.map(|p| p.to_string()),
            source: base::Source {
                url,
                line_offset,
                code: Hashed::new(code.to_string()),
            },
        }
    }

    /// Generate the test call.
    pub fn test_code(&self) -> String {
        format!(
            r##"
        #[test]
        #[allow(non_snake_case)]
        fn r#{name}() {{
            crate::markdown_test::run_test({self}).expect("No error");
        }}"##,
            name = self.name
        )
    }

    /// Return test output.
    pub fn output(&self) -> TestOutput {
        let mut output = TestOutput::new(
            self.name().into(),
            self.source_path(),
            self.banner_file(),
            self.out_file_path_stem(),
            self.log_file(),
            &["svg", "stl"],
        );

        let head = "// file: ";
        if let Some(first_line) = self.source.code.lines().find(|line| line.starts_with(head)) {
            if first_line.starts_with(head) {
                use std::io::Write;
                let (_, filename) = first_line.split_at(head.len());
                let filename = self.test_path().join(filename);
                let mut file = std::fs::File::create(filename.clone()).expect("cannot create file");
                file.write_all(self.source.code.as_bytes())
                    .expect("cannot write file");
                output.add_output(filename);
            }
        }

        std::fs::create_dir_all(self.test_path()).expect("cant create dir");

        output
    }

    /// Return test name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return test source code.
    pub fn code(&self) -> &str {
        &self.source.code.value()
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
    pub fn source_path(&self) -> PathBuf {
        pathdiff::diff_paths(
            self.source.to_file_path().expect("A valid file path"),
            std::env::current_dir().expect("Current dir"),
        )
        .unwrap()
    }

    /// Return path where to store any test output.
    pub fn test_path(&self) -> PathBuf {
        self.source_path().parent().unwrap().join(".test")
    }

    /// Return test banner filename as path.
    pub fn banner_file(&self) -> PathBuf {
        self.test_path().join(format!("{}.svg", self.name()))
    }

    /// Return log file path.
    pub fn log_file(&self) -> PathBuf {
        self.test_path().join(format!("{}.log", self.name()))
    }

    /// Return output file path (without any file extension).
    pub fn out_file_path_stem(&self) -> PathBuf {
        self.test_path().join(format!("{}-out", self.name()))
    }

    /// Return output file path with given extension.
    pub fn out_file(&self, ext: &str) -> PathBuf {
        self.out_file_path_stem().with_extension(ext)
    }

    /// Return if parameter `hires` is set.
    pub fn hires(&self) -> bool {
        self.params() == "hires"
    }

    /// Report output into log file.
    pub fn report_output(&self, output: Option<String>) -> String {
        let output = output.unwrap_or("output error".into());
        if output.is_empty() {
            "-- No Output --".to_string()
        } else {
            format!("-- Output --\n{output}")
        }
    }

    /// Report errors into log file.
    pub fn report_errors(&self, diagnosis: String) -> String {
        if diagnosis.is_empty() {
            "-- No Errors --".to_string()
        } else {
            format!("-- Errors --\n{diagnosis}")
        }
    }

    /// Return if code includes error or warning marker comments
    pub fn has_error_markers(&self) -> bool {
        self.source
            .code
            .lines()
            .any(|line| line.contains("// error"))
            || self
                .source
                .code
                .lines()
                .any(|line| line.contains("// warning"))
    }

    /// Report wrong errors into log file.
    pub fn report_wrong_errors(
        &self,
        error_lines: &HashSet<u32>,
        warning_lines: &HashSet<u32>,
    ) -> Option<String> {
        fn lines_with(code: &str, marker: &str, offset: u32) -> HashSet<u32> {
            code.lines()
                .enumerate()
                .filter_map(|line| {
                    if line.1.contains(marker) {
                        Some(line.0 as u32 + offset)
                    } else {
                        None
                    }
                })
                .collect()
        }

        fn diff(left: &HashSet<u32>, right: &HashSet<u32>, message: &str) -> Option<String> {
            let mut diff = left.difference(right).collect::<Vec<_>>();
            if diff.is_empty() {
                None
            } else {
                diff.sort();
                let diff = diff
                    .iter()
                    .map(|line| (**line + 1).to_string())
                    .collect::<Vec<_>>();
                let message = format!("{message}: {}\n", diff.join(", "));
                Some(message)
            }
        }

        let lines_with_error = lines_with(self.code(), "// error", self.source.line_offset);
        let lines_with_warning = lines_with(self.code(), "// warning", self.source.line_offset);
        let mut s = String::new();

        let expected_errors = diff(
            &lines_with_error,
            error_lines,
            "Expected error(s) which did not occur in line(s)",
        );

        let unexpected_errors = diff(
            error_lines,
            &lines_with_error,
            "Unexpected error(s) which did occur in line(s)",
        );
        let errors_ok = expected_errors.is_none() && unexpected_errors.is_none();

        s += &expected_errors.unwrap_or_default();
        s += &unexpected_errors.unwrap_or_default();

        let expected_warnings = diff(
            &lines_with_warning,
            warning_lines,
            "Expected warning(s) which did not occur in line(s)",
        );

        let unexpected_warnings = diff(
            warning_lines,
            &lines_with_warning,
            "Unexpected warning(s) which did occur in line(s)",
        );
        let warnings_ok = expected_warnings.is_none() && unexpected_warnings.is_none();

        s += &expected_warnings.unwrap_or_default();
        s += &unexpected_warnings.unwrap_or_default();

        if !errors_ok || !warnings_ok {
            Some(s)
        } else {
            None
        }
    }

    /// Report result into log file.
    pub fn result(&self, result: &TestResult) -> String {
        let (res, res_long) = match result {
            TestResult::Ok => ("ok", "OK"),
            TestResult::OkWarn => ("ok_warn", "OK (BUT WARNINGS)"),
            TestResult::OkWrong => ("ok_warn", "OK (BUT WRONG WARNINGS)"),
            TestResult::Todo => ("todo", "TODO"),
            TestResult::Fail => ("fail", "FAILS"),
            TestResult::FailWrong => ("fail_wrong", "FAILS WITH WRONG ERRORS"),
            TestResult::FailOk => ("fail_ok", "FAILS AS EXPECTED"),
            TestResult::OkFail => ("ok_fail", "OK BUT SHOULD FAIL"),
        };
        let _ = std::fs::remove_file(self.banner_file());
        let _ = std::fs::hard_link(format!("images/{res}.svg"), self.banner_file());
        format!("-- Test Result --\n{res_long}")
    }
}
