// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Qualifier of an assignment

/// Qualifier of an assignment
///
/// This is used to determine if an entity is public or private.
/// By default, entities are private.
#[derive(Copy, Clone, Default, PartialEq)]
pub enum Qualifier {
    /// Local variable.
    #[default]
    Value,
    /// Symbol.
    Const,
    /// Workbench property.
    Prop,
}

impl std::fmt::Display for Qualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Qualifier::Value => Ok(()),
            Qualifier::Const => write!(f, "const "),
            Qualifier::Prop => write!(f, "prop "),
        }
    }
}
