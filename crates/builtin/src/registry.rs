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
        [&mu::CORE, &mu::DEBUG, &mu::MATH, &mu::GEO2D, &mu::OPS]
            .iter()
            .for_each(|module| registry.register(module));

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

        match builtin {
            Builtin::Module(builtin_module) => {
                builtin_module
                    .items
                    .iter()
                    .for_each(|item| self.register(item));
            }
            _ => {}
        }
    }

    pub fn get(&self, id: BuiltinId) -> Option<&'static Builtin> {
        if self.builtins.get(&id).is_none() {
            eprintln!("--- {id}");
            self.builtins
                .iter()
                .for_each(|(k, v)| eprintln!("{k} {v:#?}"));
        }

        self.builtins.get(&id).copied()
    }
}
