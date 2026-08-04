// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad package API

pub mod builtin;
pub mod manifest;
pub mod rst;
pub mod tree;

/// Shortcut to create a `Parameter`
#[macro_export]
macro_rules! parameter {
    ($id:ident) => {
        $crate::rst::Parameter {
            id: $crate::rst::Identifier::no_ref(stringify!($id)),
            src_ref: microcad_lang_base::SrcRef::none(),
            ty: None,
            default_value: None,
            doc: None,
        }
    };
    ($id:ident: $ty:ident) => {
        $crate::rst::Parameter {
            id: $crate::rst::Identifier::no_ref(stringify!($id)),
            ty: Some($crate::builtin::BuiltinTypeHelper::$ty.into()),
            src_ref: microcad_lang_base::SrcRef::none(),
            default_value: None,
            doc: None,
        }
    };
    ($id:ident: $ty:ident = $value:expr) => {
        $crate::rst::Parameter {
            id: $crate::rst::Identifier::no_ref(stringify!($id)),
            doc: None,
            ty: Some($crate::builtin::BuiltinTypeHelper::$ty.into()),
            default_value: Some($crate::builtin::BuiltinValueHelper::$ty($value).into()),
            src_ref: microcad_lang_base::SrcRef::none(),
        }
    };
    ($id:ident = $value:expr) => {
        $crate::rst::Parameter {
            id: $crate::rst::Identifier::no_ref(stringify!($id)),
            doc: None,
            ty: None,
            default_value: Some($value),
            src_ref: microcad_lang_base::SrcRef::none(),
        }
    };
}
