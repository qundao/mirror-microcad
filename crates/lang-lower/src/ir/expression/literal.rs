// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad literal syntax elements

use derive_more::Display;
use microcad_lang_base::{Refer, SrcRef, SrcReferrer};
use microcad_lang_types::{Type, Value, ty};
use serde::{Deserialize, Serialize};

/// A constant value lowered from a ast::Literal.
#[derive(Clone, Display, Debug, Hash, PartialEq, Serialize, Deserialize)]
#[display("{}", self.value())]
pub struct ConstantValue(pub Refer<Value>);

impl ConstantValue {
    pub fn new(value: impl Into<Value>, src_ref: SrcRef) -> Self {
        Self(Refer::new(value.into(), src_ref))
    }

    /// Return value of literal.
    pub fn value(&self) -> &Value {
        &self.0.value
    }

    pub fn from_value(v: impl Into<Value>) -> Self {
        Self(Refer::none(v.into()))
    }
}

impl From<Value> for ConstantValue {
    fn from(v: Value) -> Self {
        Self::from_value(v)
    }
}

impl SrcReferrer for ConstantValue {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref
    }
}

impl ty::Ty for ConstantValue {
    fn ty(&self) -> Type {
        self.value().ty()
    }
}
