// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value evaluation entity

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

use crate::{ty::*, value::*};

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

impl std::fmt::Display for ParameterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(def) = &self.default_value {
            write!(f, "{} = {def}", def.ty())?;
        } else if let Some(ty) = &self.specified_type {
            write!(f, "= {ty}")?;
        }
        Ok(())
    }
}

#[test]
fn test_is_list_of() {
    assert!(
        Type::Array(Box::new(QuantityType::Scalar.into()))
            .is_array_of(&QuantityType::Scalar.into())
    );
}
