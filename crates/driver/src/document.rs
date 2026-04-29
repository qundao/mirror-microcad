// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::{ExporterRegistry, Symbol};
use microcad_lang::{eval::EvalContext, resolve::ResolveContext};
use microcad_lang_base::{DiagHandler, DiagList};

use crate::{Model, RcMut, RenderContext, SourceFile, Url, config::ExportConfig, export::Export};

pub struct Document {
    input: std::path::PathBuf,
    code: Option<String>,
    /// The current source file being processed (if any).
    source_file: Option<std::rc::Rc<SourceFile>>,

    /// Model resulted from an evaluation.
    model: Option<Model>,

    diag: DiagList,

    render_context: Option<RcMut<RenderContext>>,
    search_paths: Vec<std::path::PathBuf>,
}

impl Document {
    pub fn new(input: std::path::PathBuf) -> Self {
        Self {
            input,
            code: None,
            source_file: None,
            model: None,
            diag: DiagList::default(),
            render_context: None,
            search_paths: vec![],
        }
    }

    pub fn load(&mut self) -> miette::Result<&mut Self> {
        self.source_file = Some(SourceFile::load(&self.input)?);

        Ok(self)
    }

    pub fn export(
        &self,
        config: ExportConfig,
        output_path: Option<std::path::PathBuf>,
    ) -> miette::Result<Export> {
        match &self.model {
            Some(model) => Ok(Export {
                model: model.clone(),
                input_path: self.input.clone(),
                output_path,
                config,
                context: self.eval_context()?,
            }),
            None => todo!(),
        }
    }

    pub fn resolve_context(&self) -> miette::Result<ResolveContext> {
        Ok(ResolveContext::create(
            self.source_file.clone().unwrap(),
            &self.search_paths,
            Some(microcad_builtin::builtin_module()),
            DiagHandler::default(),
        )?)
    }

    pub fn eval_context(&self) -> miette::Result<EvalContext> {
        Ok(EvalContext::new(
            self.resolve_context()?,
            microcad_lang_base::Stdout::new(),
            microcad_builtin::builtin_exporters(),
            microcad_builtin::builtin_importers(),
        ))
    }

    pub fn symbol(&self) -> miette::Result<Symbol> {
        let input = &self.input;
        // Handle special case for builtin symbol.
        if let Some(s) = input.to_str()
            && s == "__builtin"
        {
            return Ok(microcad_builtin::builtin_module());
        }

        let context = self.resolve_context()?;
        let symbol = context
            .root
            .get_child(&microcad_lang::syntax::Identifier::no_ref("mod")) // FIXME. This symbol should have same name as its parent directory (e.g. `std`)
            .expect("Symbol");

        Ok(symbol)
    }

    pub fn render(&mut self) {
        todo!()
    }
}
