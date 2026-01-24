// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, tree_display::*};
use microcad_syntax::{ast, parser, tokens};
use std::fs::read_to_string;

impl SourceFile {
    /// Load µcad source file from given `path`
    pub fn load(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
    ) -> Result<Rc<Self>, ParseErrorsWithSource> {
        Self::load_with_name(&path, Self::name_from_path(&path))
    }

    /// Load µcad source file from given `path`
    pub fn load_with_name(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
        name: QualifiedName,
    ) -> Result<Rc<Self>, ParseErrorsWithSource> {
        let path = path.as_ref();
        log::trace!(
            "{load} file {path} [{name}]",
            path = path.display(),
            load = crate::mark!(LOAD)
        );

        let buf = read_to_string(path).map_err(|error| {
            ParseError::LoadSource(name.src_ref(), path.into(), error)
        })?;

        let mut source_file = Self::load_inner(None, path, &buf)
            .map_err(|errors| ParseErrorsWithSource {
                errors,
                source_code: Some(buf)
            })?;
        source_file.name = name;
        log::debug!(
            "Successfully loaded external file {} to {}",
            path.display(),
            source_file.name
        );

        Ok(Rc::new(source_file))
    }

    /// Create `SourceFile` from string
    /// The hash of the result will be of `crate::from_str!()`.
    pub fn load_from_str(
        name: Option<&str>,
        path: impl AsRef<std::path::Path>,
        source_str: &str,
    ) -> Result<Rc<Self>, Vec<ParseError>> {
        Self::load_inner(name, path, source_str).map(Rc::new)
    }

    fn load_inner(
        name: Option<&str>,
        path: impl AsRef<std::path::Path>,
        source_str: &str,
    ) -> Result<Self, Vec<ParseError>> {
        log::trace!("{load} source from string", load = crate::mark!(LOAD));
        let parse_context = ParseContext::new(source_str);

        let tokens = tokens::lex(source_str)
            .map_err(|error| vec![ParseError::Lexer {
                src_ref: parse_context.src_ref(&error.span),
                error: error.token,
            }])?;
        let ast = parser::parse(tokens.as_slice())
            .map_err(|errors| errors.into_iter().map(|error| ParseError::AstParser {
                src_ref: parse_context.src_ref(&error.span),
                error
            }).collect::<Vec<_>>())?;

        let mut source_file = Self::from_ast(&ast, &parse_context).map_err(|error| vec![error])?;
        if let Some(name) = name {
            source_file.set_name(QualifiedName::from_id(Identifier::no_ref(name)));
        } else {
            source_file.set_name(Self::name_from_path(&path));
        };
        source_file.set_filename(path);
        log::debug!("Successfully loaded source from string");
        log::trace!("Syntax tree:\n{}", FormatTree(&source_file));
        Ok(source_file)
    }

    fn calculate_hash(value: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = rustc_hash::FxHasher::default();
        value.hash(&mut hasher);
        hasher.finish()
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

impl Parse for SourceFile {
    fn parse(mut pair: Pair) -> ParseResult<Self> {
        // calculate hash over complete file content
        let hash = Self::calculate_hash(pair.as_str());
        pair.set_source_hash(hash);

        Ok(SourceFile::new(
            crate::find_rule_opt!(pair, doc_block)?,
            crate::find_rule!(pair, statement_list)?,
            pair.as_span().as_str().to_string(),
            hash,
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
    let source_file = Parser::parse_rule::<SourceFile>(
        Rule::source_file,
        r#"use std::log::info;
            part Foo(r: Scalar) {
                info("Hello, world, {r}!");
            }
            Foo(20.0);
            "#,
        0,
    )
    .expect("test error");

    assert_eq!(source_file.statements.len(), 3);
}
