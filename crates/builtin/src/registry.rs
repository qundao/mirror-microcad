// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in registry.

use microcad_lang_base::{BuiltinId, HashMap};

use crate::Builtin;

#[derive(Default)]
pub struct BuiltinRegistry {
    builtins: HashMap<BuiltinId, &'static Builtin>,
}

impl std::fmt::Debug for BuiltinRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuiltinRegistry")
    }
}

impl BuiltinRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            builtins: HashMap::default(),
        };

        use crate::mu;
        let default_modules = [
            mu::core::ALL_BUILTINS,
            mu::debug::ALL_BUILTINS,
            mu::math::ALL_BUILTINS,
            mu::geo2d::ALL_BUILTINS,
            mu::ops::ALL_BUILTINS,
        ];

        registry.register_all(default_modules.into_iter().flatten().cloned());
        registry
    }

    pub fn register(&mut self, builtin: &'static Builtin) {
        let id = builtin.id();
        let previous = self.builtins.insert(id, builtin);

        // TODO Return a Result::Err here when we want to be able to load built-ins dynamically
        assert!(
            previous.is_none(),
            "Compiler bug: Duplicate builtin ID registered: {:?}",
            id
        );
    }

    /// Register an iterator of builtin static references
    pub fn register_all(&mut self, builtins: impl IntoIterator<Item = &'static Builtin>) {
        self.builtins
            .extend(builtins.into_iter().map(|b| (b.id(), b)));
    }

    pub fn get(&self, id: BuiltinId) -> Option<&'static Builtin> {
        self.builtins.get(&id).copied()
    }
}
