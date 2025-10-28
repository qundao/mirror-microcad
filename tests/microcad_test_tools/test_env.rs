use crate::output::Output;

pub struct TestEnv {
    orig_name: String,
    name: String,
    path: std::path::PathBuf,
    mode: String,
    params: Option<String>,
    code: String,
    reference: String,
    log_file: Option<std::fs::File>,
}

pub enum TestResult {
    Ok,
    Todo,
    NotTodo,
    Fail,
    FailWrong,
    FailOk,
    TodoFail,
    NotTodoFail,
    OkFail,
}

impl std::fmt::Display for TestEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"microcad_test_tools::test_env::TestEnv::new({path:?}, {orig_name:?}, {code:?}, {reference:?})"#,
            path = self.path,
            orig_name = self.orig_name,
            code = self.code,
            reference = self.reference
        )
    }
}

impl TestEnv {
    pub fn new(
        path: impl AsRef<std::path::Path>,
        name: &str,
        code: &str,
        reference: &str,
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
                reference: reference.to_string(),
                log_file: None,
            })
        }
    }

    pub fn run(&mut self, output: &mut String) -> Output {
        output.push_str(&format!(
            r##"
        #[test]
        #[allow(non_snake_case)]
        fn r#{name}() {{
            crate::markdown_test::run_test({self});
        }}"##,
            name = self.name
        ));

        Output::new(
            self.name().into(),
            self.path.clone(),
            self.banner_file(),
            self.out_file_path_stem(),
            self.log_file(),
            &["svg", "stl"],
        )
    }

    pub fn start_log(&mut self) {
        // create log file
        self.log_file =
            Some(std::fs::File::create(self.log_file()).expect("cannot create log file"));
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    fn params(&self) -> String {
        self.params.clone().unwrap_or_default()
    }
    pub fn mode(&self) -> &str {
        &self.mode
    }
    fn test_path(&self) -> std::path::PathBuf {
        self.path.parent().unwrap().join(".test")
    }
    pub fn banner(&self) -> String {
        self.banner_file()
            .to_string_lossy()
            .escape_default()
            .to_string()
    }
    pub fn banner_file(&self) -> std::path::PathBuf {
        self.test_path().join(format!("{}.svg", self.name()))
    }
    pub fn log_file(&self) -> std::path::PathBuf {
        self.test_path().join(format!("{}.log", self.name()))
    }
    pub fn out_file_path_stem(&self) -> std::path::PathBuf {
        self.test_path().join(format!("{}-out", self.name()))
    }
    pub fn out_file(&self, ext: &str) -> std::path::PathBuf {
        self.out_file_path_stem().with_extension(ext)
    }
    pub fn todo(&self) -> bool {
        matches!(self.mode(), "todo" | "todo_fail")
    }
    pub fn hires(&self) -> bool {
        self.params() == "hires"
    }
    pub fn reference(&self) -> String {
        self.reference.clone()
    }

    pub fn log_ln(&mut self, text: &str) {
        if let Some(mut log_file) = self.log_file.as_mut() {
            let log_out = &mut std::io::BufWriter::new(&mut log_file);
            use std::io::Write;
            writeln!(log_out, "{}", text).expect("output error")
        }
    }

    pub fn log(&mut self, text: &str) {
        if let Some(mut log_file) = self.log_file.as_mut() {
            let log_out = &mut std::io::BufWriter::new(&mut log_file);
            use std::io::Write;
            write!(log_out, "{}", text).expect("output error")
        }
    }

    pub fn report_output(&mut self, output: Option<String>) {
        self.log_ln(&format!(
            "-- Output --{}",
            output.unwrap_or("output error".into())
        ));
    }

    pub fn report_errors(&mut self, diagnosis: String) {
        self.log(&format!("-- Errors --\n{diagnosis}"));
    }

    pub fn result(&mut self, result: TestResult) {
        let (res, res_long) = match result {
            TestResult::Ok => ("ok", "OK"),
            TestResult::Todo => ("todo", "TODO"),
            TestResult::NotTodo => ("not_todo", "OK BUT IS TODO"),
            TestResult::Fail => ("fail.svg", "FAIL"),
            TestResult::FailWrong => ("fail_wrong", "FAILED BUT WITH WRONG ERRORS"),
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
