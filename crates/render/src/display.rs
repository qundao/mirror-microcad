// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Implementations of Display traits for render entities.

use microcad_hash::ToHash;
use microcad_lang_types::ModelType;

use crate::{GeometryNodeData, GeometryOutput, GeometryOutputInner, GeometryTree};

impl std::fmt::Display for GeometryOutputInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.geometry)
    }
}

impl std::fmt::Display for GeometryOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for GeometryNodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{geo}[{hash}] {resolution} => {output_type}",
            output_type = match self.output_type {
                ModelType::Geometry2D => "2D",
                ModelType::Geometry3D => "3D",
                ModelType::Any => "Any",
                ModelType::NotDetermined => "?",
            },
            hash = self.to_hash(),
            geo = match &self.outputs.len() {
                0 => String::from("()"),
                1 => format!("{}", self.outputs.first().unwrap()),
                n => format!("[{n} Outputs]"),
            },
            resolution = match &self.resolution {
                Some(resolution) => resolution.to_string(),
                None => "".to_string(),
            },
        )?;
        Ok(())
    }
}

impl std::fmt::Display for GeometryTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root())
    }
}
