// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`Model`].

use derive_more::{Display, From};
use microcad_lang_base::{BuiltinId, SrcRef, element::WorkbenchKind};
use serde::{Deserialize, Serialize};

use crate::{
    Ty, Type, Value,
    model::{AffineTransform, BooleanOp, ModelOutputType},
};

#[derive(Debug, Display, Clone, Hash, PartialEq, Serialize, Deserialize)]
#[display("{kind}")]
pub struct Workpiece {
    pub kind: WorkbenchKind,
    pub src_ref: SrcRef,
}

impl Workpiece {
    pub fn new(kind: WorkbenchKind) -> Self {
        Self {
            kind,
            src_ref: SrcRef::none(),
        }
    }
}

impl Ty for WorkbenchKind {
    fn ty(&self) -> crate::Type {
        match self {
            WorkbenchKind::Sketch => Type::Model(ModelOutputType::Geometry2D),
            WorkbenchKind::Part => Type::Model(ModelOutputType::Geometry3D),
            WorkbenchKind::Op => Type::Model(ModelOutputType::Any),
        }
    }
}

/// The kind of the built-in workbench determines its output.
#[non_exhaustive]
#[derive(Debug, Display, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub enum BuiltinWorkpiece {
    /// A parametric 2D primitive.
    Primitive(BuiltinId),

    /// An affine transformation.
    AffineTransform(AffineTransform),
    /// Boolean operation
    BooleanOp(BooleanOp),
    /// Extrude
    Operation(BuiltinId),
}

impl BuiltinWorkpiece {
    fn output_type(&self) -> ModelOutputType {
        match self {
            BuiltinWorkpiece::Primitive(_) => ModelOutputType::Geometry2D,
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
    Workpiece(Workpiece),

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
            Workpiece(workpiece) => workpiece.kind.into(),
            BuiltinWorkpiece(builtin_workpiece) => builtin_workpiece.output_type(),
            Group | Multiplicity | InputPlaceholder | Value(_) => ModelOutputType::NotDetermined,
        }
    }
}
