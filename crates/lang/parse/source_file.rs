// Copyright © 2024-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, tree_display::*};
use microcad_syntax::ast;
use std::fs::read_to_string;

impl SourceFile {
    /// Load µcad source file from given `path`
    pub fn load(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
    ) -> Result<Rc<Self>, ParseErrorsWithSource> {
        let (source, error) = Self::load_with_name(&path, Self::name_from_path(&path));
        match error {
            Some(error) => Err(error),
            None => Ok(source),
        }
    }

    /// Load µcad source file from given `path`
    pub fn load_with_name(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
        name: QualifiedName,
    ) -> (Rc<Self>, Option<ParseErrorsWithSource>) {
        let path = path.as_ref();
        log::trace!(
            "{load} file {path} [{name}]",
            path = path.display(),
            load = crate::mark!(LOAD)
        );

        let buf = match read_to_string(path) {
            Ok(buf) => buf,
            Err(error) => {
                let error = ParseError::LoadSource(name.src_ref(), path.into(), error);
                let mut source_file =
                    SourceFile::new(None, StatementList::default(), String::new(), 0);
                source_file.name = name;
                return (Rc::new(source_file), Some(error.into()));
            }
        };

        let (mut source_file, errors) = Self::load_inner(None, path, &buf);
        source_file.name = name;
        log::debug!(
            "Successfully loaded external file {} to {}",
            path.display(),
            source_file.name
        );

        (Rc::new(source_file), errors)
    }

    /// Create `SourceFile` from string
    /// The hash of the result will be of `crate::from_str!()`.
    pub fn load_from_str(
        name: Option<&str>,
        path: impl AsRef<std::path::Path>,
        source_str: &str,
    ) -> Result<Rc<Self>, Vec<ParseError>> {
        let (source, error) = Self::load_inner(name, path, source_str);
        match error {
            Some(error) => Err(error.errors),
            None => Ok(Rc::new(source)),
        }
    }

    fn load_inner(
        name: Option<&str>,
        path: impl AsRef<std::path::Path>,
        source_str: &str,
    ) -> (Self, Option<ParseErrorsWithSource>) {
        log::trace!("{load} source from string", load = crate::mark!(LOAD));
        let parse_context = ParseContext::new(source_str);

        let dummy_source = || {
            let mut source = SourceFile::new(
                None,
                StatementList::default(),
                source_str.into(),
                parse_context.source_file_hash,
            );
            source.filename = Some(path.as_ref().into());
            source
        };

        let ast = match build_ast(source_str, &parse_context) {
            Ok(ast) => ast,
            Err(error) => {
                return (dummy_source(), Some(error));
            }
        };

        let mut source_file =
            match Self::from_ast(&ast, &parse_context).map_err(|error| vec![error]) {
                Ok(source_file) => source_file,
                Err(errors) => {
                    return (
                        dummy_source(),
                        Some(ParseErrorsWithSource {
                            errors,
                            source_code: Some(source_str.into()),
                            source_hash: parse_context.source_file_hash,
                        }),
                    );
                }
            };
        if let Some(name) = name {
            source_file.set_name(QualifiedName::from_id(Identifier::no_ref(name)));
        } else {
            source_file.set_name(Self::name_from_path(&path));
        };
        source_file.set_filename(path);
        log::debug!("Successfully loaded source from string");
        log::trace!("Syntax tree:\n{}", FormatTree(&source_file));
        (source_file, None)
    }

    /// Get the source file name from path.
    pub fn name_from_path(path: impl AsRef<std::path::Path>) -> QualifiedName {
        QualifiedName::from_id(Identifier::no_ref(
            &path
                .as_ref()
                .file_stem()
                .expect("illegal file name")
                .to_string_lossy(),
        ))
    }
}

impl FromAst for SourceFile {
    type AstNode = ast::SourceFile;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(SourceFile::new(
            None, // todo
            StatementList::from_ast(&node.statements, context)?,
            context.source.into(),
            context.source_file_hash,
        ))
    }
}

#[test]
fn parse_source_file() {
    let source_file = SourceFile::load_from_str(
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
