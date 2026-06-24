// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, LowerErrorsWithSource, ir};

use microcad_lang_base::{Diagnostics, Hashed, SrcReferrer, Url, virtual_url};
use microcad_lang_parse::ast;

impl ir::Source {
    pub fn from_ast(
        source: &ast::Source,
        diagnostics: &mut Diagnostics,
    ) -> Result<Self, LowerError> {
        let mut context =
            LowerContext::new(source.code.as_str()).with_line_offset(source.line_offset);
        let source = Self {
            doc: None,
            statements: ir::StatementList::lower(&source.ast.statements, &mut context)?,
            source: source.code.clone(),
            name: ir::QualifiedName::default(),
            url: source.url.clone(),
            line_offset: source.line_offset,
        };
        diagnostics.append(context.diagnostics);
        Ok(source)
    }

    /// Load µcad source file from given `path`
    pub fn load_with_name(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
        name: ir::QualifiedName,
        diagnostics: &mut Diagnostics,
    ) -> (std::rc::Rc<Self>, Option<LowerErrorsWithSource>) {
        let path = path.as_ref();
        log::trace!(
            "{load} file {path} [{name}]",
            path = path.display(),
            load = microcad_lang_base::mark!(LOAD)
        );

        let buf = match std::fs::read_to_string(path) {
            Ok(buf) => buf,
            Err(error) => {
                let error = LowerError::LoadSource(name.src_ref(), path.into(), error);
                let mut source_file = ir::Source::new(
                    None,
                    ir::StatementList::default(),
                    Hashed::new(String::new()),
                    Url::from_file_path(path).unwrap_or(virtual_url("invalid")),
                );
                source_file.name = name;
                return (std::rc::Rc::new(source_file), Some(error.into()));
            }
        };

        let (mut source_file, errors) = Self::load_inner(None, path, &buf, 0, diagnostics);
        source_file.name = name;
        log::debug!(
            "Successfully loaded external file {} to {}",
            path.display(),
            source_file.name
        );

        (std::rc::Rc::new(source_file), errors)
    }

    fn load_inner(
        name: Option<&str>,
        path: impl AsRef<std::path::Path>,
        source_str: &str,
        line_offset: u32,
        diagnostics: &mut Diagnostics,
    ) -> (Self, Option<LowerErrorsWithSource>) {
        log::trace!(
            "{load} source from string",
            load = microcad_lang_base::mark!(LOAD)
        );
        let mut lower_context = LowerContext::new(source_str).with_line_offset(line_offset);

        let dummy_source = || {
            ir::Source::new(
                None,
                ir::StatementList::default(),
                Hashed::new(source_str.into()),
                Url::from_file_path(&path).unwrap_or(virtual_url("dummy")),
            )
            .with_line_offset(line_offset)
        };

        let ast = match crate::lower::lower::build_ast(source_str, &mut lower_context) {
            Ok(ast) => ast,
            Err(error) => {
                return (dummy_source(), Some(error));
            }
        };

        let mut source_file =
            match Self::lower(&ast, &mut lower_context).map_err(|error| vec![error]) {
                Ok(source_file) => source_file,
                Err(errors) => {
                    return (
                        dummy_source(),
                        Some(LowerErrorsWithSource {
                            errors,
                            source_code: Some(Hashed::new(source_str.into())),
                        }),
                    );
                }
            };
        if let Some(name) = name {
            source_file.set_name(ir::Identifier::no_ref(name).into());
        } else {
            source_file.set_name(Self::name_from_path(&path));
        };
        source_file.set_filename(path);
        log::debug!("Successfully loaded source from string");

        diagnostics.append(lower_context.diagnostics);

        (source_file.with_line_offset(line_offset), None)
    }

    /// Get the source file name from path.
    pub fn name_from_path(path: impl AsRef<std::path::Path>) -> ir::QualifiedName {
        ir::Identifier::no_ref(
            &path
                .as_ref()
                .file_stem()
                .expect("illegal file name")
                .to_string_lossy(),
        )
        .into()
    }
}

impl Lower for ir::Source {
    type AstNode = ast::Program;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Source::new(
            None, // todo
            ir::StatementList::lower(&node.statements, context)?,
            Hashed::new(context.source.to_string()),
            microcad_lang_base::virtual_url("name"),
        ))
    }
}
