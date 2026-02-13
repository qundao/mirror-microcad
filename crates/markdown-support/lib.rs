// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Markdown support library

use microcad_lang::syntax::DocBlock;

trait ToMdAst {
    fn to_md_std(&self) -> markdown::mdast::Node;
}

impl ToMdAst for DocBlock {
    fn to_md_std(&self) -> markdown::mdast::Node {}
}
