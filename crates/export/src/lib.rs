// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export models to files

use microcad_lang_types::{ModelNodeRef, ModelOutputType, Value};
use thiserror::Error;

pub mod ply;
pub mod stl;
pub mod svg;
pub mod wkt;

/// An export error
#[derive(Debug, Error)]
pub enum ExportError {
    /// Custom error.
    #[error("{0}")]
    Custom(String),

    /// I/O Error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Format error
    #[error("Format error: {0}")]
    FmtError(#[from] std::fmt::Error),

    /// Error when rendering the model.
    #[error("Render error: {0}")]
    RenderError(#[from] microcad_render::RenderError),
}

/// Export parameters
#[non_exhaustive]
#[derive(Debug)]
pub struct ExporterParameters {
    /// Path into which the file is exported.
    pub path: std::path::PathBuf,
    /// The render resolution.
    pub resolution: microcad_render::RenderResolution,
}

/// The exporter trait.
pub trait Exporter {
    /// The export function.
    fn export<'tree>(
        &self,
        model: &ModelNodeRef<'tree>,
        parameters: &ExporterParameters,
    ) -> Result<Value, ExportError>;

    /// The model type this export is supposed to access.
    fn model_type(&self) -> ModelOutputType;
}
