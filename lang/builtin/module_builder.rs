// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builder pattern to build built-in modules.

use crate::{builtin::BuiltinWorkbenchDefinition, resolve::*, syntax::*};

/// Builder pattern to build built-in modules.
pub struct ModuleBuilder {
    // Symbol to build.
    module: Symbol,
}

impl ModuleBuilder {
    /// Create new module symbol with a name.
    pub fn new(id: impl Into<Identifier>) -> Self {
        Self {
            module: Symbol::new(
                SymbolDefinition::Module(ModuleDefinition::new(Visibility::Public, id.into())),
                None,
            ),
        }
    }

    /// Add a symbol to the module.
    pub fn symbol(self, symbol: Symbol) -> Self {
        Symbol::add_child(&self.module, symbol);
        self
    }

    /// Add the symbol from a built-in workbench definition.
    pub fn builtin<T: BuiltinWorkbenchDefinition>(self) -> Self {
        self.symbol(T::symbol())
    }

    /// Return our module symbol.
    pub fn build(self) -> Symbol {
        self.module
    }
}
