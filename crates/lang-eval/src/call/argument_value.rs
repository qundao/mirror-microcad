// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument value evaluation entity

use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_proc_macros::SrcReferrer;
use microcad_lang_types::{Ty, Type, Value};

/// Argument value.
#[derive(Clone, Debug)]
pub struct ArgumentValue {
    /// *value* of the argument.
    pub value: Value,
    /// If expression of value is a single identifier, this item catches it.
    pub id: Option<Identifier>,
}

impl std::fmt::Display for ArgumentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{val}", val = self.value,)
    }
}

impl Ty for ArgumentValue {
    fn ty(&self) -> Type {
        self.value.ty()
    }
}

impl ArgumentValue {
    /// Create new argument value
    pub fn new(value: Value, id: Option<Identifier>) -> Self {
        Self { value, id }
    }
}
