// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value evaluation entity

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;
use microcad_lang_types::{Ty, Type, Value};

/// Parameter value is the result of evaluating a parameter
#[derive(Clone, Debug, Default, SrcReferrer)]
pub struct ParameterValue {
    /// Parameter type
    pub specified_type: Option<Type>,
    /// Parameter default
    pub default_value: Option<Value>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl ParameterValue {
    /// Creates an invalid parameter value, in case an error occurred during evaluation
    pub fn invalid(src_ref: SrcRef) -> Self {
        Self {
            specified_type: None,
            default_value: None,
            src_ref,
        }
    }
}

impl Ty for ParameterValue {
    /// Return effective type
    ///
    /// Returns any `specified_type` or the type of the `default_value`.
    /// Panics if neither of both is available.
    fn ty(&self) -> Type {
        if let Some(ty) = &self.specified_type {
            ty.clone()
        } else if let Some(def) = &self.default_value {
            def.ty()
        } else {
            Type::Invalid
        }
    }
}
