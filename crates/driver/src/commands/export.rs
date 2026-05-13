// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use crate::{config, document};

use microcad_builtin::{Exporter, ExporterRegistry};
use microcad_lang::{
    model::{Model, OutputType},
    value::Value,
};

pub use microcad_lang::model::ExportCommand;

pub struct ExportTargets(Vec<(Model, ExportCommand)>);

#[derive(Debug)]
pub struct ExportResult {
    pub model: Model,
    pub output_path: std::path::PathBuf,
    pub value: Value,
}

impl std::fmt::Display for ExportResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val_str = match &self.value {
            Value::None => String::new(),
            v => format!(" = {v}"),
        };
        write!(
            f,
            "{} @ {}{}",
            self.model,
            self.output_path.display(),
            val_str
        )
    }
}

#[derive(Debug, derive_more::Deref)]
pub struct ExportResults(Vec<ExportResult>);

impl std::fmt::Display for ExportResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|r| writeln!(f, "{r}"))?;
        if self.0.is_empty() {
            writeln!(f, "Nothing to export")
        } else {
            writeln!(f, "\nExported {} file(s) successfully.", self.0.len())
        }
    }
}

impl ExportTargets {
    pub fn export(&self) -> miette::Result<ExportResults> {
        let mut results = Vec::new();
        self.0
            .iter()
            .try_for_each(|(model, export)| -> miette::Result<()> {
                let value = export.export(model)?;
                results.push(ExportResult {
                    model: model.clone(),
                    output_path: export.filename.clone(),
                    value,
                });
                Ok(())
            })?;

        Ok(ExportResults(results))
    }

    pub fn list(&self) -> miette::Result<()> {
        for (model, attr) in &self.0 {
            eprintln!("{model} => {attr}");
        }
        Ok(())
    }
}

pub struct GetExportTargetParameters {
    /// Input file path (used as template when output path is not set).
    pub input_path: std::path::PathBuf,
    /// Output file path.
    pub output_path: Option<std::path::PathBuf>,
    /// Export configuration
    pub config: config::ExportConfig,
}

impl GetExportTargetParameters {
    pub fn default_exporter(
        &self,
        output_type: &OutputType,
        exporters: &ExporterRegistry,
    ) -> miette::Result<Rc<dyn Exporter>> {
        use microcad_builtin::ExporterAccess;

        match output_type {
            OutputType::NotDetermined => Err(miette::miette!("Could not determine output type.")),
            OutputType::Geometry2D => Ok(exporters.exporter_by_id(&(&self.config.sketch).into())?),
            OutputType::Geometry3D => Ok(exporters.exporter_by_id(&(&self.config.part).into())?),
            OutputType::InvalidMixed => Err(miette::miette!(
                "Invalid output type, the model cannot be exported."
            )),
        }
    }

    pub fn default_export_attribute_command(
        &self,
        exporters: &ExporterRegistry,
        default_exporter: Rc<dyn Exporter>,
    ) -> miette::Result<ExportCommand> {
        use microcad_builtin::ExporterAccess;
        let resolution = self.config.render_resolution();

        Ok(match &self.output_path {
            Some(filename) => ExportCommand {
                filename: filename.clone(),
                resolution,
                exporter: exporters
                    .exporter_by_filename(filename)
                    .unwrap_or(default_exporter),
            },
            None => {
                let mut filename = self.input_path.clone();
                let ext = default_exporter
                    .file_extensions()
                    .first()
                    .unwrap_or(&default_exporter.id())
                    .to_string();
                filename.set_extension(&ext);

                ExportCommand {
                    filename,
                    exporter: default_exporter,
                    resolution,
                }
            }
        })
    }

    pub fn targets(
        &self,
        model: &Model,
        default_command: ExportCommand,
    ) -> miette::Result<ExportTargets> {
        use microcad_lang::model::AttributesAccess;
        let mut models = model
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
            models.push((model.clone(), default_command.clone()))
        }

        Ok(ExportTargets(models))
    }
}

pub trait Export {
    fn get_export_targets(
        &self,
        params: &GetExportTargetParameters,
    ) -> miette::Result<ExportTargets>;

    fn export(&self, params: &GetExportTargetParameters) -> miette::Result<ExportResults> {
        self.get_export_targets(params)?.export()
    }
}
