// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::Diagnostics;

#[derive(Debug)]
pub struct Builtin {
    //symbol: Symbol,
    diags: Diagnostics,
}

impl Builtin {
    pub fn new() -> Self {
        Self {
            //symbol: microcad_builtin::__builtin(),
            diags: Diagnostics::default(),
        }
    }
}
