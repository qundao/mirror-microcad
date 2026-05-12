// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_builtin::Symbol;
use microcad_lang::{
    eval::EvalContext,
    lower::ir,
    model::Model,
    render::{RenderContext, RenderWithContext},
    resolve::ResolveContext,
};
use microcad_lang_base::{DiagHandler, DiagRenderOptions, RcMut, ResourceLocation};
use microcad_lang_parse::{Parse, ParseContext, ast};
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

    /// An that occured during lowering
    #[error("Lower error: {0}")]
    LowerError(#[from] microcad_lang::lower::LowerError),

    /// No source code available.
    #[error("No source code loaded for {0}")]
    InvalidState(Url),
}

#[derive(Default)]
pub struct State {
    code: Option<String>,
    ast_source: Option<ast::Source>,
    ir_source: Option<Rc<ir::Source>>,
    resolve_context: Option<ResolveContext>,
    eval_context: Option<EvalContext>,
    model: Option<Model>,
}

impl commands::Format for document::Source {
    fn format(&self, params: &commands::FormatParameters) -> document::Result<bool> {
        let state = &mut *self.state.borrow_mut();
        if state.code.is_none() {
            return Err(RcMut::new(
                SourceError::InvalidState(self.url.clone()).into(),
            ));
        }

        if let Some(ast_source) = &mut state.ast_source {
            let old_code = state.code.as_ref().unwrap();
            let new_source = microcad_lang_format::format_source(&ast_source, &params)?;
            let formatted = new_source.source.value() != old_code;
            *ast_source = new_source;
            // Reset state
            state.ir_source = None;
            state.resolve_context = None;
            state.eval_context = None;
            state.model = None;
            state.code = Some(ast_source.source.value().clone());
            Ok(formatted)
        } else {
            return Err(RcMut::new(
                miette::miette!("You need to parse the source code first").into(),
            ));
        }
    }
}

impl commands::LoadFromFile for document::Source {
    fn load_from_file(&self) -> document::Result {
        let mut state = self.state.borrow_mut();
        let path = self
            .url
            .to_file_path()
            .map_err(|_| RcMut::new(SourceError::NoFileUrl(self.url.clone()).into()))?;

        state.code = Some(
            std::fs::read_to_string(path)
                .map_err(|err| RcMut::new(SourceError::IoError(err).into()))?,
        );
        Ok(())
    }
}

impl commands::Sync for document::Source {
    fn sync(&self) -> document::Result {
        let state = &*self.state.borrow();
        match &state.code {
            Some(code) => {
                std::fs::write(self.to_file_path().expect("path"), code.as_bytes())
                    .expect("No error");
                Ok(())
            }
            None => Err(RcMut::new(
                SourceError::InvalidState(self.url.clone()).into(),
            )),
        }
    }
}

impl commands::Pipeline for document::Source {
    fn parse(&self) -> document::Result {
        let state = &mut *self.state.borrow_mut();

        match &mut state.code {
            Some(code) => {
                let parse_context = ParseContext::new(&code).with_url(self.url.clone());
                state.ast_source = Some(
                    ast::Source::parse(&parse_context)
                        .map_err(|err| err.to_diagnostics(&parse_context))?,
                );
                state.ir_source = None;
                state.resolve_context = None;
                state.eval_context = None;
                state.model = None;
                Ok(())
            }
            None => Err(RcMut::new(
                SourceError::InvalidState(self.url.clone()).into(),
            )),
        }
    }

    fn lower(&self) -> document::Result {
        let state = &mut *self.state.borrow_mut();

        match &mut state.ast_source {
            Some(ast_source) => {
                state.ir_source = Some(
                    ir::Source::from_source(&ast_source).map_err(|err| RcMut::new(err.into()))?,
                );
                state.resolve_context = None;
                state.eval_context = None;
                state.model = None;
                Ok(())
            }
            None => Err(RcMut::new(
                SourceError::InvalidState(self.url.clone()).into(),
            )),
        }
    }

    fn resolve(&self, config: &Config) -> document::Result {
        let state = &mut *self.state.borrow_mut();

        match &mut state.ir_source {
            Some(ir_source) => {
                state.resolve_context = Some(
                    ResolveContext::create(
                        ir_source.clone(),
                        &config.search_paths,
                        Some(microcad_builtin::builtin_module()),
                        DiagHandler::default(),
                    )
                    .map_err(|err| RcMut::new(err.into()))?,
                );
                state.eval_context = None;
                state.model = None;
                Ok(())
            }
            None => Err(RcMut::new(
                SourceError::InvalidState(self.url.clone()).into(),
            )),
        }
    }

    fn eval(&self) -> document::Result {
        let state = &mut *self.state.borrow_mut();

        let resolve_context = std::mem::replace(&mut state.resolve_context, None);
        let resolve_context = match resolve_context {
            Some(resolve_context) => resolve_context,
            None => {
                return Err(RcMut::new(
                    SourceError::InvalidState(self.url.clone()).into(),
                ));
            }
        };

        state.eval_context = Some(microcad_lang::eval::EvalContext::new(
            resolve_context,
            microcad_lang_base::Stdout::new(),
            microcad_builtin::builtin_exporters(),
            microcad_builtin::builtin_importers(),
        ));

        match state.eval_context.as_mut().unwrap().eval() {
            Ok(model) => {
                state.model = model;
                Ok(())
            }
            Err(err) => {
                panic!("Eval error {err}");
            }
        }
    }
}

impl commands::Render for document::Source {
    fn render(&self, params: &commands::RenderParameters) -> document::Result {
        let state = &mut *self.state.borrow_mut();
        let model = std::mem::replace(&mut state.model, None);
        let model = match model {
            Some(model) => model,
            None => {
                return Err(RcMut::new(
                    SourceError::InvalidState(self.url.clone()).into(),
                ));
            }
        };

        let mut render_context = RenderContext::new(
            &model,
            params.resolution.clone(),
            params.cache.clone(),
            None,
        )
        .expect("Error handling");

        state.model = Some(
            model
                .render_with_context(&mut render_context)
                .expect("Error handling"),
        );
        Ok(())
    }
}

impl document::GetAssetSymbol for document::Source {
    fn get_symbol(&self) -> document::Result<Symbol> {
        let state = &*self.state.borrow();

        if let Some(eval_context) = &state.eval_context {
            return Ok(eval_context.root.clone());
        }

        if let Some(resolve_context) = &state.resolve_context {
            return Ok(resolve_context.root.clone());
        }

        panic!("Missing error handling")
    }
}

impl commands::DocGen for document::Source {}

impl commands::PrintDiagnostics for document::Source {
    fn print_diagnostics(
        &self,
        f: &mut dyn std::fmt::Write,
        options: &DiagRenderOptions,
    ) -> std::fmt::Result {
        let state = &*self.state.borrow();
        let diag = self.diagnostics.borrow();

        if let Some(eval_context) = &state.eval_context {
            return diag.pretty_print(f, eval_context, options);
        } else if let Some(resolve_context) = &state.resolve_context {
            return diag.pretty_print(f, resolve_context, options);
        } else if let Some(source) = &state.ast_source {
            return diag.pretty_print(f, source, options);
        }

        panic!("Missing error handling")
    }
}

impl commands::GetExportTargets for document::Source {
    fn get_export_targets(
        &self,
        params: &commands::GetExportTargetParameters,
    ) -> document::Result<commands::ExportTargets> {
        let state = &*self.state.borrow();

        if let Some(model) = &state.model {
            let exporters = state.eval_context.as_ref().unwrap().exporters();
            let default_exporter = params
                .default_exporter(&model.deduce_output_type(), exporters)
                .map_err(|err| RcMut::new(err.into()))?;
            let default_command = params
                .default_export_attribute_command(exporters, default_exporter)
                .map_err(|err| RcMut::new(err.into()))?;

            params
                .targets(model, default_command)
                .map_err(|err| RcMut::new(err.into()))
        } else {
            panic!("Error")
        }
    }
}

impl commands::Check for document::Source {
    fn check(&self, config: &Config) -> document::Result<bool> {
        use crate::commands::Pipeline;
        match self.load_from_file().and(self.run_pipeline(config)) {
            Ok(()) => Ok(true),
            Err(diag) => Err(diag),
        }
    }
}
