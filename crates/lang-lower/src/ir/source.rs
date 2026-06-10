// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source file representation

use crate::ir;

use microcad_lang_base::{
    ComputedHash, LineCol, ResourceLocation, SourceLocInfo, SrcRef, SrcReferrer,
};

/// µcad source file
#[derive(Debug)]
pub struct Source {
    /// Inner attributes.
    pub attr: Option<ir::DocBlock>,

    /// List of file modules: `mod foo;`.
    pub file_modules: ir::FileModules,
    /// Use statements: `use ...`.
    pub aliases: ir::Aliases,
    /// Inline modules: `mod bar {...}`.
    pub inline_modules: ir::InlineModules,
    /// Constants: `const FOO = 42;`.
    pub constants: ir::Constants,
    /// Functions: `fn foo(...) {...}`.
    pub functions: ir::Functions,
    /// Workbenches: `part Bar(...) {...}`.
    pub workbenches: ir::Workbenches,

    /// Root code body.
    pub statements: ir::WorkbenchStatements,
    /// Original source
    pub source: microcad_lang_base::Source,
}

impl Source {
    pub fn with_line_offset(self, line_offset: u32) -> Self {
        let mut src = self;
        src.source.line_offset = line_offset;
        src
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
            code: &self.source.code,
            url: self.source.url.clone(),
            line_offset: self.source.line_offset,
        }
    }
}

impl ResourceLocation for Source {
    fn url(&self) -> &microcad_lang_base::Url {
        &self.source.url
    }
}

impl SrcReferrer for Source {
    fn src_ref(&self) -> SrcRef {
        SrcRef::new(
            0..self.source.code.len(),
            LineCol::default(),
            self.source.code.computed_hash(),
        )
    }
}
