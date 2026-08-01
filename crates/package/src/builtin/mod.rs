// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin module

// Re-export symbols
pub use crate::parameter;
use crate::rst;
use derive_more::Display;
pub use microcad_lang_base::Identifier;
use microcad_lang_types::{
    Angle, Color, Integer, Length, Quantity, QuantityType, Scalar, Tuple, Type, Value,
    ty::TupleType,
};
use microcad_model::Model;
use miette::Diagnostic;
use thiserror::Error;

/// This enum is used to declare parameter list for builtin symbols conveniently.
///
/// It is used in the `parameter!` and `argument!` macros to be able
/// to declare parameters and arguments in µcad like way, for example: `a: Scalar = 4.0`.
pub enum BuiltinTypeHelper {
    /// Integer type.
    Integer,
    /// A unitless scalar value.
    Scalar,
    /// Length in mm.
    Length,
    /// Area in mm².
    Area,
    /// Volume in mm³.
    Volume,
    /// Density in g/mm³
    Density,
    /// An angle in radians.
    Angle,
    /// Weight of a specific volume of material.
    Weight,
    /// String type.
    String,
    /// Bool type.
    Bool,
    /// Color type.
    Color,
}

impl From<BuiltinTypeHelper> for Type {
    fn from(value: BuiltinTypeHelper) -> Self {
        match value {
            BuiltinTypeHelper::Integer => Type::Integer,
            BuiltinTypeHelper::Scalar => Type::Quantity(QuantityType::Scalar),
            BuiltinTypeHelper::Length => Type::Quantity(QuantityType::Length),
            BuiltinTypeHelper::Area => Type::Quantity(QuantityType::Area),
            BuiltinTypeHelper::Volume => Type::Quantity(QuantityType::Volume),
            BuiltinTypeHelper::Density => Type::Quantity(QuantityType::Density),
            BuiltinTypeHelper::Angle => Type::Quantity(QuantityType::Angle),
            BuiltinTypeHelper::Weight => Type::Quantity(QuantityType::Weight),
            BuiltinTypeHelper::String => Type::String,
            BuiltinTypeHelper::Bool => Type::Bool,
            BuiltinTypeHelper::Color => Type::Tuple(TupleType::new_color().into()),
        }
    }
}

/// This enum is used to declare parameter list for builtin symbols conveniently.
///
/// It is used in the `parameter!` and `argument!` macros to be able
/// to declare parameters and arguments in µcad like way, for example: `a: Scalar = 4.0`.
pub enum BuiltinValueHelper {
    /// Integer type.
    Integer(Integer),
    /// Scalar type.
    Scalar(Scalar),
    /// Length type.
    Length(Length),
    /// Angle type
    Angle(Angle),
    /// String type.
    String(String),
    /// Bool type
    Bool(bool),
    /// Color type
    Color(Color),
}

impl From<BuiltinValueHelper> for Value {
    fn from(value: BuiltinValueHelper) -> Self {
        match value {
            BuiltinValueHelper::Scalar(v) => {
                Value::Quantity(Quantity::new(v, QuantityType::Scalar))
            }
            BuiltinValueHelper::Integer(i) => Value::Integer(i),
            BuiltinValueHelper::Length(v) => {
                Value::Quantity(Quantity::new(*v, QuantityType::Length))
            }
            BuiltinValueHelper::Angle(v) => {
                Value::Quantity(Quantity::new(v.0, QuantityType::Angle))
            }
            BuiltinValueHelper::String(s) => Value::String(s),
            BuiltinValueHelper::Bool(b) => Value::Bool(b),
            BuiltinValueHelper::Color(c) => c.into(),
        }
    }
}

pub trait BuiltinEvalContext {}

#[derive(Debug, Error, Diagnostic)]
pub enum BuiltinError {}

/// Builtin function type
pub type BuiltinFunctionFn =
    dyn Fn(&rst::ParameterList, &Tuple, &mut dyn BuiltinEvalContext) -> Result<Value, BuiltinError>;

/// Builtin function struct
#[derive(Clone)]
pub struct BuiltinFunction {
    /// Documentation of this function.
    pub doc: Option<rst::DocBlock>,

    /// Optional parameter value list to check the builtin signature.
    pub parameters: rst::ParameterList,

    /// Functor to evaluate this function
    pub f: &'static BuiltinFunctionFn,
}

/// Builtin function type
pub type BuiltinWorkbenchFn =
    dyn Fn(&rst::ParameterList, &Tuple, &mut dyn BuiltinEvalContext) -> Result<Model, BuiltinError>;

/// Builtin workbench
#[derive(Clone)]
pub struct BuiltinWorkbench {
    /// Documentation of this workbench.
    pub doc: Option<rst::DocBlock>,

    /// Optional parameter value list to check the builtin signature.
    pub parameters: rst::ParameterList,

    /// Functor to evaluate this function
    pub f: &'static BuiltinWorkbenchFn,

    /// Builtin workbench kind.
    pub kind: BuiltinWorkbenchKind,
}

/// The kind of the built-in workbench determines its output.
#[derive(Debug, Clone, Display, PartialEq)]
pub enum BuiltinWorkbenchKind {
    /// A parametric 2D primitive.
    Primitive2D,
    /// A parametric 3D primitive.
    Primitive3D,
    /// An affine transformation.
    Transform,
    /// An operation on a model.
    Operation,
}

/// A builtin constant.
#[derive(Debug, Clone)]
pub struct BuiltinConstant {
    /// Documentation.
    pub doc: Option<rst::DocBlock>,
    /// The actual value.
    pub value: Value,
}

/// Builtin enum
#[derive(Clone, derive_more::From)]
pub enum Builtin {
    /// Builtin function.
    Function(BuiltinFunction),
    /// Builtin workbench.
    Workbench(BuiltinWorkbench),
    /// Builtin constant
    Constant(BuiltinConstant),
}
