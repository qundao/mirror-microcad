// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Output of a markdown test.

use std::path::PathBuf;

pub struct TestList(Vec<TestOutput>);

#[derive(Default)]
pub struct TestModule {
    pub submodules: std::collections::HashMap<String, TestModule>,
    pub outputs: Vec<(String, TestOutput)>,
}

impl TestModule {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        todo!()
    }

    pub fn test_code(&self, name: &str) -> String {
        let mut output = String::new();

        // 1. Generate child submodules recursively
        for (sub_name, sub_module) in &self.submodules {
            // Sanitize name to handle spaces or hyphens if necessary
            let sanitized_name = sub_name.replace(['.', '-', ' '], "_");

            let sub_code = sub_module.test_code(&sanitized_name);

            output.push_str(&format!(
                "#[allow(non_snake_case)]\nmod r#{sanitized_name} {{\n{sub_code}}}\n\n"
            ));
        }

        // 2. Append the actual test content/outputs for this specific module
        for (test, _) in &self.outputs {
            // Assuming Output has a method or field providing the code string
            output.push_str(&test);
            output.push_str("\n");
        }

        output
    }

    pub fn test_list() -> TestList {
        todo!()
    }
}

/// Output of a markdown test.
#[derive(Clone)]
pub struct TestOutput {
    /// Name of the test
    pub name: String,
    input: PathBuf,
    banner: PathBuf,
    exports: Vec<PathBuf>,
    out: PathBuf,
    log: PathBuf,
    externals: Vec<PathBuf>,
}

impl TestOutput {
    /// Create new output.
    pub fn new(
        name: String,
        input: PathBuf,
        banner: PathBuf,
        out: PathBuf,
        log: PathBuf,
        export_ext: &[&str],
    ) -> Self {
        Self {
            name,
            input,
            banner,
            exports: {
                export_ext
                    .iter()
                    .map(|ext| {
                        let mut export_path = out.clone();
                        export_path.set_extension(ext);
                        export_path
                    })
                    .collect()
            },
            out,
            log,
            externals: Vec::new(),
        }
    }

    /// Check if a path is one of the output files.
    pub fn has_path(&self, path: &PathBuf) -> bool {
        self.banner == *path
            || self.out == *path
            || self.log == *path
            || self.exports.contains(path)
            || self.externals.contains(path)
    }

    /// Add an output file.
    pub fn add_output(&mut self, path: PathBuf) {
        self.externals.push(path);
    }

    /// Return a test list table row.
    pub fn table_row(&self, base_path: impl AsRef<std::path::Path>) -> String {
        let base_path = base_path.as_ref();
        let banner = &self.banner;
        let input = &self.input;
        let log = &self.log;

        let e = &format!(
            "wrong paths: {:?}\n{base_path:?}\n{banner:?}\n{input:?}\n{log:?}",
            std::env::current_dir().expect("current dir")
        );

        use pathdiff::diff_paths;
        let banner = diff_paths(banner, base_path).expect(e);
        let input = diff_paths(input, base_path).expect(e);
        let log = diff_paths(log, base_path).expect(e);

        let banner = banner.to_str().expect(e);
        let input = input.to_str().expect(e);
        let log = log.to_str().expect(e);

        let input = match input.strip_suffix("README.md") {
            Some(input) => input,
            None => input,
        };

        format!(
            "| [![test]({banner})]({log}) | [{name}]({input}) |\n",
            name = self.name,
        )
    }
}

impl Eq for TestOutput {}

impl PartialEq for TestOutput {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase().eq(&other.name.to_lowercase())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for TestOutput {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name
            .to_lowercase()
            .partial_cmp(&other.name.to_lowercase())
    }
}

impl Ord for TestOutput {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}
