// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source file representation

use crate::{ir, is_default};

use microcad_lang_base::{
    ComputedHash, LineCol, ResourceLocation, SourceLocInfo, SrcRef, SrcReferrer,
};
use serde::Serialize;

/// µcad source file
#[derive(Debug, Serialize)]
pub struct Source {
    /// Inner attributes.
    #[serde(skip_serializing_if = "is_default", default)]
    pub attr: ir::InnerAttributes,
    /// List of file modules: `mod foo;`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub file_modules: ir::FileModules,
    /// Inline modules: `mod bar {...}`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub inline_modules: ir::InlineModules,
    /// Use statements: `use ...`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub aliases: ir::Aliases,
    /// Constants: `const FOO = 42;`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub constants: ir::Constants,
    /// Functions: `fn foo(...) {...}`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub functions: ir::Functions,
    /// Workbenches: `part Bar(...) {...}`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub workbenches: ir::Workbenches,
    /// Workbench statements
    #[serde(skip_serializing_if = "is_default", default)]
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
        &self.source.code[src_ref.start..src_ref.end]
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
