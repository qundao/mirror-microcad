// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use tower_lsp::lsp_types as lsp;

use microcad_driver::prelude as mu;

pub trait ToLsp {
    type Output;

    fn to_lsp(&self) -> Self::Output;
}

impl ToLsp for mu::SrcRef {
    type Output = Option<lsp::Range>;

    fn to_lsp(&self) -> Self::Output {
        match self.is_some() {
            true => {
                let start = lsp::Position::new(self.at.line, self.at.col - 1);
                let end =
                    lsp::Position::new(self.at.line, (self.at.col + self.range.len() as u32) - 1);

                Some(lsp::Range::new(start, end))
            }
            false => None,
        }
    }
}
