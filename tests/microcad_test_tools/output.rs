// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Output of a markdown test.

use std::path::PathBuf;

/// Output of a markdown test.
pub struct Output {
    name: String,
    input: PathBuf,
    banner: PathBuf,
    exports: Vec<PathBuf>,
    out: PathBuf,
    log: PathBuf,
    externals: Vec<PathBuf>,
}

impl Output {
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

        let e = &format!("wrong paths: {base_path:?}\n{banner:?}\n{input:?}\n{log:?}");

        let base_path = base_path.canonicalize().expect(e);
        let banner = self.banner.canonicalize().expect(e);
        let input = self.input.canonicalize().expect(e);
        let log = self.log.canonicalize().expect(e);

        use pathdiff::diff_paths;
        let banner = diff_paths(banner, &base_path).expect(e);
        let input = diff_paths(input, &base_path).expect(e);
        let log = diff_paths(log, &base_path).expect(e);

        let banner = banner.to_str().expect(e);
        let input = input.to_str().expect(e);
        let log = log.to_str().expect(e);

        format!(
            "| [![test]({banner})]({log}) | [{name}]({input}) |\n",
            name = self.name,
        )
    }
}

impl Eq for Output {}

impl PartialEq for Output {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase().eq(&other.name.to_lowercase())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Output {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name
            .to_lowercase()
            .partial_cmp(&other.name.to_lowercase())
    }
}

impl Ord for Output {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}
