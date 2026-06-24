// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use crate::Cached;
use crate::Result;
use crate::prelude as mu;

use microcad_lang_base::SourceKind;
use mu::document::CaptureDiags;

use microcad_lang::{eval::EvalContext, resolve::ResolveContext};
use microcad_lang_base::{DiagHandler, DiagRenderOptions, Diagnostics};
use microcad_lang_parse::Parse;
use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum SourceError {
    /// An IO error
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    /// An that occured during lowering
    #[error("Lower error: {0}")]
    LowerError(#[from] mu::lower::LowerError),

    /// No source code available.
    #[error("No source code loaded for {0}")]
    InvalidState(mu::Url),
}

/// A µcad source file document.
pub struct SourceFile {
    pub source: mu::Cached<mu::Source>,
    diagnostics: mu::Diagnostics,
    pub ast: Option<mu::Ast>,
    pub ir: Option<mu::Ir>,
    resolve_context: Option<ResolveContext>,
    eval_context: Option<EvalContext>,
    model: Option<mu::Model>,
}

impl SourceFile {
    pub fn new(source: mu::Cached<mu::Source>) -> Self {
        Self {
            diagnostics: Default::default(),
            source,
            ast: None,
            ir: None,
            resolve_context: None,
            eval_context: None,
            model: None,
        }
    }

    /// Loads the code from the file specified in the `url`.
    ///
    /// # Errors
    /// Returns an error if the URL is not a valid file path or if the file cannot be read.
    pub fn load_from_file(url: mu::Url, line_offset: u32) -> mu::Result<Self> {
        // 1. Convert the URL to a local file path
        let path: std::path::PathBuf = url
            .to_file_path()
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "The provided URL is not a valid local file path",
                )
            })
            .into_diagnostic()?;

        // 2. Read the file contents to a string
        let raw_code = std::fs::read_to_string(path).into_diagnostic()?;

        // 3. Construct and return the Source instance
        Ok(Self::new(Cached::new(mu::Source::new(
            url,
            line_offset,
            raw_code,
        ))))
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> mu::Result<Self> {
        Ok(Self::load_from_file(
            mu::locate::to_url(path.as_ref().as_os_str().to_str().unwrap())?,
            0,
        )?)
    }
}

impl mu::commands::GetCode for SourceFile {
    fn get_code(&self) -> Option<&str> {
        Some(self.source.code.as_str())
    }
}

impl mu::commands::SetCode for SourceFile {
    fn set_code(&mut self, code: String) -> Option<&str> {
        self.source = mu::Cached::new(mu::Source {
            url: self.source.url.clone(),
            line_offset: self.source.line_offset,
            code: mu::Hashed::new(code),
        });
        self.ast = None;
        self.ir = None;
        Some(self.source.code())
    }
}

impl CaptureDiags for SourceFile {
    fn diags(&self) -> &Diagnostics {
        &self.diagnostics
    }

    fn diags_mut(&mut self) -> &mut Diagnostics {
        &mut self.diagnostics
    }
}

impl mu::commands::Format for SourceFile {
    fn format(&mut self, params: &mu::commands::FormatParameters) -> Result<bool> {
        if let Some(ast) = &self.ast {
            let new_ast = self.capture_diags(microcad_lang_format::format_ast(ast, params));
            self.ast = new_ast;
            self.ir = None;
            self.resolve_context = None;
            self.eval_context = None;
            self.model = None;
        }

        if let Some(ast) = &self.ast {
            let old_code = self.source.as_ref();
            let formatted = ast.code.value() != old_code.code.value();
            // Reset state
            self.source = mu::Cached::new(mu::Source {
                url: self.source.url.clone(),
                line_offset: ast.line_offset,
                code: ast.code.clone(),
            });
            Ok(formatted)
        } else {
            Err(miette::miette!(
                "Source code has not been formatted successfully"
            ))
        }
    }
}

impl mu::commands::Sync for SourceFile {
    fn sync(&self) -> Result {
        std::fs::write(
            SourceKind::from(self.source.url.clone()).path().unwrap(),
            self.source.code.value().as_bytes(),
        )
        .into_diagnostic()
    }
}

impl mu::commands::compile::Parse for SourceFile {
    fn parse(&mut self) -> Result {
        let parse_context = mu::parse::ParseContext::from(self.source.as_ref());
        self.ast = Some(
            self.capture_diags(
                mu::Ast::parse(&parse_context).map_err(|err| err.to_diagnostics(&parse_context)),
            )
            .ok_or_else(|| miette::miette!("Failed to parse"))?,
        );
        self.ir = None;
        self.resolve_context = None;
        self.eval_context = None;
        self.model = None;
        Ok(())
    }
}

impl mu::commands::compile::Lower for SourceFile {
    fn lower(&mut self) -> Result {
        match &self.ast {
            Some(ast) => {
                let ir = mu::Ir::from_ast(ast, &mut self.diagnostics);
                self.ir = Some(
                    self.capture_diags(ir)
                        .ok_or_else(|| miette::miette!("Failed to lower"))?,
                );

                self.resolve_context = None;
                self.eval_context = None;
                self.model = None;
                Ok(())
            }
            None => Err(SourceError::InvalidState(self.source.url.clone()).into()),
        }
    }
}

impl mu::commands::compile::Resolve for SourceFile {
    fn resolve(
        &mut self,
        parameters: impl Into<mu::commands::compile::ResolveParameters>,
    ) -> Result<mu::Symbol> {
        let parameters = parameters.into();
        match &self.ir {
            Some(ir) => {
                self.eval_context = None;
                self.model = None;

                if let Ok(resolve_context) = ResolveContext::create(
                    Rc::new(ir.clone()),
                    parameters.search_paths,
                    match parameters.no_builtin {
                        true => None,
                        false => Some(microcad_builtin::builtin_module()),
                    },
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
            None => Err(SourceError::InvalidState(self.source.url.clone()).into()),
        }
    }
}

impl mu::commands::compile::Eval for SourceFile {
    fn eval(&mut self) -> Result<mu::Model> {
        if self.resolve_context.is_none() {
            return Err(SourceError::InvalidState(self.source.url.clone()).into());
        }

        let resolve_context = self.resolve_context.take();
        let resolve_context = match resolve_context {
            Some(resolve_context) => resolve_context,
            None => {
                return Err(SourceError::InvalidState(self.source.url.clone()).into());
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

impl mu::commands::Render for SourceFile {
    fn render(
        &mut self,
        parameters: impl Into<mu::commands::RenderParameters>,
    ) -> mu::Result<mu::Model> {
        if self.model.is_none() {
            return Err(SourceError::InvalidState(self.source.url.clone()).into());
        }

        let model = self.model.take();
        let parameters = parameters.into();
        let model = match model {
            Some(model) => model,
            None => {
                return Err(SourceError::InvalidState(self.source.url.clone()).into());
            }
        };

        if let Some(mut render_context) = self.capture_diags(mu::RenderContext::new(
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

impl mu::commands::Compile for SourceFile {}

impl mu::document::GetSymbol for SourceFile {
    fn get_symbol(
        &mut self,
        parameters: impl Into<mu::commands::compile::ResolveParameters>,
    ) -> mu::Result<mu::Symbol> {
        use crate::commands::compile::{Lower, Parse, Resolve};
        self.parse()?;
        self.lower()?;
        self.resolve(parameters)
    }
}

impl mu::commands::DocGen for SourceFile {}

impl mu::commands::PrintDiagnostics for SourceFile {
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
        } else if let Some(ast) = &self.ast {
            return diag.pretty_print(f, ast, options);
        } else {
            return diag.pretty_print(f, self.source.as_ref(), options);
        }
    }
}

impl mu::commands::Export for SourceFile {
    fn get_export_targets(
        &self,
        params: impl Into<mu::commands::ExportParameters>,
    ) -> mu::Result<mu::commands::ExportTargets> {
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
