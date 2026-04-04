// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod code_block;
mod markdown;
mod paragraph;
mod section;

pub use code_block::CodeBlock;
pub use markdown::{Markdown, MarkdownError};
pub use paragraph::Paragraph;
pub use section::Section;

pub mod mdbook {
    use thiserror::Error;

    use crate::{Markdown, MarkdownError};

    #[derive(Debug, Error)]
    pub enum MdBookDirectoryError {
        /// The directory does not contain an mdbook.
        #[error("No mdbook in directory: {0}")]
        NoMdBookDirectory(std::path::PathBuf),

        #[error("Error parsing markdown file `{file}`: {err}")]
        Parse {
            file: std::path::PathBuf,
            err: MarkdownError,
        },
    }

    /// Directory that contains a markdown book.
    pub struct MdBookDirectory {
        /// Relative paths to `src` folder in md book folder
        pub md_files: Vec<std::path::PathBuf>,
    }

    impl MdBookDirectory {
        /// Create a new [`MdBookDirectory`].
        ///
        /// Will fail if the directory does not contain a `book.toml` file.
        /// Scans the directory `src` recursively for markdown files ending with `.md`.
        pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self, MdBookDirectoryError> {
            let root = path.as_ref();

            // 1. Validate book.toml existence
            if !root.join("book.toml").exists() {
                return Err(MdBookDirectoryError::NoMdBookDirectory(root.to_path_buf()));
            }

            // 2. Identify the src directory
            let src_path = root.join("src");
            let mut md_files = Vec::new();

            // 3. Recursively scan src for .md files
            if src_path.exists() && src_path.is_dir() {
                Self::visit_dirs(&src_path, &src_path, &mut md_files);
            }

            Ok(Self { md_files })
        }

        pub fn update_book(&self) -> Result<(), MdBookDirectoryError> {
            self.md_files.iter().try_for_each(|md_file| {
                let md = Markdown::update(md_file).map_err(|err| MdBookDirectoryError::Parse {
                    file: md_file.clone(),
                    err,
                })?;
                Ok(())
            })
        }

        /*
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
        }*/

        pub fn generate_test_list(&self) -> Result<(), MdBookDirectoryError> {
            todo!()
            /*
                        let count = tests.len();
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

            {
                let mut tests = tests.iter().collect::<Vec<_>>().clone();
                tests.sort();
                tests.iter().for_each(|test| {
                    result.push_str(&test.table_row(path.as_ref().parent().expect("invalid path")));
                });
            }

            result
                     */
        }

        /// Helper to recursively find markdown files.
        ///
        /// Stores paths relative to the `src` folder.
        fn visit_dirs(
            src_root: &std::path::Path,
            dir: &std::path::Path,
            cb: &mut Vec<std::path::PathBuf>,
        ) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        Self::visit_dirs(src_root, &path, cb);
                    } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                        // Strip the src_root prefix to keep paths relative to src
                        if let Ok(relative) = path.strip_prefix(src_root) {
                            cb.push(relative.to_path_buf());
                        }
                    }
                }
            }
        }
    }
}
