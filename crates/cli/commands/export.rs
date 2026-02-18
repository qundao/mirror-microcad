// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI export command

use microcad_builtin::*;
use microcad_core::RenderResolution;
use microcad_lang::{model::*, ty::*, value::*};

use crate::{config::Config, *};

/// Parse and evaluate and export a µcad file.
#[derive(clap::Parser)]
pub struct Export {
    #[clap(flatten)]
    pub eval: Eval,

    /// Output file (e.g. an SVG or STL).
    pub output: Option<std::path::PathBuf>,

    /// List all export target files.
    #[arg(short, long)]
    pub targets: bool,

    /// Omit export.
    #[arg(short, long)]
    pub dry_run: bool,

    /// The resolution of this export.
    ///
    /// The resolution can changed relatively `200%` or to an absolute value `0.05mm`.
    #[arg(short, long, default_value = "0.1mm")]
    pub resolution: String,
}

impl RunCommand<Vec<(Model, ExportCommand)>> for Export {
    fn run(&self, cli: &Cli) -> miette::Result<Vec<(Model, ExportCommand)>> {
        // run prior parse step
        let (context, model) = self.eval.run(cli)?;

        if let Some(model) = model {
            let target_models = self.target_models(&model, cli, context.exporters())?;

            if self.targets {
                self.list_targets(&target_models)?;
            }

            if !self.dry_run {
                let start = std::time::Instant::now();
                self.export_targets(&target_models)?;

                if cli.time {
                    eprintln!("Exporting Time : {}", Cli::time_to_string(&start.elapsed()));
                }
            }

            if cli.is_export() {
                if self.dry_run {
                    eprintln!("Did not export {} file(s) (dry-run!).", target_models.len());
                } else {
                    eprintln!("Exported {} file(s) successfully:", target_models.len());
                    target_models.iter().for_each(|(_, export)| {
                        let filename = export.filename.display();
                        eprintln!("\t{filename}");
                    })
                }
            }
            Ok(target_models)
        } else {
            miette::bail!("Model missing!")
        }
    }
}

impl Export {
    /// Get default exporter.
    fn default_exporter(
        output_type: &OutputType,
        config: &Config,
        exporters: &ExporterRegistry,
    ) -> miette::Result<std::rc::Rc<dyn Exporter>> {
        match output_type {
            OutputType::NotDetermined => Err(miette::miette!("Could not determine output type.")),
            OutputType::Geometry2D => {
                Ok(exporters.exporter_by_id(&(&config.export.sketch).into())?)
            }
            OutputType::Geometry3D => Ok(exporters.exporter_by_id(&(&config.export.part).into())?),
            OutputType::InvalidMixed => Err(miette::miette!(
                "Invalid output type, the model cannot be exported."
            )),
        }
    }

    /// Parse render resolution.
    pub fn resolution(&self) -> RenderResolution {
        use microcad_lang::*;

        use std::str::FromStr;
        let value = syntax::NumberLiteral::from_str(&self.resolution)
            .map(|literal| literal.value())
            .unwrap_or(value::Value::None);

        match value {
            value::Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Length,
            }) => RenderResolution::new(value),
            _ => {
                let default = RenderResolution::default();
                log::warn!(
                    "Invalid resolution `{resolution}`. Using default resolution: {value}mm",
                    resolution = self.resolution,
                    value = default.linear
                );
                default
            }
        }
    }

    /// Get default export attribute.
    fn default_export_attribute(
        &self,
        model: &Model,
        cli: &Cli,
        exporters: &ExporterRegistry,
    ) -> miette::Result<ExportCommand> {
        let default_exporter =
            Self::default_exporter(&model.deduce_output_type(), &cli.config, exporters);
        let resolution = self.resolution();

        match &self.output {
            Some(filename) => Ok(ExportCommand {
                filename: filename.to_path_buf(),
                resolution,
                exporter: exporters
                    .exporter_by_filename(filename)
                    .or(default_exporter)?,
            }),
            None => {
                let mut filename = self.eval.resolve.parse.input_with_ext(cli);
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
    pub fn target_models(
        &self,
        model: &Model,
        cli: &Cli,
        exporters: &ExporterRegistry,
    ) -> miette::Result<Vec<(Model, ExportCommand)>> {
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
            models.push((
                model.clone(),
                self.default_export_attribute(model, cli, exporters)?,
            ))
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
}
