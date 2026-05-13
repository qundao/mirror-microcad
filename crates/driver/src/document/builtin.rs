// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::Symbol;
use microcad_lang_base::{Diagnostics, RcMut, ResourceLocation, Url};

use crate::{
    commands,
    document::{self, CaptureDiags},
};

#[derive(Debug)]
pub struct Builtin {
    url: Url,
    symbol: Symbol,
}

impl Builtin {
    pub fn new() -> Self {
        Self {
            url: "builtin://__builtin".try_into().unwrap(),
            symbol: microcad_builtin::__builtin(),
        }
    }
}

impl CaptureDiags for Builtin {
    fn diags(&self) -> RcMut<Diagnostics> {
        Diagnostics::default().into()
    }
}

impl document::GetSymbol for Builtin {
    fn get_symbol(&self) -> document::Result<Symbol> {
        Ok(self.symbol.clone())
    }
}

impl ResourceLocation for Builtin {
    fn url(&self) -> &url::Url {
        &self.url
    }
}

impl commands::DocGen for Builtin {}
