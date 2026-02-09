// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Work piece element

use crate::{eval::*, model::*, render::*, syntax::*, value::*};

/// A workpiece is an element produced by a workbench.
#[derive(Debug, Clone)]
pub struct Workpiece {
    /// Workpiece kind: `op`, `sketch`, `part`.
    pub kind: WorkbenchKind,
    /// Workpiece properties.
    pub properties: Properties,
    /// Creator symbol.
    pub creator: Hashed<Creator>,
}

impl Workpiece {
    /// Check the output type of the workpiece.
    pub fn check_output_type(&self, output_type: OutputType) -> EvalResult<()> {
        match (self.kind, output_type) {
            (WorkbenchKind::Part, OutputType::Geometry3D)
            | (WorkbenchKind::Sketch, OutputType::Geometry2D)
            | (WorkbenchKind::Operation, OutputType::Geometry3D | OutputType::Geometry2D) => Ok(()),

            (_, OutputType::NotDetermined) => Err(EvalError::WorkbenchNoOutput(self.kind)),
            _ => Err(EvalError::WorkbenchInvalidOutput(self.kind, output_type)),
        }
    }
}

impl std::fmt::Display for Workpiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            WorkbenchKind::Part => write!(f, "Workpiece(part) {}", *self.creator),
            WorkbenchKind::Sketch => write!(f, "Workpiece(sketch) {}", *self.creator),
            WorkbenchKind::Operation => write!(f, "Workpiece(op) {}", *self.creator),
        }
    }
}

impl ComputedHash for Workpiece {
    fn computed_hash(&self) -> crate::render::HashId {
        self.creator.computed_hash()
    }
}

impl PropertiesAccess for Workpiece {
    fn get_property(&self, id: &Identifier) -> Option<&Value> {
        self.properties.get(id)
    }

    fn set_property(&mut self, id: Identifier, value: Value) -> Option<Value> {
        self.properties.insert(id, value)
    }
    fn get_properties(&self) -> Option<&Properties> {
        Some(&self.properties)
    }

    fn add_properties(&mut self, props: Properties) {
        self.properties
            .extend(props.iter().map(|(id, prop)| (id.clone(), prop.clone())));
    }
}
