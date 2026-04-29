// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::{Exporter, ExporterRegistry};
use microcad_lang::{
    model::{Model, OutputType},
    value::Value,
};

pub use microcad_lang::model::ExportCommand;

use crate::config::ExportConfig;

/// An export from a model
pub struct Export {
    /// Root model
    pub(crate) model: Model,
    pub(crate) command: ExportCommand,
}

impl Export {
    /// Get all models that are supposed to be exported.
    ///
    /// All child models of `model` that are in the same source file and
    /// that have an `export` attribute will be exported.
    ///
    /// If no models have been found, we simply export this model with the default export attribute.
    pub fn target_models(&self) -> miette::Result<Vec<(Model, ExportCommand)>> {
        use microcad_lang::model::AttributesAccess;
        let mut models =
            self.model
                .source_file_descendants()
                .fold(Vec::new(), |mut models, model| {
                    let b = model.borrow();
                    models.append(
                        &mut b
                            .attributes
                            .get_exports()
                            .iter()
                            .map(|attr| (model.clone(), attr.clone()))
                            .collect(),
                    );
                    models
                });

        // No models with export attributes have been found.
        if models.is_empty() {
            // Add the root model with default exporters.
            models.push((self.model.clone(), self.command.clone()))
        }

        Ok(models)
    }

    pub fn export_targets(&self, models: &[(Model, ExportCommand)]) -> miette::Result<()> {
        models
            .iter()
            .try_for_each(|(model, export)| -> miette::Result<()> {
                let value = export.export(model)?;
                if !matches!(value, Value::None) {
                    log::info!("{value}");
                };
                Ok(())
            })?;
        Ok(())
    }

    pub fn list_targets(&self, models: &Vec<(Model, ExportCommand)>) -> miette::Result<()> {
        for (model, attr) in models {
            eprintln!("{model} => {attr}");
        }
        Ok(())
    }

    pub fn export(&self) -> miette::Result<Vec<(Model, ExportCommand)>> {
        let target_models = self.target_models()?;

        self.export_targets(&target_models)?;

        eprintln!("Exported {} file(s) successfully:", target_models.len());
        target_models.iter().for_each(|(_, export)| {
            let filename = export.filename.display();
            eprintln!("\t{filename}");
        });
        Ok(target_models)
    }
}
