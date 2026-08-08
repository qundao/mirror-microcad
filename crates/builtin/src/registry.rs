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

        registry.register_all(crate::mu::core::ALL_BUILTINS);
        registry
    }

    pub fn register(&mut self, builtin: &'static Builtin) {
        self.builtins.insert(builtin.id(), builtin);
    }

    /// Register an iterator of builtin static references
    pub fn register_all<'a>(&mut self, builtins: impl IntoIterator<Item = &'a &'static Builtin>) {
        builtins
            .into_iter()
            .for_each(|builtin| self.register(builtin))
    }

    pub fn get(&self, id: BuiltinId) -> Option<&'static Builtin> {
        self.builtins.get(&id).copied()
    }
}
