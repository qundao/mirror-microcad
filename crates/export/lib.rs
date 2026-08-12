// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export models to files

use microcad_lang_types::{ModelOutputType, ModelRef, Value};
use thiserror::Error;

pub mod ply;
pub mod stl;
pub mod svg;
pub mod wkt;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("{0}")]
    Custom(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Format error: {0}")]
    FmtError(#[from] std::fmt::Error),

    #[error("Render error: {0}")]
    RenderError(#[from] microcad_render::RenderError),
}

#[derive(Debug)]
pub struct ExporterParameters {
    pub path: std::path::PathBuf,
}

pub trait Exporter<'tree> {
    fn export(
        &self,
        model: &ModelRef<'tree>,
        parameters: &ExporterParameters,
    ) -> Result<Value, ExportError>;

    fn output_type(&self) -> ModelOutputType;
}
