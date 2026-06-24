// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::Symbol;
use microcad_lang_base::Diagnostics;

use crate::{Result, commands, document};

#[derive(Debug)]
pub struct Builtin {
    symbol: Symbol,
    diags: Diagnostics,
}

impl Builtin {
    pub fn new() -> Self {
        Self {
            symbol: microcad_builtin::__builtin(),
            diags: Diagnostics::default(),
        }
    }
}

impl document::CaptureDiags for Builtin {
    fn diags(&self) -> &Diagnostics {
        &self.diags
    }

    fn diags_mut(&mut self) -> &mut Diagnostics {
        &mut self.diags
    }
}

impl document::GetSymbol for Builtin {
    fn get_symbol(&mut self, _: impl Into<commands::compile::ResolveParameters>) -> Result<Symbol> {
        Ok(self.symbol.clone())
    }
}

impl commands::DocGen for Builtin {}
