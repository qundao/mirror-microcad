// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_syntax::ast::{self, ItemExtras};

mod expression;
mod literal;
mod statement;
mod ty;

pub(crate) type DocBuilder<'a> = pretty::DocBuilder<'a, Arena<'a>>;
pub(crate) use pretty::{Arena, DocAllocator};

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

pub struct Formatter<'a> {
    arena: &'a Arena<'a>,
    config: FormatConfig,
}

pub(crate) trait Format {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a>;
}

pub(crate) fn format_with_extras<'a>(
    doc: DocBuilder<'a>,
    extras: &ItemExtras,
    f: &Formatter<'a>,
) -> DocBuilder<'a> {
    fn format_leading<'a>(extras: &ItemExtras, f: &Formatter<'a>) -> DocBuilder<'a> {
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

    fn format_trailing<'a>(extras: &ItemExtras, f: &Formatter<'a>) -> DocBuilder<'a> {
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
    f: &Formatter<'a>,
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

pub(crate) fn format_body<'a>(body: &ast::StatementList, f: &Formatter<'a>) -> DocBuilder<'a> {
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
    f: &Formatter<'a>,
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

impl Format for ast::Identifier {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        f.arena.text(self.name.clone())
    }
}

impl Format for ast::Comment {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
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
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        match &self {
            ast::ItemExtra::Comment(comment) => comment.format(f),
            ast::ItemExtra::Whitespace(_) => f.arena.nil(),
            _ => todo!(),
        }
    }
}

impl Format for ast::SourceFile {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        self.statements.format(f)
    }
}

/// Format µcad source file.
pub fn format(source_file: &ast::SourceFile, config: FormatConfig) -> String {
    let formatter = Formatter {
        arena: &Arena::new(),
        config,
    };

    source_file
        .format(&formatter)
        .pretty(formatter.config.max_width)
        .to_string()
}

/// High-level API to format a &str containing µcad source code.
pub fn format_str(
    source: &str,
    config: FormatConfig,
) -> Result<String, Vec<microcad_syntax::ParseError>> {
    let tokens: Vec<_> = microcad_syntax::lex(&source).collect();
    let source_file = microcad_syntax::parse(&tokens)?;
    Ok(format(&source_file, config))
}
