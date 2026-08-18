// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`Model`].

use derive_more::{Display, From};
use microcad_lang_base::{BuiltinId, BuiltinInfo, element::WorkbenchKind};
use serde::{Deserialize, Serialize};

use crate::{
    Value,
    model::{AffineTransform, BooleanOp, ModelOutputType},
};

/// The kind of the built-in workbench determines its output.
#[non_exhaustive]
#[derive(Debug, Display, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub enum BuiltinWorkpiece {
    /// A parametric 2D primitive.
    Primitive2D(BuiltinId),
    /// A parametric 3D primitive.
    Primitive3D,
    /// An affine transformation.
    AffineTransform(AffineTransform),
    /// Boolean operation
    BooleanOp(BooleanOp),
    /// Extrude
    Operation(BuiltinId),
}

/// Trait to implement a Primitive2D
#[typetag::serde(tag = "primitive2d")]
pub trait Primitive2D {
    /// Get the builtin name for this primitive.
    fn builtin_info(&self) -> &'static BuiltinInfo;

    /// Get a property of this model
    fn get_property(&self, s: &str) -> Value;
}

impl BuiltinWorkpiece {
    fn output_type(&self) -> ModelOutputType {
        match self {
            BuiltinWorkpiece::Primitive2D(_) => ModelOutputType::Geometry2D,
            BuiltinWorkpiece::Primitive3D => ModelOutputType::Geometry3D,
            BuiltinWorkpiece::AffineTransform(_) | BuiltinWorkpiece::BooleanOp(_) => {
                ModelOutputType::NotDetermined
            }
            _ => todo!(),
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
    BuiltinWorkpiece(BuiltinWorkpiece),

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
            BuiltinWorkpiece(builtin_workpiece) => builtin_workpiece.output_type(),
            Group | Multiplicity | InputPlaceholder | Value(_) => ModelOutputType::NotDetermined,
        }
    }
}
