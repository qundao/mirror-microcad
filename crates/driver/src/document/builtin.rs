// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::Symbol;
use microcad_docgen::DocGen;

use crate::document;

#[derive(Default)]
pub enum State {
    #[default]
    Raw,
    Symbol(Symbol),
}

impl document::BuiltinItem {
    pub fn symbol(&'_ self) -> document::DiagResult<'_> {
        self.transition(|_| Ok(State::Symbol(microcad_builtin::__builtin())))
    }

    /// Generate documentation
    pub fn doc_gen(&'_ self, path: std::path::PathBuf) -> miette::Result<()> {
        self.symbol().expect("No error");

        let state = &*self.state.borrow();

        let symbol = match state {
            State::Raw => todo!(),
            State::Symbol(symbol) => symbol,
        };
        let generator = microcad_docgen::MdBook::new(path);

        generator
            .doc_gen(symbol)
            .map_err(|err| miette::miette!("{err}"))
    }
}
