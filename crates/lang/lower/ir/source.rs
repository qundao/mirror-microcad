// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source file representation

use crate::lower::ir;

use microcad_lang_base::{
    ComputedHash, Hashed, Identifier, LineCol, SourceKind, SourceLocInfo, SrcRef, SrcReferrer, Url,
};

/// µcad source file
#[derive(Clone, Debug)]
pub struct Source {
    /// Documentation.
    pub doc: Option<ir::DocBlock>,
    /// Qualified name of the file if loaded from externals
    pub name: ir::QualifiedName,
    /// Root code body.
    pub statements: ir::StatementList,
    /// Name of loaded file.
    pub url: Url,
    /// Source file string with hash
    pub source: Hashed<String>,
    /// Line offset
    pub line_offset: u32,
}

impl Source {
    /// Create new source file from existing source.
    pub fn new(
        doc: Option<ir::DocBlock>,
        statements: ir::StatementList,
        source: Hashed<String>,
        url: Url,
    ) -> Self {
        Self {
            doc,
            statements,
            source,
            url,
            name: ir::QualifiedName::default(),
            line_offset: 0,
        }
    }

    pub fn with_line_offset(self, line_offset: u32) -> Self {
        let mut src = self;
        src.line_offset = line_offset;
        src
    }

    /// Return the module name from the file name
    pub fn id(&self) -> Identifier {
        self.name.last().unwrap_or(&Identifier::none()).clone()
    }

    /// Return filename of loaded file or `<NO FILE>`
    pub fn filename(&self) -> std::path::PathBuf {
        self.to_file_path()
            .unwrap_or(std::path::PathBuf::from("<NO FILE>"))
    }

    pub fn to_file_path(&self) -> Option<std::path::PathBuf> {
        SourceKind::from(self.url.clone()).path()
    }

    /// Return filename of loaded file or `<NO FILE>`
    pub fn set_filename(&mut self, path: impl AsRef<std::path::Path>) {
        assert!(self.to_file_path().is_none());
        self.url = Url::from_file_path(
            path.as_ref()
                .canonicalize()
                .unwrap_or(path.as_ref().to_path_buf()),
        )
        .unwrap_or(self.url.clone());
    }

    /// get a specific line
    ///
    /// - `line`: line number beginning at `0`
    pub fn get_code(&self, src_ref: &SrcRef) -> &str {
        &self.source[src_ref.start..src_ref.end]
    }

    /// Set file name.
    pub fn set_name(&mut self, name: ir::QualifiedName) {
        self.name = name
    }

    /// Get a miette source adapter for the SourceFile
    pub fn source_loc_info<'a>(&'a self) -> SourceLocInfo<'a> {
        SourceLocInfo {
            code: &self.source,
            url: self.url.clone(),
            line_offset: self.line_offset,
        }
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.statements.iter().try_for_each(|s| writeln!(f, "{s}"))
    }
}

impl SrcReferrer for Source {
    fn src_ref(&self) -> SrcRef {
        SrcRef::new(
            &(0..self.source.len()),
            LineCol::default(),
            self.source.computed_hash(),
        )
    }
}
