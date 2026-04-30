// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad markdown book support.

use std::collections::HashMap;

use thiserror::Error;

use crate::{CodeBlock, Markdown, MarkdownError};

#[derive(Debug, Error)]
pub enum MdBookError {
    /// Io Error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

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
pub struct MdBook {
    pub name: String,

    /// Relative paths to `src` folder in md book folder
    pub md_files: HashMap<std::path::PathBuf, Markdown>,

    /// Source directory inside book, usually `src`.
    pub src_path: std::path::PathBuf,
}

impl MdBook {
    /// Create a new [`MdBookDirectory`].
    ///
    /// Will fail if the directory does not contain a `book.toml` file.
    /// Scans the directory `src` recursively for markdown files ending with `.md`.
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self, MdBookError> {
        let root = path.as_ref();

        let root = if root.ends_with("book.toml") {
            root.parent()
                .map(|path| path.to_path_buf())
                .unwrap_or(std::env::current_dir()?)
        } else {
            root.to_path_buf()
        };

        // 1. Validate book.toml existence
        if !root.join("book.toml").exists() {
            return Err(MdBookError::NoMdBookDirectory(root));
        }

        // 2. Identify the src directory
        let src_path = root.join("src");
        let mut md_files = Vec::new();

        // 3. Recursively scan src for .md files
        if src_path.exists() && src_path.is_dir() {
            Self::visit_dirs(&src_path, &src_path, &mut md_files);
        }

        let md_files = md_files
            .iter()
            .map(|md_file| {
                (
                    md_file.clone(),
                    Markdown::load(src_path.join(md_file))
                        .unwrap_or_else(|_| panic!("No error: {}", md_file.display())),
                )
            })
            .collect();

        let name = root
            .file_name()
            .expect("Some directory name")
            .to_str()
            .expect("Valid string")
            .to_string();

        Ok(Self {
            name,
            src_path,
            md_files,
        })
    }

    pub fn abs_md_file(&self, md_file: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        self.src_path.join(md_file.as_ref())
    }

    pub fn save_all(&self) -> Result<(), MdBookError> {
        self.md_files.iter().try_for_each(|(md_file, md)| {
            md.save(self.abs_md_file(md_file))
                .map_err(|err| MdBookError::Parse {
                    file: md_file.clone(),
                    err,
                })
        })
    }

    /// Returns an iterator over all code blocks in the entire document.
    pub fn code_blocks(&self) -> impl Iterator<Item = (std::path::PathBuf, &CodeBlock)> {
        self.md_files.iter().flat_map(|(md_file, md)| {
            md.code_blocks()
                .map(|code_block| (md_file.clone(), code_block))
        })
    }

    /// Returns an iterator over all code blocks in the entire document.
    pub fn code_blocks_mut(
        &mut self,
    ) -> impl Iterator<Item = (std::path::PathBuf, &mut CodeBlock)> {
        self.md_files.iter_mut().flat_map(|(md_file, md)| {
            md.code_blocks_mut()
                .map(|code_block| (md_file.clone(), code_block))
        })
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
