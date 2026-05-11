// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad literal syntax elements

mod number_literal;
mod units;

use microcad_lang_base::{Refer, SrcRef, SrcReferrer};
pub use number_literal::*;
pub use units::*;

use crate::{ty::*, value::Value};

/// Literal of any kind.
#[derive(Clone, Debug, PartialEq)]
pub struct Literal(pub Refer<Value>);

impl Literal {
    /// Return value of literal.
    pub fn value(&self) -> &Value {
        &self.0.value
    }
}

impl SrcReferrer for Literal {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref.clone()
    }
}

impl crate::ty::Ty for Literal {
    fn ty(&self) -> Type {
        self.value().ty()
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
