// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Output of a markdown test.

use std::{ops::Deref, path::PathBuf};

/// A list of test outputs
pub struct TestList {
    /// Input path for tests
    path: std::path::PathBuf,
    /// Path containing the tests
    outputs: Vec<TestOutput>,
}

impl std::ops::Deref for TestList {
    type Target = Vec<TestOutput>;

    fn deref(&self) -> &Self::Target {
        &self.outputs
    }
}

impl TestList {
    pub fn new(path: std::path::PathBuf, mut outputs: Vec<TestOutput>) -> Self {
        outputs.sort();

        // Check adjacent elements for duplicates
        if let Some(duplicate) = outputs.windows(2).find(|w| w[0].name == w[1].name) {
            panic!("doublet test name '{}'", duplicate[0].name);
        }

        Self { path, outputs }
    }

    pub fn markdown_string(&self) -> String {
        let count = self.outputs.len();
        let mut result = format!(
            "# Test List

The following table lists all tests included in this documentation.

**{count}** tests have been evaluated with version **{version}** of microcad.

Click on the test names to jump to file with the test or click the buttons to get the logs.

| Result | Source | Name |
|-------:|--------|------|
",
            version = env!("CARGO_PKG_VERSION")
        );

        self.outputs.iter().for_each(|test| {
            result.push_str(&test.table_row(self.path.parent().expect("invalid path")));
        });

        result
    }

    pub fn write(&self, output_path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        use std::io::Write;
        std::fs::File::create(&output_path)?.write_all(self.markdown_string().as_bytes())?;
        println!("cargo:rerun-if-changed={}", output_path.as_ref().display());
        Ok(())
    }

    pub fn cargo(&self) {
        self.outputs.iter().for_each(|output| {
            println!("cargo:rerun-if-changed={}", output.input.display());
        });
    }
}

#[derive(Default)]
pub struct TestModule {
    pub submodules: std::collections::HashMap<String, TestModule>,
    pub outputs: Vec<(String, TestOutput)>,
}

impl TestModule {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        let mdbook = microcad_lang_markdown::MdBook::new(&path).expect("No error");

        let mut root = TestModule::default();

        mdbook.code_blocks().for_each(|(path, code_block)| {
            let header = &code_block.header;

            let mut name = match (&header.name, &header.fragment) {
                (None, _) => {
                    // We need a name
                    return;
                }
                (Some(name), None) => name.clone(),
                (Some(name), Some(fragment)) => format!("{name}#{fragment}"),
            };
            if !header.parameters.is_empty() {
                name += &format!("({})", header.parameters.join(","));
            }

            let env = crate::test_env::TestEnv::new(
                &mdbook.abs_md_file(&path),
                &name,
                &code_block.code,
                (code_block.line_offset + 1) as u32,
            );

            let mut current = &mut root;

            // Iterate through path components
            for component in path.components() {
                let name = component.as_os_str().to_string_lossy().into_owned();

                // Traverse deeper into the tree, creating modules if they don't exist
                current = current.submodules.entry(name).or_default();
            }

            let output = env.output();
            // Now 'current' is the specific leaf module for this path
            current.outputs.push((env.test_code(), output.clone()));
        });

        root
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

    /// Collects all TestOutputs from this module and all submodules.
    pub fn all_outputs(&self) -> Vec<&TestOutput> {
        let mut all = Vec::new();
        self.collect_recursive(&mut all);
        all
    }

    fn collect_recursive<'a>(&'a self, collector: &mut Vec<&'a TestOutput>) {
        // 1. Add outputs from the current module
        for (_, output) in &self.outputs {
            collector.push(output);
        }

        // 2. Recurse into submodules
        for submodule in self.submodules.values() {
            submodule.collect_recursive(collector);
        }
    }

    pub fn test_list(&self, test_list_file: impl AsRef<std::path::Path>) -> TestList {
        TestList::new(
            test_list_file.as_ref().to_path_buf(),
            self.all_outputs()
                .into_iter()
                .map(|output| output.clone())
                .collect(),
        )
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
