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
use miette::Diagnostic;
use thiserror::Error;
use url::Url;

use crate::{
    Config, base,
    commands::{self, LoadFromFile},
    document::{self, TryFilePath},
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
}

impl ResourceLocation for Source {
    fn url(&self) -> &Url {
        &self.url
    }
}

impl TryFilePath for Source {}

impl commands::Format for Source {
    fn format(&mut self, params: &commands::FormatParameters) -> document::Result<bool> {
        if self.base_source.is_none() {
            return Err(RcMut::new(
                SourceError::InvalidState(self.url.clone()).into(),
            ));
        }

        if let Some(ast_source) = &mut self.ast_source {
            let old_code = self.base_source.as_ref().unwrap();
            let new_source = microcad_lang_format::format_source(&ast_source, &params)?;
            let formatted = new_source.code.value() != old_code.code.value();
            *ast_source = new_source;
            // Reset state
            self.ir_source = None;
            self.resolve_context = None;
            self.eval_context = None;
            self.model = None;
            self.base_source = Some(base::Source {
                url: self.url.clone(),
                line_offset: ast_source.line_offset,
                code: ast_source.code.clone(),
            });
            Ok(formatted)
        } else {
            return Err(RcMut::new(
                miette::miette!("You need to parse the source code first").into(),
            ));
        }
    }
}

impl commands::LoadFromFile for document::Source {
    fn load_from_file(&mut self) -> document::Result {
        let path = self
            .url
            .to_file_path()
            .map_err(|_| RcMut::new(SourceError::NoFileUrl(self.url.clone()).into()))?;

        self.base_source = Some(base::Source::new(
            self.url.clone(),
            0,
            std::fs::read_to_string(path)
                .map_err(|err| RcMut::new(SourceError::IoError(err).into()))?,
        ));
        Ok(())
    }
}

impl commands::Sync for document::Source {
    fn sync(&self) -> document::Result {
        match &self.base_source {
            Some(base_source) => {
                std::fs::write(self.try_file_path()?, base_source.code.value().as_bytes())
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
    fn parse(&mut self) -> document::Result {
        match &self.base_source {
            Some(base_source) => {
                let parse_context = ParseContext::from(base_source).with_url(self.url.clone());
                self.ast_source = Some(
                    ast::Source::parse(&parse_context)
                        .map_err(|err| err.to_diagnostics(&parse_context))?,
                );
                self.ir_source = None;
                self.resolve_context = None;
                self.eval_context = None;
                self.model = None;
                Ok(())
            }
            None => Err(RcMut::new(
                SourceError::InvalidState(self.url.clone()).into(),
            )),
        }
    }

    fn lower(&mut self) -> document::Result {
        match &self.ast_source {
            Some(ast_source) => {
                self.ir_source = Some(
                    ir::Source::from_source(&ast_source).map_err(|err| RcMut::new(err.into()))?,
                );
                self.resolve_context = None;
                self.eval_context = None;
                self.model = None;
                Ok(())
            }
            None => Err(RcMut::new(
                SourceError::InvalidState(self.url.clone()).into(),
            )),
        }
    }

    fn resolve(&mut self, config: &Config) -> document::Result {
        match &self.ir_source {
            Some(ir_source) => {
                self.resolve_context = Some(
                    ResolveContext::create(
                        ir_source.clone(),
                        &config.search_paths,
                        Some(microcad_builtin::builtin_module()),
                        DiagHandler::default(),
                    )
                    .map_err(|err| RcMut::new(err.into()))?,
                );
                self.eval_context = None;
                self.model = None;
                Ok(())
            }
            None => Err(RcMut::new(
                SourceError::InvalidState(self.url.clone()).into(),
            )),
        }
    }

    fn eval(&mut self) -> document::Result {
        let resolve_context = std::mem::replace(&mut self.resolve_context, None);
        let resolve_context = match resolve_context {
            Some(resolve_context) => resolve_context,
            None => {
                return Err(RcMut::new(
                    SourceError::InvalidState(self.url.clone()).into(),
                ));
            }
        };

        self.eval_context = Some(microcad_lang::eval::EvalContext::new(
            resolve_context,
            microcad_lang_base::Stdout::new(),
            microcad_builtin::builtin_exporters(),
            microcad_builtin::builtin_importers(),
        ));

        match self.eval_context.as_mut().unwrap().eval() {
            Ok(model) => {
                self.model = model;
                Ok(())
            }
            Err(err) => {
                panic!("Eval error {err}");
            }
        }
    }
}

impl commands::Render for document::Source {
    fn render(&mut self, params: &commands::RenderParameters) -> document::Result {
        let model = std::mem::replace(&mut self.model, None);
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

        self.model = Some(
            model
                .render_with_context(&mut render_context)
                .expect("Error handling"),
        );
        Ok(())
    }
}

impl document::GetAssetSymbol for document::Source {
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

impl commands::GetExportTargets for document::Source {
    fn get_export_targets(
        &self,
        params: &commands::GetExportTargetParameters,
    ) -> document::Result<commands::ExportTargets> {
        if let Some(model) = &self.model {
            let exporters = self.eval_context.as_ref().unwrap().exporters();
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
    fn check(&mut self, config: &Config) -> document::Result<bool> {
        use crate::commands::Pipeline;
        match self.load_from_file().and(self.run_pipeline(config)) {
            Ok(()) => Ok(true),
            Err(diag) => Err(diag),
        }
    }
}
