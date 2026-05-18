// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use crate::Result;
use crate::prelude::*;

use document::{CaptureDiags, TryFilePath, commands::LoadFromFile};

use microcad_lang::{eval::EvalContext, resolve::ResolveContext};
use microcad_lang_base::{DiagHandler, DiagRenderOptions, Diagnostics, ResourceLocation};
use microcad_lang_parse::Parse;
use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;

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
    diagnostics: Diagnostics,
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
            diagnostics: Default::default(),
            base_source: None,
            ast_source: None,
            ir_source: None,
            resolve_context: None,
            eval_context: None,
            model: None,
        }
    }

    /// Load a document from a url.
    pub fn load(url: Url) -> Result<Self> {
        use commands::LoadFromFile;
        let mut document = Self::new(url);
        document.load_from_file()?;
        Ok(document)
    }

    pub fn from_source(source: base::Source) -> Self {
        let mut self_ = Self::new(source.url.clone());
        self_.base_source = Some(source);
        self_
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let mut s = Self::new(crate::locate::to_url(
            path.as_ref().as_os_str().to_str().unwrap(),
        )?);
        s.load_from_file()?;
        Ok(s)
    }
}

impl document::GetCode for Source {
    fn get_code(&self) -> Option<&str> {
        self.base_source.as_ref().map(|s| s.code.value().as_str())
    }
}

impl ResourceLocation for Source {
    fn url(&self) -> &Url {
        &self.url
    }
}

impl TryFilePath for Source {}

impl CaptureDiags for Source {
    fn diags(&self) -> &Diagnostics {
        &self.diagnostics
    }

    fn diags_mut(&mut self) -> &mut Diagnostics {
        &mut self.diagnostics
    }
}

impl commands::Format for Source {
    fn format(&mut self, params: &commands::FormatParameters) -> Result<bool> {
        if self.base_source.is_none() {
            return Err(SourceError::InvalidState(self.url.clone()).into());
        }

        if let Some(ast_source) = &self.ast_source {
            let new_source =
                self.capture_diags(microcad_lang_format::format_source(ast_source, params));
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
                let parse_context = parse::ParseContext::from(base_source);
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
                    self.capture_diags(ir::Source::from_source(ast_source))
                        .ok_or_else(|| miette::miette!("Failed to lower"))?,
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
    fn resolve(
        &mut self,
        parameters: impl Into<commands::compile::ResolveParameters>,
    ) -> Result<Symbol> {
        let parameters = parameters.into();
        match &self.ir_source {
            Some(ir_source) => {
                self.eval_context = None;
                self.model = None;

                if let Ok(resolve_context) = ResolveContext::create(
                    ir_source.clone(),
                    parameters.search_paths,
                    Some(microcad_builtin::builtin_module()),
                    DiagHandler::default(),
                ) {
                    self.diagnostics
                        .append(resolve_context.diag.diagnostics.clone());

                    if self.diagnostics.has_errors() {
                        self.resolve_context = None;
                        Err(miette::miette!("Failed to resolve"))
                    } else {
                        self.resolve_context = Some(resolve_context);
                        Ok(self.resolve_context.as_ref().unwrap().root.clone())
                    }
                } else {
                    Err(miette::miette!("Failed to resolve"))
                }
            }
            None => Err(SourceError::InvalidState(self.url.clone()).into()),
        }
    }
}

impl commands::compile::Eval for document::Source {
    fn eval(&mut self) -> Result<Model> {
        if self.resolve_context.is_none() {
            return Err(SourceError::InvalidState(self.url.clone()).into());
        }

        let resolve_context = self.resolve_context.take();
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
                self.diagnostics
                    .append(eval_context.diag.diagnostics.clone());

                if self.diags().has_errors() {
                    self.model = None;
                    self.eval_context = None;
                    Err(miette::miette!("Error during evaluation"))
                } else {
                    self.model = Some(model.clone());
                    self.eval_context = Some(eval_context);
                    Ok(model)
                }
            }
            Err(err) => Err(miette::miette!("{err}")),
        }
    }
}

impl commands::compile::Render for document::Source {
    fn render(
        &mut self,
        parameters: impl Into<commands::compile::RenderParameters>,
    ) -> document::Result<Model> {
        if self.model.is_none() {
            return Err(SourceError::InvalidState(self.url.clone()).into());
        }

        let model = self.model.take();
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
            use crate::prelude::RenderWithContext;
            self.model = self.capture_diags(model.render_with_context(&mut render_context));
        }

        if let Some(model) = &self.model {
            Ok(model.clone())
        } else {
            Err(miette::miette!("No model to render"))
        }
    }
}

impl commands::Compile for document::Source {}

impl document::GetSymbol for document::Source {
    fn get_symbol(
        &mut self,
        parameters: impl Into<commands::compile::ResolveParameters>,
    ) -> document::Result<Symbol> {
        use crate::commands::compile::{Lower, Parse, Resolve};
        self.parse()?;
        self.lower()?;
        self.resolve(parameters)
    }
}

impl commands::DocGen for document::Source {}

impl commands::PrintDiagnostics for document::Source {
    fn print_diagnostics(
        &self,
        f: &mut dyn std::fmt::Write,
        options: &DiagRenderOptions,
    ) -> std::fmt::Result {
        let diag = self.diags();
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
        params: impl Into<commands::ExportParameters>,
    ) -> document::Result<commands::ExportTargets> {
        let params = params.into();
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
