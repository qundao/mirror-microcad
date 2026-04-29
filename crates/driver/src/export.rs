// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::{Exporter, ExporterRegistry};
use microcad_lang::{
    eval::EvalContext,
    model::{Model, OutputType},
    value::Value,
};

pub use microcad_lang::model::ExportCommand;

use crate::config::ExportConfig;

/// An export from a model
pub struct Export {
    /// Root model
    pub(crate) model: Model,
    pub(crate) input_path: std::path::PathBuf,
    pub(crate) output_path: Option<std::path::PathBuf>,
    pub(crate) config: ExportConfig,
    pub(crate) context: EvalContext,
}

impl Export {
    /// Get default exporter.
    pub fn default_exporter(
        &self,
        output_type: &OutputType,
    ) -> miette::Result<std::rc::Rc<dyn Exporter>> {
        use microcad_builtin::ExporterAccess;
        let exporters = self.context.exporters();

        match output_type {
            OutputType::NotDetermined => Err(miette::miette!("Could not determine output type.")),
            OutputType::Geometry2D => Ok(exporters.exporter_by_id(&(&self.config.sketch).into())?),
            OutputType::Geometry3D => Ok(exporters.exporter_by_id(&(&self.config.part).into())?),
            OutputType::InvalidMixed => Err(miette::miette!(
                "Invalid output type, the model cannot be exported."
            )),
        }
    }

    /// Get default export attribute.
    fn default_export_attribute(&self) -> miette::Result<ExportCommand> {
        use microcad_builtin::ExporterAccess;
        let default_exporter = self.default_exporter(&self.model.deduce_output_type());
        let resolution = self.config.render_resolution();
        let exporters = self.context.exporters();

        match &self.output_path {
            Some(filename) => Ok(ExportCommand {
                filename: filename.to_path_buf(),
                resolution,
                exporter: exporters
                    .exporter_by_filename(filename)
                    .or(default_exporter)?,
            }),
            None => {
                let mut filename = self.input_path.clone();
                let exporter = default_exporter?;

                let ext = exporter
                    .file_extensions()
                    .first()
                    .unwrap_or(&exporter.id())
                    .to_string();
                filename.set_extension(&ext);

                Ok(ExportCommand {
                    filename,
                    exporter,
                    resolution,
                })
            }
        }
    }

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
            models.push((self.model.clone(), self.default_export_attribute()?))
        }

        Ok(models)
    }

    pub fn export_targets(&self, models: &[(Model, ExportCommand)]) -> miette::Result<()> {
        models
            .iter()
            .try_for_each(|(model, export)| -> miette::Result<()> {
                let value = export.render_and_export(model)?;
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
