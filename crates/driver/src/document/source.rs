// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::Symbol;
use microcad_lang::{
    eval::EvalContext,
    lower::ir::SourceFile,
    model::Model,
    render::{RenderContext, RenderWithContext},
    resolve::ResolveContext,
};
use microcad_lang_base::{ComputedHash, DiagHandler, DiagRenderOptions, RcMut, ResourceLocation};
use microcad_lang_parse::{Parse, ParseContext, Source};
use miette::Diagnostic;
use thiserror::Error;
use url::Url;

use crate::{
    Config,
    commands::{self, LoadFromFile},
    document,
};

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
        source: Source,
    },
    Lowered {
        source: Source,
        resolve_context: ResolveContext,
    },
    Resolved {
        source: Source,
        eval_context: EvalContext,
    },
    Evaluated {
        source: Source,
        eval_context: EvalContext,
        model: Model,
    },
    Rendered {
        source: Source,
        eval_context: EvalContext,
        model: Model,
    },
}

impl commands::Format for document::SourceAsset {
    fn format(&self, params: &commands::FormatParameters) -> document::Result<bool> {
        let mut formatted = false;
        let config = params;

        self.transition(|current| match current {
            State::Parsed { source } => {
                let hash = source.source.computed_hash();
                let source = microcad_lang_format::format_source(source, &config)?;
                formatted = source.source.computed_hash() != hash;

                Ok(State::Parsed { source })
            }
            _ => todo!(),
        })?;

        Ok(formatted)
    }
}

impl commands::LoadFromFile for document::SourceAsset {
    fn load_from_file(&self) -> document::Result {
        self.transition(|_| {
            let path = self
                .url
                .to_file_path()
                .map_err(|_| SourceError::NoFileUrl(self.url.clone()))?;
            let source = std::fs::read_to_string(path).map_err(|err| SourceError::IoError(err))?;
            Ok(State::Loaded { source })
        })
    }
}

impl commands::Sync for document::SourceAsset {
    fn sync(&self) -> document::Result {
        let state = &*self.state.borrow();
        let source = match state {
            State::Raw => todo!(),
            State::Loaded { .. } => todo!(),
            State::Parsed { source }
            | State::Lowered { source, .. }
            | State::Resolved { source, .. }
            | State::Evaluated { source, .. }
            | State::Rendered { source, .. } => &source.source,
        };
        std::fs::write(self.to_file_path().expect("path"), source.value()).expect("No error");
        Ok(())
    }
}

impl commands::Pipeline for document::SourceAsset {
    fn parse(&self) -> document::Result {
        self.transition(|current| match current {
            State::Loaded { source } => {
                let parse_context = ParseContext::new(&source).with_url(self.url.clone());
                let source = microcad_lang_parse::Source::parse(&parse_context)?;
                Ok(State::Parsed { source })
            }
            _ => Ok(current),
        })
    }

    fn lower(&self, config: &Config) -> document::Result {
        self.transition(|current| match current {
            State::Parsed { source } => Ok(State::Lowered {
                resolve_context: ResolveContext::create(
                    SourceFile::from_source(&source).expect("Missing error handling"),
                    &config.search_paths,
                    Some(microcad_builtin::builtin_module()),
                    DiagHandler::default(),
                )
                .expect("Error handling"),
                source,
            }),
            _ => Ok(current),
        })
    }

    fn resolve(&self) -> document::Result {
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

    fn eval(&self) -> document::Result {
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
}

impl commands::Render for document::SourceAsset {
    fn render(&self, params: &commands::RenderParameters) -> document::Result {
        commands::Pipeline::eval(self)?;

        self.transition(|current| match current {
            State::Evaluated {
                source,
                eval_context,
                model,
            } => {
                let mut render_context = RenderContext::new(
                    &model,
                    params.resolution.clone(),
                    params.cache.clone(),
                    None,
                )
                .expect("Error handling");
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
}

impl document::GetAssetSymbol for document::SourceAsset {
    fn get_symbol(&self) -> document::Result<Symbol> {
        Ok(match &*self.state.borrow() {
            State::Raw => todo!(),
            State::Loaded { .. } => todo!(),
            State::Parsed { .. } => todo!(),
            State::Lowered {
                resolve_context, ..
            } => resolve_context.root.clone(),
            State::Resolved { eval_context, .. }
            | State::Evaluated { eval_context, .. }
            | State::Rendered { eval_context, .. } => eval_context.root.clone(),
        })
    }
}

impl commands::DocGen for document::SourceAsset {}

impl commands::PrintDiagnostics for document::SourceAsset {
    fn print_diagnostics(
        &self,
        f: &mut dyn std::fmt::Write,
        options: &DiagRenderOptions,
    ) -> std::fmt::Result {
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

        self.diagnostics.borrow().pretty_print(f, source, &options)
    }
}

impl commands::GetExportTargets for document::SourceAsset {
    fn get_export_targets(
        &self,
        params: &commands::GetExportTargetParameters,
    ) -> document::Result<commands::ExportTargets> {
        match &*self.state.borrow() {
            State::Rendered {
                eval_context,
                model,
                ..
            } => {
                let exporters = eval_context.exporters();
                let default_exporter = params
                    .default_exporter(&model.deduce_output_type(), eval_context.exporters())
                    .map_err(|err| RcMut::new(err.into()))?;
                let default_command = params
                    .default_export_attribute_command(exporters, default_exporter)
                    .map_err(|err| RcMut::new(err.into()))?;

                params
                    .targets(model, default_command)
                    .map_err(|err| RcMut::new(err.into()))
            }
            _ => todo!(),
        }
    }
}

impl commands::Check for document::SourceAsset {
    fn check(&self, config: &Config) -> document::Result<bool> {
        use crate::commands::Pipeline;
        match self.load_from_file().and(self.run_pipeline(config)) {
            Ok(()) => Ok(true),
            Err(diag) => Err(diag),
        }
    }
}
