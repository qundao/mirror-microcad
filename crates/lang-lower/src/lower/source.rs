// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, LowerErrorsWithSource, ir};

use microcad_lang_base::{Diagnostics, Hashed, SrcReferrer, Url, virtual_url};
use microcad_lang_parse::ast;

impl Lower<ast::Source> for ir::Source {
    fn lower(node: &ast::Source, context: &mut LowerContext) -> super::LowerResult<Self> {
        todo!()
    }
}
