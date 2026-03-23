// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument value evaluation entity

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

use crate::{syntax::*, ty::*, value::*};

/// Argument value.
#[derive(Clone, Debug, SrcReferrer)]
pub struct ArgumentValue {
    /// *value* of the argument.
    pub value: Value,
    /// If expression of value is a single identifier, this item catches it.
    pub inline_id: Option<Identifier>,
    /// Source code reference.
    src_ref: SrcRef,
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
    pub fn new(value: Value, inline_id: Option<Identifier>, src_ref: SrcRef) -> Self {
        Self {
            value,
            inline_id,
            src_ref,
        }
    }

    /// If argument is an array returns the inner type
    pub fn ty_inner(&self) -> Type {
        if let Type::Array(ty) = self.ty() {
            ty.as_ref().clone()
        } else {
            Type::Invalid
        }
    }
}
