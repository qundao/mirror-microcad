// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model output type.

use microcad_lang_base::element::WorkbenchKind;
use serde::{Deserialize, Serialize};

/// The output type of the [`crateModel`].
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum OutputType {
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

impl OutputType {
    /// Merge this output type with another.
    pub fn merge(&self, other: &Self) -> OutputType {
        match (self, other) {
            (OutputType::NotDetermined, output_type) => *output_type,
            (OutputType::Geometry2D, OutputType::NotDetermined)
            | (OutputType::Geometry2D, OutputType::Geometry2D)
            | (OutputType::Geometry3D, OutputType::NotDetermined)
            | (OutputType::Geometry3D, OutputType::Geometry3D) => *self,
            (OutputType::Geometry2D, OutputType::Geometry3D)
            | (OutputType::Geometry3D, OutputType::Geometry2D)
            | (OutputType::Geometry2D, OutputType::Any)
            | (OutputType::Geometry3D, OutputType::Any)
            | (OutputType::Any, _) => OutputType::Any,
        }
    }
}

impl std::fmt::Display for OutputType {
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

impl From<WorkbenchKind> for OutputType {
    fn from(kind: WorkbenchKind) -> Self {
        match kind {
            WorkbenchKind::Sketch => Self::Geometry2D,
            WorkbenchKind::Part => Self::Geometry3D,
            WorkbenchKind::Op => Self::NotDetermined,
        }
    }
}
