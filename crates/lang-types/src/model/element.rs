// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Element of a [`Model`].

use derive_more::{Display, From};
use microcad_lang_base::{SrcRef, SrcReferrer, element::WorkbenchKind};
use serde::{Deserialize, Serialize};

use crate::{Value, model::Creator, model::OutputType};

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
}

impl From<BuiltinWorkbenchKind> for OutputType {
    fn from(kind: BuiltinWorkbenchKind) -> Self {
        match kind {
            BuiltinWorkbenchKind::Primitive2D => Self::Geometry2D,
            BuiltinWorkbenchKind::Primitive3D => Self::Geometry3D,
            BuiltinWorkbenchKind::Operation | BuiltinWorkbenchKind::Transform => {
                Self::NotDetermined
            }
        }
    }
}

/// An element defines the entity of a [`Model`].
#[derive(Clone, Debug, Hash, PartialEq, Default, From, Serialize, Deserialize)]
pub enum ElementKind {
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

impl ElementKind {
    fn output_type(&self) -> OutputType {
        use ElementKind::*;
        match &self {
            Workpiece(workpiece) => (*workpiece).into(),
            BuiltinWorkpiece(builtin_workpiece) => (*builtin_workpiece).into(),
            Group | Multiplicity | InputPlaceholder | Value(_) => OutputType::NotDetermined,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Element {
    kind: ElementKind,
    src_ref: SrcRef,
    creator: Option<Creator>,
}

impl SrcReferrer for Element {
    fn src_ref(&self) -> SrcRef {
        self.src_ref
    }
}

impl Element {
    pub fn kind(&self) -> &ElementKind {
        &self.kind
    }

    /// Creator.
    pub fn creator(&self) -> &Option<Creator> {
        &self.creator
    }

    /// Get output type of element.
    pub fn output_type(&self) -> OutputType {
        self.kind.output_type()
    }
}
