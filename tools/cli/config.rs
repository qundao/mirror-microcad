// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI config.

use miette::IntoDiagnostic;
use serde::Deserialize;

/// Microcad CLI config.
#[derive(Deserialize)]
pub struct Config {
    /// Default extension (default: `µcad`).
    pub default_extension: String,

    /// Export settings.
    pub export: Export,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_extension: "µcad".to_string(),
            export: Default::default(),
        }
    }
}

impl Config {
    /// Load config from TOML file.
    pub fn load(filename: &std::path::Path) -> miette::Result<Self> {
        let content = std::fs::read_to_string(filename).into_diagnostic()?;
        let mut config: Config = toml::from_str(&content).into_diagnostic()?;

        if !microcad_lang::MICROCAD_EXTENSIONS.contains(&config.default_extension.as_str()) {
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
#[derive(Deserialize)]
pub struct Export {
    /// Default sketch exporter.
    pub sketch: String,
    /// Default part exporter.
    pub part: String,
}

impl Default for Export {
    fn default() -> Self {
        Self {
            sketch: "svg".into(),
            part: "stl".into(),
        }
    }
}
