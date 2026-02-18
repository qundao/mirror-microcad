// Copyright © 2024-2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Visibility of an entity.

use crate::syntax::*;

/// Visibility of an entity.
///
/// This is used to determine if an entity is public or private.
/// By default, entities are private.
#[derive(Clone, Default, PartialEq)]
pub enum Visibility {
    /// Private visibility
    #[default]
    Private,
    /// Private visibility within a given use all reference.
    PrivateUse(QualifiedName),
    /// Public visibility
    Public,
    /// Mark symbol for deletion {used internally while resolving)
    Deleted,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Private => Ok(()),
            Visibility::PrivateUse(_) => Ok(()),
            Visibility::Public => write!(f, "pub "),
            Visibility::Deleted => write!(f, "(deleted) "),
        }
    }
}

impl std::fmt::Debug for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Private => Ok(()),
            Visibility::PrivateUse(name) => write!(f, "«{name}» "),
            Visibility::Public => write!(f, "pub "),
            Visibility::Deleted => write!(f, "❌ "),
        }
    }
}
