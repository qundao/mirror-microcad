// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of symbols.

mod argument_match;

mod context;
mod eval_error;
mod function;
mod workbench;

pub use argument_match::*;
pub use eval_error::*;

pub use context::EvalContext;

use microcad_lang_types::ArgumentValueList;
pub use microcad_package::{builtin, rst};

/// Evaluation trait.
///
/// The return type `T` defines to which output type the type is evaluated.
pub trait Eval<T> {
    /// Evaluate a syntax element into a type `T`.
    fn eval(&self, context: &mut EvalContext) -> EvalResult<T>;
}

/// Trait for calls with argument list.
pub trait CallTrait<T> {
    /// Evaluate call into value (if possible).
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<T>;
}

/// Shortcut to create an argument value
#[macro_export]
macro_rules! argument_value {
    ($id:ident: $ty:ident = $value:expr) => {
        microcad_lang_types::ArgumentValue::new(
            $crate::builtin::BuiltinValueHelper::$ty($value).into(),
            Some($crate::rst::Identifier::no_ref(stringify!($id))),
        )
    };
    ($ty:ident = $value:expr) => {
        microcad_lang_types::ArgumentValue::new(
            $crate::builtin::BuiltinValueHelper::$ty($value).into(),
            Some($crate::rst::Identifier::no_ref(stringify!($id))),
        )
    };
    () => {};
}

/// Create a Tuple from items
#[macro_export]
macro_rules! arguments {
        ($($key:ident = $value:expr),*) => {
                $crate::Arguments::from(microcad_lang_types::tuple!($( (stringify!($key), $crate::value::Value::from($value)) ),*))
    };
}
