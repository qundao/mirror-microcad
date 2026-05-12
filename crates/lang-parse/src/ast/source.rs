// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source API

use microcad_lang_base::{
    ComputedHash, GetSourceLocInfoByHash, Hashed, Refer, SourceLocInfo, SrcRef, SrcReferrer, Url,
};

use crate::ast;

/// A µcad source with a parse syntax tree with a line offset and the hashed original source code.
pub struct Source {
    /// The source url
    pub url: Url,
    /// Line offset
    pub line_offset: u32,
    /// The original code
    pub code: Hashed<String>,
    /// The µcad program
    pub ast: Refer<ast::Program>,
}

impl SrcReferrer for Source {
    fn src_ref(&self) -> SrcRef {
        self.ast.src_ref()
    }
}

impl GetSourceLocInfoByHash for Source {
    fn get_source_loc_info_by_hash(
        &'_ self,
        hash: microcad_lang_base::HashId,
    ) -> Option<microcad_lang_base::SourceLocInfo<'_>> {
        if hash == self.code.computed_hash() {
            Some(SourceLocInfo {
                code: &self.code,
                url: self.url.clone(),
                line_offset: self.line_offset,
            })
        } else {
            None
        }
    }
}
