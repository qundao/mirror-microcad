// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_builtin::Symbol;
use microcad_lang::{
    eval::EvalContext,
    model::Model,
    render::{RenderContext, RenderWithContext},
    resolve::ResolveContext,
};
use microcad_lang_base::{DiagHandler, DiagRenderOptions, Diagnostics, RcMut, ResourceLocation};
use microcad_lang_parse::{Parse, ParseContext, ast};
use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;
use url::Url;

use crate::{
    Result, base, commands,
    document::{self, CaptureDiags, TryFilePath},
    ir,
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

pub struct Source {
    /// Each source item keeps its [`Diagnostics`]
    pub url: Url,
    pub diagnostics: RcMut<Diagnostics>,
    base_source: Option<base::Source>,
    ast_source: Option<ast::Source>,
    ir_source: Option<Rc<ir::Source>>,
    resolve_context: Option<ResolveContext>,
    eval_context: Option<EvalContext>,
    model: Option<Model>,
}

impl Source {
    pub fn new(url: Url) -> Self {
        Self {
            url,
            diagnostics: RcMut::new(Default::default()),
            base_source: None,
            ast_source: None,
            ir_source: None,
            resolve_context: None,
            eval_context: None,
            model: None,
        }
    }

    pub fn from_source(source: base::Source) -> Self {
        Self {
            url: source.url.clone(),
            diagnostics: RcMut::new(Default::default()),
            base_source: Some(source),
            ast_source: None,
            ir_source: None,
            resolve_context: None,
            eval_context: None,
            model: None,
        }
    }
}

impl ResourceLocation for Source {
    fn url(&self) -> &Url {
        &self.url
    }
}

impl TryFilePath for Source {}

impl CaptureDiags for Source {
    fn diags(&self) -> RcMut<Diagnostics> {
        self.diagnostics.clone()
    }
}

impl commands::Format for Source {
    fn format(&mut self, params: &commands::FormatParameters) -> Result<bool> {
        if self.base_source.is_none() {
            return Err(SourceError::InvalidState(self.url.clone()).into());
        }

        if let Some(ast_source) = &self.ast_source {
            let new_source =
                self.capture_diags(microcad_lang_format::format_source(&ast_source, &params));
            self.ast_source = new_source;
            self.ir_source = None;
            self.resolve_context = None;
            self.eval_context = None;
            self.model = None;
        }

        if let Some(ast_source) = &self.ast_source {
            let old_code = self.base_source.as_ref().unwrap();
            let formatted = ast_source.code.value() != old_code.code.value();
            // Reset state
            self.base_source = Some(base::Source {
                url: self.url.clone(),
                line_offset: ast_source.line_offset,
                code: ast_source.code.clone(),
            });
            Ok(formatted)
        } else {
            Err(miette::miette!(
                "Source code has not been formatted successfully"
            ))
        }
    }
}

impl commands::LoadFromFile for document::Source {
    fn load_from_file(&mut self) -> Result {
        let path = self
            .url
            .to_file_path()
            .map_err(|_| SourceError::NoFileUrl(self.url.clone()))?;

        self.base_source = Some(base::Source::new(
            self.url.clone(),
            0,
            std::fs::read_to_string(path).into_diagnostic()?,
        ));
        Ok(())
    }
}

impl commands::Sync for document::Source {
    fn sync(&self) -> Result {
        match &self.base_source {
            Some(base_source) => {
                std::fs::write(self.try_file_path()?, base_source.code.value().as_bytes())
                    .into_diagnostic()
            }
            None => Err(SourceError::InvalidState(self.url.clone()).into()),
        }
    }
}

impl commands::compile::Parse for document::Source {
    fn parse(&mut self) -> Result {
        match &self.base_source {
            Some(base_source) => {
                let parse_context = ParseContext::from(base_source);
                self.ast_source = Some(
                    self.capture_diags(
                        ast::Source::parse(&parse_context)
                            .map_err(|err| err.to_diagnostics(&parse_context)),
                    )
                    .ok_or_else(|| miette::miette!("Failed to parse"))?,
                );
                self.ir_source = None;
                self.resolve_context = None;
                self.eval_context = None;
                self.model = None;
                Ok(())
            }
            None => Err(SourceError::InvalidState(self.url.clone()).into()),
        }
    }
}

impl commands::compile::Lower for document::Source {
    fn lower(&mut self) -> Result {
        match &self.ast_source {
            Some(ast_source) => {
                self.ir_source = Some(
                    self.capture_diags(ir::Source::from_source(&ast_source))
                        .ok_or_else(|| miette::miette!("Failed to parse"))?,
                );

                self.resolve_context = None;
                self.eval_context = None;
                self.model = None;
                Ok(())
            }
            None => Err(SourceError::InvalidState(self.url.clone()).into()),
        }
    }
}

impl commands::compile::Resolve for document::Source {
    fn resolve(&mut self, parameters: impl Into<commands::compile::ResolveParameters>) -> Result {
        let parameters = parameters.into();
        match &self.ir_source {
            Some(ir_source) => {
                self.resolve_context = Some(
                    self.capture_diags(ResolveContext::create(
                        ir_source.clone(),
                        parameters.search_paths,
                        Some(microcad_builtin::builtin_module()),
                        DiagHandler::default(),
                    ))
                    .ok_or_else(|| miette::miette!("Failed to parse"))?,
                );

                self.eval_context = None;
                self.model = None;
                Ok(())
            }
            None => Err(SourceError::InvalidState(self.url.clone()).into()),
        }
    }
}

impl commands::compile::Eval for document::Source {
    fn eval(&mut self) -> Result {
        let resolve_context = std::mem::replace(&mut self.resolve_context, None);
        let resolve_context = match resolve_context {
            Some(resolve_context) => resolve_context,
            None => {
                return Err(SourceError::InvalidState(self.url.clone()).into());
            }
        };

        let mut eval_context = microcad_lang::eval::EvalContext::new(
            resolve_context,
            microcad_lang_base::Stdout::new(),
            microcad_builtin::builtin_exporters(),
            microcad_builtin::builtin_importers(),
        );

        match eval_context.eval() {
            Ok(model) => {
                if eval_context.diag.error_count() > 0 {
                    let diag = RcMut::new(eval_context.diag.diagnostics);
                    self.diagnostics = diag.clone();
                    self.model = None;
                    self.eval_context = None;
                    Err(miette::miette!("Error during evaluation"))
                } else {
                    self.model = model;
                    self.eval_context = Some(eval_context);
                    Ok(())
                }
            }
            Err(err) => {
                panic!("Eval error {err}");
            }
        }
    }
}

impl commands::compile::Render for document::Source {
    fn render(
        &mut self,
        parameters: impl Into<commands::compile::RenderParameters>,
    ) -> document::Result {
        let model = std::mem::replace(&mut self.model, None);
        let parameters = parameters.into();
        let model = match model {
            Some(model) => model,
            None => {
                return Err(SourceError::InvalidState(self.url.clone()).into());
            }
        };

        if let Some(mut render_context) = self.capture_diags(RenderContext::new(
            &model,
            parameters.resolution,
            parameters.cache,
            None,
        )) {
            self.model = self.capture_diags(model.render_with_context(&mut render_context));
        }

        Ok(())
    }
}

impl commands::Compile for document::Source {}

impl document::GetSymbol for document::Source {
    fn get_symbol(&self) -> document::Result<Symbol> {
        if let Some(eval_context) = &self.eval_context {
            return Ok(eval_context.root.clone());
        }

        if let Some(resolve_context) = &self.resolve_context {
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
        let diag = self.diagnostics.borrow();

        if let Some(eval_context) = &self.eval_context {
            return diag.pretty_print(f, eval_context, options);
        } else if let Some(resolve_context) = &self.resolve_context {
            return diag.pretty_print(f, resolve_context, options);
        } else if let Some(ast_source) = &self.ast_source {
            return diag.pretty_print(f, ast_source, options);
        } else if let Some(base_source) = &self.base_source {
            return diag.pretty_print(f, base_source, options);
        }

        panic!("Missing error handling")
    }
}

impl commands::Export for document::Source {
    fn get_export_targets(
        &self,
        params: &commands::ExportParameters,
    ) -> document::Result<commands::ExportTargets> {
        if let Some(model) = &self.model {
            let exporters = self.eval_context.as_ref().unwrap().exporters();
            let default_exporter =
                params.default_exporter(&model.deduce_output_type(), exporters)?;
            let default_command =
                params.default_export_attribute_command(exporters, default_exporter)?;

            params.targets(model, default_command)
        } else {
            Err(miette::miette!("No model to export"))
        }
    }
}
