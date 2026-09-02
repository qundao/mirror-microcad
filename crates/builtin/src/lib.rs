// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in crate.

mod error;
mod item;
pub use error::BuiltinError;

pub use item::{
    BuiltinConstant, BuiltinFunction, BuiltinItem, BuiltinModule, BuiltinOperation,
    BuiltinPrimitive,
};

pub mod mu;
mod registry;
use microcad_lang_base::{DisplayWithCtx, PushDiag};
pub use registry::BuiltinRegistry;

use derive_more::Debug;
use microcad_lang_types::{Arguments, Model, Value, model};

pub use microcad_lang_base::{BuiltinId, BuiltinInfo, builtin_info};
pub use microcad_macros::__mu;

#[derive(Debug, Default)]
pub struct BuiltinEvalContext<'a> {
    /// Current function being evaluated.
    pub current_fn: Option<&'a BuiltinFunction>,

    /// Diagnostics.
    pub diags: Vec<BuiltinError>,
}

impl<'a> PushDiag<BuiltinError> for BuiltinEvalContext<'a> {
    fn push_diag(&mut self, err: impl Into<BuiltinError>) {
        self.diags.push(err.into());
    }
}

impl<'a> BuiltinEvalContext<'a> {
    pub fn diag(&mut self, err: impl Into<BuiltinError>) {
        self.diags.push(err.into())
    }
}

/// Built-in execution function signature
pub type BuiltinEvalFn<T = Value> =
    fn(Arguments, &mut BuiltinEvalContext) -> Result<T, BuiltinError>;

/// A type of a function returning a T as builtin.
pub type BuiltinFn<T> = fn() -> T;

pub type BuiltinResult<T = Value> = Result<T, BuiltinError>;

pub trait BuiltinConstruct: Sized {
    const ITEM: &'static BuiltinItem;

    /// Returns the primitive element descriptor.
    fn element() -> model::Element {
        model::BuiltinWorkpiece::Primitive(Self::ITEM.id()).into()
    }

    fn check_element(model: &Model) -> Result<(), BuiltinError> {
        if model.element == Self::element() {
            Ok(())
        } else {
            let default_ctx = microcad_lang_base::DefaultContext;
            Err(BuiltinError::ElementMismatch {
                expected: Self::element().to_string_with_ctx(&default_ctx),
                actual: model.element.clone().to_string_with_ctx(&default_ctx),
            })
        }
    }

    /// Constructs the Rust struct representation from a evaluated `Model`.
    fn from_model(model: &Model) -> Result<Self, BuiltinError>;
}

pub trait BuiltinPrimitiveCall: BuiltinConstruct {
    /// Call this builtin with builtin with arguments and evaluate it into a model.
    fn call(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Model, BuiltinError> {
        Ok(Model::new(Self::element()).with_properties(args))
    }
}

#[macro_export]
macro_rules! construct_from_model {
    ($model:expr, $struct:ident { $( $field:ident ),* $(,)? }) => {{
        Self::check_element($model)?;
        Ok($struct {
            $(
                $field: $model.get_property_as(stringify!($field))?,
            )*
        })
    }};
}
