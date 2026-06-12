// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad literal syntax elements

use microcad_lang_base::{Refer, SrcRef, SrcReferrer};
use microcad_lang_types::{Type, Value, ty};
use serde::Serialize;

/// Literal of any kind.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Literal(pub Refer<Value>);

impl Literal {
    /// Return value of literal.
    pub fn value(&self) -> &Value {
        &self.0.value
    }
}

impl SrcReferrer for Literal {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref
    }
}

impl ty::Ty for Literal {
    fn ty(&self) -> Type {
        self.value().ty()
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
