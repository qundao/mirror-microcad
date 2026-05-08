// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source API

use microcad_lang_base::{
    ComputedHash, Diagnostic, Diagnostics, GetSourceLocInfoByHash, Hashed, PushDiag, Refer,
    SourceLocInfo, SrcRef, SrcReferrer, Url,
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
        let ast = crate::parse(context.source.value()).map_err(|errors| {
            let mut diag_list = Diagnostics::default();

            for err in errors {
                let span = err.span.clone();
                diag_list
                    .push_diag(Diagnostic::Error(Refer::new(
                        err.into(),
                        context.src_ref(&span),
                    )))
                    .expect("Diag list should return no error");
            }

            diag_list
        })?;
        let src_ref = context.src_ref(&ast.span);

        Ok(Self {
            url: context.url.clone(),
            ast: Refer::new(ast, src_ref),
            line_offset: context.line_offset,
            source: context.source.clone().map(|s| s.to_string()),
        })
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
