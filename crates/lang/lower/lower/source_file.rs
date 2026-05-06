// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, LowerErrorsWithSource, ir};

use microcad_lang_base::{FormatTree, Hashed, SrcReferrer};
use microcad_syntax::ast;

impl ir::SourceFile {
    pub fn from_source(source: &microcad_syntax::Source) -> Result<std::rc::Rc<Self>, LowerError> {
        let context = LowerContext::new(source.text.as_str());
        Ok(std::rc::Rc::new(Self {
            doc: None,
            statements: ir::StatementList::lower(&source.ast.statements, &context)?,
            source: source.text.clone(),
            name: ir::QualifiedName::default(),
            filename: source.url.to_file_path().ok(),
        }))
    }

    /// Load µcad source file from given `path`
    pub fn load(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
    ) -> Result<std::rc::Rc<Self>, LowerErrorsWithSource> {
        let (source, error) = Self::load_with_name(&path, Self::name_from_path(&path));
        match error {
            Some(error) => Err(error),
            None => Ok(source),
        }
    }

    /// Load µcad source file from given `path`
    pub fn load_with_name(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
        name: ir::QualifiedName,
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
                let mut source_file = ir::SourceFile::new(
                    None,
                    ir::StatementList::default(),
                    Hashed::new(String::new()),
                );
                source_file.name = name;
                return (std::rc::Rc::new(source_file), Some(error.into()));
            }
        };

        let (mut source_file, errors) = Self::load_inner(None, path, &buf);
        source_file.name = name;
        log::debug!(
            "Successfully loaded external file {} to {}",
            path.display(),
            source_file.name
        );

        (std::rc::Rc::new(source_file), errors)
    }

    /// Create `SourceFile` from string
    /// The hash of the result will be of `crate::from_str!()`.
    pub fn load_from_str(
        name: Option<&str>,
        path: impl AsRef<std::path::Path>,
        source_str: &str,
    ) -> Result<std::rc::Rc<Self>, Vec<LowerError>> {
        let (source, error) = Self::load_inner(name, path, source_str);
        match error {
            Some(error) => Err(error.errors),
            None => Ok(std::rc::Rc::new(source)),
        }
    }

    /// Create `SourceFile` from string, returning the empty `SourceFile` on error
    /// The hash of the result will be of `crate::from_str!()`.
    pub fn load_from_str_with_recovery(
        name: Option<&str>,
        path: impl AsRef<std::path::Path>,
        source_str: &str,
    ) -> (std::rc::Rc<Self>, Option<Vec<LowerError>>) {
        let (source, error) = Self::load_inner(name, path, source_str);
        (std::rc::Rc::new(source), error.map(|err| err.errors))
    }

    fn load_inner(
        name: Option<&str>,
        path: impl AsRef<std::path::Path>,
        source_str: &str,
    ) -> (Self, Option<LowerErrorsWithSource>) {
        log::trace!(
            "{load} source from string",
            load = microcad_lang_base::mark!(LOAD)
        );
        let parse_context = LowerContext::new(source_str);

        let dummy_source = || {
            let mut source = ir::SourceFile::new(
                None,
                ir::StatementList::default(),
                Hashed::new(source_str.into()),
            );
            source.filename = Some(path.as_ref().into());
            source
        };

        let ast = match crate::lower::lower::build_ast(source_str, &parse_context) {
            Ok(ast) => ast,
            Err(error) => {
                return (dummy_source(), Some(error));
            }
        };

        let mut source_file =
            match Self::lower(&ast, &parse_context).map_err(|error| vec![error]) {
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
        log::trace!("Syntax tree:\n{}", FormatTree(&source_file));
        (source_file, None)
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

impl Lower for ir::SourceFile {
    type AstNode = ast::Program;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::SourceFile::new(
            None, // todo
            ir::StatementList::lower(&node.statements, context)?,
            Hashed::new(context.source.to_string()),
        ))
    }
}

#[test]
fn parse_source_file() {
    let source_file = ir::SourceFile::load_from_str(
        None,
        "test.µcad",
        r#"use std::log::info;
            part Foo(r: Scalar) {
                info("Hello, world, {r}!");
            }
            Foo(20.0);
            "#,
    )
    .expect("test error");

    assert_eq!(source_file.statements.len(), 3);
}
