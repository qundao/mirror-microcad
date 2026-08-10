// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model output type.

use microcad_lang_base::element::WorkbenchKind;
use serde::{Deserialize, Serialize};

/// The output type of the [`crateModel`].
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum ModelOutputType {
    /// The output type has not yet been determined.
    #[default]
    NotDetermined,
    /// The [`Model`] outputs a 2d geometry.
    Geometry2D,
    /// The [`Model`] outputs a 3d geometry.
    Geometry3D,
    /// The [`Model`] contains both 2D and 3D geometry.
    Any,
}

impl ModelOutputType {
    /// Merge this output type with another.
    pub fn merge(&self, other: &Self) -> ModelOutputType {
        match (self, other) {
            (ModelOutputType::NotDetermined, output_type) => *output_type,
            (ModelOutputType::Geometry2D, ModelOutputType::NotDetermined)
            | (ModelOutputType::Geometry2D, ModelOutputType::Geometry2D)
            | (ModelOutputType::Geometry3D, ModelOutputType::NotDetermined)
            | (ModelOutputType::Geometry3D, ModelOutputType::Geometry3D) => *self,
            (ModelOutputType::Geometry2D, ModelOutputType::Geometry3D)
            | (ModelOutputType::Geometry3D, ModelOutputType::Geometry2D)
            | (ModelOutputType::Geometry2D, ModelOutputType::Any)
            | (ModelOutputType::Geometry3D, ModelOutputType::Any)
            | (ModelOutputType::Any, _) => ModelOutputType::Any,
        }
    }
}

impl std::fmt::Display for ModelOutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::NotDetermined => "Undetermined",
                Self::Geometry2D => "2D",
                Self::Geometry3D => "3D",
                Self::Any => "2D/3D",
            }
        )
    }
}

impl From<WorkbenchKind> for ModelOutputType {
    fn from(kind: WorkbenchKind) -> Self {
        match kind {
            WorkbenchKind::Sketch => Self::Geometry2D,
            WorkbenchKind::Part => Self::Geometry3D,
            WorkbenchKind::Op => Self::NotDetermined,
        }
    }
}
