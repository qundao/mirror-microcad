// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source file representation

use crate::syntax::*;
use microcad_lang_base::{
    ComputedHash, Hashed, MietteSourceFile, SrcRef, SrcReferrer, TreeDisplay, TreeState,
};

/// µcad source file
#[derive(Clone, Debug)]
pub struct SourceFile {
    /// Documentation.
    pub doc: Option<DocBlock>,
    /// Qualified name of the file if loaded from externals
    pub name: QualifiedName,
    /// Root code body.
    pub statements: StatementList,
    /// Name of loaded file.
    pub filename: Option<std::path::PathBuf>,
    /// Source file string with hash
    pub source: Hashed<String>,
}

impl SourceFile {
    /// Create new source file from existing source.
    pub fn new(doc: Option<DocBlock>, statements: StatementList, source: Hashed<String>) -> Self {
        Self {
            doc,
            statements,
            source,
            filename: None,
            name: QualifiedName::default(),
        }
    }

    /// Return filename of loaded file or `<NO FILE>`
    pub fn filename(&self) -> std::path::PathBuf {
        self.filename
            .clone()
            .unwrap_or(std::path::PathBuf::from("<NO FILE>"))
    }

    /// Return filename of loaded file or `<NO FILE>`
    pub fn set_filename(&mut self, path: impl AsRef<std::path::Path>) {
        assert!(self.filename.is_none());
        self.filename = Some(
            path.as_ref()
                .canonicalize()
                .unwrap_or(path.as_ref().to_path_buf()),
        )
    }

    /// Return filename of loaded file or `<no file>`
    pub fn filename_as_str(&self) -> &str {
        self.filename
            .as_ref()
            .map(|f| f.to_str().expect("File name error {filename:?}"))
            .unwrap_or("NO FILE")
    }

    /// Return the module name from the file name
    pub fn id(&self) -> Identifier {
        self.name.last().unwrap_or(&Identifier::none()).clone()
    }

    /// get a specific line
    ///
    /// - `line`: line number beginning at `0`
    pub fn get_code(&self, src_ref: &SrcRef) -> &str {
        let range = &src_ref.as_ref().expect("source reference empty").range;
        &self.source[range.start..range.end]
    }

    /// Set file name.
    pub fn set_name(&mut self, name: QualifiedName) {
        self.name = name
    }

    /// Get a miette source adapter for the SourceFile
    pub fn miette_source<'a>(&'a self, path: String, line_offset: usize) -> MietteSourceFile<'a> {
        MietteSourceFile {
            source: &self.source,
            name: path,
            line_offset,
        }
    }
}

impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.statements.iter().try_for_each(|s| writeln!(f, "{s}"))
    }
}

impl TreeDisplay for SourceFile {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}SourceFile '{:?}' ({}):",
            "",
            self.id(),
            self.filename_as_str()
        )?;
        depth.indent();
        if let Some(doc) = &self.doc {
            doc.tree_print(f, depth)?;
        }
        self.statements
            .iter()
            .try_for_each(|s| s.tree_print(f, depth))
    }
}

impl SrcReferrer for SourceFile {
    fn src_ref(&self) -> SrcRef {
        SrcRef::new(0..self.source.len(), 0, 0, self.source.computed_hash())
    }
}

#[test]
fn load_source_file_wrong_location() {
    let source_file = SourceFile::load("I do not exist.µcad");
    if let Err(err) = source_file {
        log::info!("{err}");
        //assert_eq!(format!("{err}"), "Cannot load source file");
    } else {
        panic!("Does file exist?");
    }
}
