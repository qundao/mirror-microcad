// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::Symbol;

use crate::{commands, document};

#[derive(Default)]
pub enum State {
    #[default]
    Raw,
    Symbol(Symbol),
}

impl document::GetAssetSymbol for document::BuiltinAsset {
    fn get_symbol(&self) -> document::Result<Symbol> {
        let symbol = microcad_builtin::__builtin();
        self.transition(|_| Ok(State::Symbol(symbol.clone())))?;
        Ok(symbol)
    }
}

impl commands::DocGen for document::BuiltinAsset {}
