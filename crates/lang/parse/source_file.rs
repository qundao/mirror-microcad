// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, tree_display::*};
use microcad_syntax::ast;
use std::fs::read_to_string;

impl SourceFile {
    /// Load µcad source file from given `path`
    pub fn load(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
    ) -> Result<Rc<Self>, ParseErrorWithSource> {
        Self::load_with_name(&path, Self::name_from_path(&path))
    }

    /// Load µcad source file from given `path`
    pub fn load_with_name(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
        name: QualifiedName,
    ) -> Result<Rc<Self>, ParseErrorWithSource> {
        let path = path.as_ref();
        log::trace!(
            "{load} file {path} [{name}]",
            path = path.display(),
            load = crate::mark!(LOAD)
        );

        let buf = read_to_string(path).map_err(|error| {
            ParseError::LoadSource(name.src_ref(), path.into(), error)
        })?;

        let mut source_file: Self = Parser::parse_rule(Rule::source_file, &buf, 0)
            .map_err(|error| error.with_source(buf))?;
        assert_ne!(source_file.hash, 0);
        source_file.set_filename(path);
        source_file.name = name;
        log::debug!(
            "Successfully loaded external file {} to {}",
            path.display(),
            source_file.name
        );
        log::trace!("Syntax tree:\n{}", FormatTree(&source_file));

        Ok(Rc::new(source_file))
    }

    /// Create `SourceFile` from string
    /// The hash of the result will be of `crate::from_str!()`.
    pub fn load_from_str(
        name: Option<&str>,
        path: impl AsRef<std::path::Path>,
        source_str: &str,
    ) -> ParseResult<Rc<Self>> {
        log::trace!("{load} source from string", load = crate::mark!(LOAD));
        let mut source_file: Self =
            Parser::parse_rule(crate::parser::Rule::source_file, source_str, 0)?;
        if let Some(name) = name {
            source_file.set_name(QualifiedName::from_id(Identifier::no_ref(name)));
        } else {
            source_file.set_name(Self::name_from_path(&path));
        };
        source_file.set_filename(path);
        log::debug!("Successfully loaded source from string");
        log::trace!("Syntax tree:\n{}", FormatTree(&source_file));
        Ok(Rc::new(source_file))
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
