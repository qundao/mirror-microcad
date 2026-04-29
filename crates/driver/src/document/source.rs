// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::RenderResolution;
use microcad_lang::{
    eval::EvalContext,
    model::{ExportCommand, Model, OutputType},
    render::{RenderCache, RenderContext, RenderWithContext},
    resolve::ResolveContext,
    syntax::SourceFile,
};
use microcad_lang_base::{
    ComputedHash, DiagHandler, DiagRenderOptions, Diagnostics, GetSourceStrByHash, RcMut,
};
use miette::Diagnostic;
use thiserror::Error;
use url::Url;

use crate::{Export, document};

#[derive(Error, Debug, Diagnostic)]
pub enum SourceError {
    /// An error that occurs if the Url is not a file.
    #[error("No file URL: {0}")]
    NoFileUrl(Url),

    /// An IO error
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Default)]
pub enum State {
    #[default]
    Raw,
    Loaded {
        source: String,
    },
    Parsed {
        source: microcad_syntax::Source,
    },
    Lowered {
        source: microcad_syntax::Source,
        resolve_context: ResolveContext,
    },
    Resolved {
        source: microcad_syntax::Source,
        eval_context: EvalContext,
    },
    Evaluated {
        source: microcad_syntax::Source,
        eval_context: EvalContext,
        model: Model,
    },
    Rendered {
        source: microcad_syntax::Source,
        eval_context: EvalContext,
        model: Model,
    },
}

impl document::SourceItem {
    pub fn load_from_file(&'_ self) -> document::DiagResult<'_> {
        self.transition(|_| {
            let path = self
                .url
                .to_file_path()
                .map_err(|_| Diagnostics::single_error(SourceError::NoFileUrl(self.url.clone())))?;
            let source = std::fs::read_to_string(path)
                .map_err(|err| Diagnostics::single_error(SourceError::IoError(err)))?;
            Ok(State::Loaded { source })
        })
    }

    pub fn format(&'_ self) -> document::DiagResult<'_, bool> {
        let mut formatted = false;

        self.transition(|current| match current {
            State::Parsed { source } => {
                let hash = source.text.computed_hash();
                let config = microcad_lang_format::FormatConfig::from(&self.config.format);
                let source = microcad_lang_format::format_source(source, &config)?;
                formatted = source.text.computed_hash() != hash;

                Ok(State::Parsed { source })
            }
            _ => todo!(),
        })?;

        Ok(formatted)
    }

    pub fn sync(&'_ self) -> document::DiagResult<'_> {
        let state = &*self.state.borrow();
        let source = match state {
            State::Raw => todo!(),
            State::Loaded { .. } => todo!(),
            State::Parsed { source }
            | State::Lowered { source, .. }
            | State::Resolved { source, .. }
            | State::Evaluated { source, .. }
            | State::Rendered { source, .. } => &source.text,
        };
        std::fs::write(self.file_path().expect("path"), source.value()).expect("No error");
        Ok(())
    }

    pub fn parse(&'_ self) -> document::DiagResult<'_> {
        self.load_from_file()?;

        self.transition(|current| match current {
            State::Loaded { source } => {
                let source = microcad_syntax::Source::new(self.url.clone(), source)?;
                Ok(State::Parsed { source })
            }
            _ => Ok(current),
        })
    }

    pub fn lower(&'_ self) -> document::DiagResult<'_> {
        self.parse()?;

        self.transition(|current| match current {
            State::Parsed { source } => Ok(State::Lowered {
                resolve_context: ResolveContext::create(
                    SourceFile::from_source(&source).expect("Missing error handling"),
                    &self.config.search_paths,
                    Some(microcad_builtin::builtin_module()),
                    DiagHandler::default(),
                )
                .expect("Error handling"),
                source,
            }),
            _ => Ok(current),
        })
    }

    pub fn resolve(&'_ self) -> document::DiagResult<'_> {
        self.lower()?;

        self.transition(|current| match current {
            State::Lowered {
                source,
                resolve_context,
            } => Ok(State::Resolved {
                eval_context: EvalContext::new(
                    resolve_context,
                    microcad_lang_base::Stdout::new(),
                    microcad_builtin::builtin_exporters(),
                    microcad_builtin::builtin_importers(),
                ),
                source,
            }),
            _ => Ok(current),
        })
    }

    pub fn eval(&'_ self) -> document::DiagResult<'_> {
        self.resolve()?;

        self.transition(|current| match current {
            State::Resolved {
                source,
                mut eval_context,
            } => Ok(State::Evaluated {
                model: eval_context.eval().expect("No error").expect("A model"),
                eval_context,
                source,
            }
            .into()),
            _ => Ok(current),
        })
    }

    pub fn render(
        &'_ self,
        resolution: RenderResolution,
        cache: Option<RcMut<RenderCache>>,
    ) -> document::DiagResult<'_> {
        self.eval()?;

        self.transition(|current| match current {
            State::Evaluated {
                source,
                eval_context,
                model,
            } => {
                let mut render_context =
                    RenderContext::new(&model, resolution, cache, None).expect("Error handling");
                let model: Model = model
                    .render_with_context(&mut render_context)
                    .expect("Error handling");

                Ok(State::Rendered {
                    source,
                    model,
                    eval_context,
                })
            }
            _ => Ok(current),
        })
    }

    /// Generate documentation
    pub fn doc_gen(
        &'_ self,
        generator: Option<String>,
        output_path: Option<std::path::PathBuf>,
    ) -> miette::Result<()> {
        fn doc_generator(
            generator: Option<String>,
            output_path: Option<std::path::PathBuf>,
        ) -> miette::Result<Box<dyn microcad_docgen::DocGen>> {
            let name = generator.clone().unwrap_or("md".to_string());
            use microcad_docgen::*;
            match name.as_str() {
                "md" => Ok(Box::new(Md { output_path })),
                "mdbook" => Ok(Box::new(MdBook {
                    path: output_path.clone().unwrap_or_default(),
                })),
                _ => Err(miette::miette!("No generator with name `{name}`")),
            }
        }

        let generator = doc_generator(generator, output_path).expect("Impl Error handling");
        let state = &*self.state.borrow();

        let symbol = match state {
            State::Raw => todo!(),
            State::Loaded { .. } => todo!(),
            State::Parsed { .. } => todo!(),
            State::Lowered {
                resolve_context, ..
            } => &resolve_context.root,
            State::Resolved { eval_context, .. }
            | State::Evaluated { eval_context, .. }
            | State::Rendered { eval_context, .. } => &eval_context.root,
        };

        generator
            .doc_gen(symbol)
            .map_err(|err| miette::miette!("{err}"))
    }

    pub fn pretty_print(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        let state = &*self.state.borrow();
        let source = match state {
            State::Raw => todo!(),
            State::Loaded { .. } => todo!(),
            State::Parsed { source }
            | State::Lowered { source, .. }
            | State::Resolved { source, .. }
            | State::Evaluated { source, .. }
            | State::Rendered { source, .. } => source,
        };

        self.diagnostics.borrow().pretty_print(
            f,
            source,
            0,
            &microcad_lang_base::DiagRenderOptions::from(&self.config.diagnostics),
        )
    }

    pub fn export(&self, output_path: Option<std::path::PathBuf>) -> miette::Result<Export> {
        match &*self.state.borrow() {
            State::Rendered {
                eval_context,
                model,
                ..
            } => {
                use microcad_builtin::ExporterAccess;
                let output_type = model.deduce_output_type();
                let exporters = eval_context.exporters();
                let default_exporter = match output_type {
                    OutputType::NotDetermined => {
                        Err(miette::miette!("Could not determine output type."))
                    }
                    OutputType::Geometry2D => {
                        Ok(exporters.exporter_by_id(&(&self.config.export.sketch).into())?)
                    }
                    OutputType::Geometry3D => {
                        Ok(exporters.exporter_by_id(&(&self.config.export.part).into())?)
                    }
                    OutputType::InvalidMixed => Err(miette::miette!(
                        "Invalid output type, the model cannot be exported."
                    )),
                };
                let command = match &output_path {
                    Some(filename) => ExportCommand {
                        filename: filename.to_path_buf(),
                        resolution: self.config.export.render_resolution(),
                        exporter: exporters
                            .exporter_by_filename(filename)
                            .or(default_exporter)?,
                    },
                    None => {
                        let mut filename = self.file_path().expect("No error");
                        let exporter = default_exporter?;

                        let ext = exporter
                            .file_extensions()
                            .first()
                            .unwrap_or(&exporter.id())
                            .to_string();
                        filename.set_extension(&ext);

                        ExportCommand {
                            filename,
                            exporter,
                            resolution: self.config.export.render_resolution(),
                        }
                    }
                };

                Ok(Export {
                    model: model.clone(),
                    command,
                })
            }
            _ => todo!(),
        }
    }
}
