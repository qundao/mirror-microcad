// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builder pattern to build built-in modules.

use crate::{builtin::BuiltinWorkbenchDefinition, resolve::*, syntax::*, value::Value};

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
                SymbolDef::Module(ModuleDefinition::new(Visibility::Public, id.into())),
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

    /// Add a public constant.
    pub fn pub_const(self, name: &str, value: impl Into<Value>) -> Self {
        self.symbol(Symbol::new_with_visibility(
            Visibility::Public,
            SymbolDef::Constant(name.into(), value.into()),
            None,
        ))
    }

    /// Return our module symbol.
    pub fn build(self) -> Symbol {
        self.module
    }
}
