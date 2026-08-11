// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`Model`].

use derive_more::{Display, From};
use microcad_lang_base::{BuiltinId, BuiltinName, element::WorkbenchKind};
use serde::{Deserialize, Serialize};

use crate::{
    Value,
    model::{BooleanOp, ModelOutputType},
};

/// The kind of the built-in workbench determines its output.
#[derive(Debug, Copy, Clone, Hash, Display, PartialEq, Serialize, Deserialize)]
pub enum BuiltinWorkbenchKind {
    /// A parametric 2D primitive.
    Primitive2D,
    /// A parametric 3D primitive.
    Primitive3D,
    /// An affine transformation.
    Transform,
    /// An operation on a model.
    Operation,
    /// Boolean operation
    BooleanOp(BooleanOp),
}

/// Trait to implement a Primitive2D
#[typetag::serde(tag = "primitive2d")]
pub trait Primitive2D {
    /// Get the builtin name for this primitive.
    fn builtin_name(&self) -> BuiltinName;

    /// Get a property of this model
    fn get_property(&self, s: &str) -> Value;
}

#[derive(Clone, Debug, Display, Hash, PartialEq, From, Serialize, Deserialize)]
pub enum BuiltinWorkpiece {
    Primitive2D(BuiltinId),
    //Primitive3D(Box<dyn Primitive3D>),
    //Operation(Box<dyn Operation>),
    //Transform(AffineTransform),
    BooleanOp(BooleanOp),
}

impl From<BuiltinWorkbenchKind> for ModelOutputType {
    fn from(kind: BuiltinWorkbenchKind) -> Self {
        match kind {
            BuiltinWorkbenchKind::Primitive2D => Self::Geometry2D,
            BuiltinWorkbenchKind::Primitive3D => Self::Geometry3D,
            BuiltinWorkbenchKind::Operation
            | BuiltinWorkbenchKind::Transform
            | BuiltinWorkbenchKind::BooleanOp(_) => Self::NotDetermined,
        }
    }
}

/// An element defines the entity of a [`Model`].
#[derive(Clone, Debug, Display, Hash, PartialEq, Default, From, Serialize, Deserialize)]
pub enum Element {
    #[default]
    /// A group element is created by a body `{}`.
    Group,

    /// An element containing a value.
    Value(Value),

    /// A workpiece which is created by workbenches.
    Workpiece(WorkbenchKind),

    /// A built-in workpiece which created by built-in workbenches.
    BuiltinWorkpiece(BuiltinWorkbenchKind),

    /// Multiplicity.
    Multiplicity,

    /// Created from `@input` marker. Will never be part of the final model.
    InputPlaceholder,
}

impl Element {
    pub fn output_type(&self) -> ModelOutputType {
        use Element::*;
        match &self {
            Workpiece(workpiece) => (*workpiece).into(),
            BuiltinWorkpiece(builtin_workpiece) => (*builtin_workpiece).into(),
            Group | Multiplicity | InputPlaceholder | Value(_) => ModelOutputType::NotDetermined,
        }
    }
}
