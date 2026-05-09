// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source API

use microcad_lang_base::{
    ComputedHash, Diagnostics, GetSourceLocInfoByHash, Hashed, Refer, SourceLocInfo, SrcRef,
    SrcReferrer, Url,
};

use crate::{Parse, ParseContext, ast};

/// A µcad source with a parse syntax tree with a line offset and the hashed original source code.
pub struct Source {
    /// The source url
    pub url: Url,
    /// Line offset
    pub line_offset: u32,
    /// The original source
    pub source: Hashed<String>,
    /// The µcad program
    pub ast: Refer<ast::Program>,
}

impl Parse for Source {
    fn parse(context: &ParseContext) -> Result<Self, Diagnostics> {
        match context {
            ParseContext::Element(_) => panic!("Expected parse source context"),
            ParseContext::Source {
                url,
                line_offset,
                source,
                ..
            } => {
                let ast = crate::parse(source.value())
                    .map_err(|errors| errors.to_diagnostics(context))?;
                let src_ref = context.src_ref(&ast.span);

                Ok(Self {
                    url: url.clone(),
                    ast: Refer::new(ast, src_ref),
                    line_offset: *line_offset,
                    source: source.clone().map(|s| s.to_string()),
                })
            }
        }
    }
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
        if hash == self.source.computed_hash() {
            Some(SourceLocInfo {
                source: &self.source,
                url: self.url.clone(),
                line_offset: self.line_offset,
            })
        } else {
            None
        }
    }
}
