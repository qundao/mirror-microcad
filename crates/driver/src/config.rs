// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad driver config.

use microcad_builtin::{Exporter, ExporterRegistry};
use microcad_core::RenderResolution;
use microcad_lang::model::{ExportCommand, Model, OutputType};
use miette::IntoDiagnostic;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub no_std: bool,

    pub search_paths: Vec<std::path::PathBuf>,

    /// Default extension (default: `µcad`).
    pub default_extension: String,

    /// Export settings.
    pub export: ExportConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            no_std: false,
            default_extension: "µcad".to_string(),
            export: Default::default(),
            search_paths: vec![microcad_std::global_library_search_path()],
        }
    }
}

impl Config {
    /// Load config from TOML file.
    pub fn load(filename: &std::path::Path) -> miette::Result<Self> {
        let content = std::fs::read_to_string(filename).into_diagnostic()?;
        let mut config: Config = toml::from_str(&content).into_diagnostic()?;

        if !microcad_lang_base::MICROCAD_EXTENSIONS.contains(&config.default_extension.as_str()) {
            let fallback = Config::default().default_extension;
            log::warn!(
                "`{}` is a valid µcad extension, switching to `{fallback}`.",
                &config.default_extension
            );
            config.default_extension = fallback;
        }

        Ok(config)
    }
}

/// Export settings.
#[derive(Deserialize, Clone)]
pub struct ExportConfig {
    /// Default sketch exporter.
    pub sketch: String,
    /// Default part exporter.
    pub part: String,
    /// Default render resolution when exported.
    pub resolution: Option<String>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            sketch: "svg".into(),
            part: "stl".into(),
            resolution: None,
        }
    }
}

impl ExportConfig {
    pub fn render_resolution(&self) -> RenderResolution {
        use microcad_lang::*;

        use std::str::FromStr;
        let resolution_str = self.resolution.clone().unwrap_or("0.1mm".into());
        let value = syntax::NumberLiteral::from_str(&resolution_str)
            .map(|literal| literal.value())
            .unwrap_or(value::Value::None);

        match value {
            value::Value::Quantity(value::Quantity {
                value,
                quantity_type: ty::QuantityType::Length,
            }) => RenderResolution::new(value),
            _ => {
                let default = RenderResolution::default();
                log::warn!(
                    "Invalid resolution `{resolution_str}`. Using default resolution: {value}mm",
                    value = default.linear
                );
                default
            }
        }
    }
}
