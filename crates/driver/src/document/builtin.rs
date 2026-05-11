// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::Symbol;

use crate::{commands, document};

#[derive(Default, Debug)]
pub struct State {
    symbol: Option<Symbol>,
}

impl document::GetAssetSymbol for document::Builtin {
    fn get_symbol(&self) -> document::Result<Symbol> {
        let state = &mut *self.state.borrow_mut();
        if let Some(symbol) = &state.symbol {
            return Ok(symbol.clone());
        }

        let symbol = microcad_builtin::__builtin();
        state.symbol = Some(symbol.clone());

        Ok(symbol)
    }
}

impl commands::DocGen for document::Builtin {}
