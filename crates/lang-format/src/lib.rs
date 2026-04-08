// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;

use microcad_syntax::ast;

mod error;
mod expression;
mod literal;
mod node;
mod statement;
mod ty;

use crate::{error::FormatError, node::Node};

#[derive(Debug, Clone)]
pub struct FormatConfig {
    pub max_width: usize,
    pub indent_size: usize,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            max_width: 80,
            indent_size: 4,
        }
    }
}

pub(crate) trait Format {
    fn format(&self, f: &FormatConfig) -> Node;
}

impl Format for ast::Identifier {
    fn format(&self, _: &FormatConfig) -> Node {
        self.name.clone().into()
    }
}

/*
pub(crate) fn format_with_extras<'a>(
    doc: DocBuilder<'a>,
    extras: &ItemExtras,
    f: &FormatConfig<'a>,
) -> DocBuilder<'a> {
    fn format_leading<'a>(extras: &ItemExtras, f: &FormatConfig<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        if extras.leading.is_empty() {
            return a.nil();
        }

        let mut doc = a.nil();
        for comment in &extras.leading {
            // Append the comment and force a newline so the item starts below it
            doc = doc.append(comment.format(f)).append(a.hardline());
        }
        doc
    }

    fn format_trailing<'a>(extras: &ItemExtras, f: &FormatConfig<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        if extras.trailing.is_empty() {
            return a.nil();
        }

        let mut doc = a.nil();
        for comment in &extras.trailing {
            // Add a space so the comment doesn't touch the code: `10mm // comment`
            doc = doc.append(a.space()).append(comment.format(f));
        }
        doc
    }

    format_leading(extras, f)
        .append(doc)
        .append(format_trailing(extras, f))
}

pub(crate) fn format_symbol_outer<'a>(
    doc: &Option<ast::Comment>,
    attributes: &Vec<ast::Attribute>,
    f: &FormatConfig<'a>,
) -> DocBuilder<'a> {
    let a = f.arena;

    let doc = match doc {
        Some(doc) => doc.format(f).append(a.hardline()),
        None => a.nil(),
    };
    if attributes.is_empty() {
        doc
    } else {
        doc.append(attributes.format(f).append(a.hardline()))
    }
}

pub(crate) fn format_body<'a>(body: &ast::StatementList, f: &FormatConfig<'a>) -> DocBuilder<'a> {
    let a = f.arena;
    let statements = body.format(f);

    match (&body.statements.is_empty(), &body.tail) {
        (true, None) => a.nil().braces(),
        (true, Some(_)) => statements.braces().group(),
        _ => a
            .hardline()
            .append(statements)
            .nest(4)
            .append(a.hardline())
            .braces()
            .group(),
    }
}

pub(crate) fn format_assignment<'a>(
    name: &ast::Identifier,
    ty: &Option<ast::Type>,
    value: Option<&ast::Expression>,
    f: &FormatConfig<'a>,
) -> DocBuilder<'a> {
    let a = f.arena;
    name.format(f)
        .append(match ty {
            Some(ty) => a.text(":").append(a.space()).append(ty.format(f)),
            None => a.nil(),
        })
        .append(match value {
            Some(value) => a
                .space()
                .append("=")
                .append(a.space())
                .append(value.format(f)),
            None => a.nil(),
        })
}


impl Format for ast::Comment {
    fn format(&self, f: &FormatConfig) -> Node {
        let a = f.arena;
        match &self.inner {
            ast::CommentInner::SingleLine(items) => {
                let comment_lines = items.iter().map(|line| a.text(line.clone()));
                a.intersperse(comment_lines, a.hardline()) // `hardline` assures line break.
            }
            ast::CommentInner::MultiLine(line) => a.text(line.clone()).append(a.softline()),
        }
    }
}

impl Format for ast::ItemExtra {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::ItemExtra::Comment(comment) => comment.format(f),
            ast::ItemExtra::Whitespace(_) => f.arena.nil(),
            _ => todo!(),
        }
    }
}

*/

impl Format for ast::SourceFile {
    fn format(&self, f: &FormatConfig) -> Node {
        self.statements.format(f)
    }
}

/// Format µcad source file.
pub fn format(source_file: &ast::SourceFile, config: &FormatConfig) -> String {
    source_file.format(config).to_string()
}

/// High-level API to format a &str containing µcad source code.
pub fn format_str(source: &str, config: &FormatConfig) -> Result<String, FormatError> {
    let tokens: Vec<_> = microcad_syntax::lex(&source).collect();
    let source_file =
        microcad_syntax::parse(&tokens).map_err(|err| FormatError::ParseErrors(err))?;
    Ok(format(&source_file, &config))
}

/// High-level API to format an entire mdbook.
///
/// TODO: needs proper error handling.
pub fn format_mdbook(
    mdbook: &mut microcad_lang_markdown::MdBookDirectory,
    config: &FormatConfig,
) -> Result<(), FormatError> {
    let mut errors_by_file: HashMap<std::path::PathBuf, Vec<FormatError>> = HashMap::new();

    // 1. Iterate over code blocks. 'path' is the PathBuf of the .md file.
    mdbook.code_blocks_mut().for_each(|(path, code_block)| {
        if let Err(err) = format_str(&code_block.code, config) {
            errors_by_file.entry(path.clone()).or_default().push(
                FormatError::CodeBlockFormatError {
                    name: code_block.name().as_ref().cloned().unwrap_or_default(),
                    error: Box::new(err),
                },
            );
        } else if let Ok(formatted) = format_str(&code_block.code, config) {
            // Only update the code if formatting succeeded
            code_block.code = formatted;
        }
    });

    // 2. Persist the successfully formatted parts to disk
    mdbook.save_all()?;

    // 3. If we hit issues, return the map in the specific variant
    if !errors_by_file.is_empty() {
        return Err(FormatError::MdBookFormatError {
            src_path: mdbook.src_path.clone(),
            errors: errors_by_file,
        });
    }

    Ok(())
}
