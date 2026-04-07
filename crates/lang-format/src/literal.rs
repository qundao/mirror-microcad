// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{DocAllocator, DocBuilder, Format, Formatter, format_with_extras};

use microcad_syntax::ast;

impl Format for ast::Literal {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        format_with_extras(self.literal.format(f), &self.extras, f)
    }
}

impl Format for ast::LiteralKind {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        match self {
            ast::LiteralKind::String(lit) => lit.format(f),
            ast::LiteralKind::Bool(lit) => lit.format(f),
            ast::LiteralKind::Integer(lit) => lit.format(f),
            ast::LiteralKind::Float(lit) => lit.format(f),
            ast::LiteralKind::Quantity(lit) => lit.format(f),
            ast::LiteralKind::Error(_) => f.arena.nil(),
        }
    }
}

impl Format for ast::StringLiteral {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        a.text("\"")
            .append(a.text(self.content.clone()))
            .append(a.text("\""))
    }
}

impl Format for ast::BoolLiteral {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        f.arena.text(if self.value { "true" } else { "false" })
    }
}

impl Format for ast::IntegerLiteral {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        f.arena.text(self.raw.clone())
    }
}

impl Format for ast::FloatLiteral {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        f.arena.text(self.raw.clone())
    }
}

impl Format for ast::QuantityLiteral {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        // We concatenate value and unit without a space or softline
        // to ensure they are treated as a single "atom" by the layout engine.
        a.text(self.raw.clone()).append(self.ty.format(f)).group()
    }
}
