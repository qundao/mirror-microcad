// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{DocAllocator, DocBuilder, Format, Formatter};

use microcad_syntax::ast;

impl Format for ast::Type {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        match &self {
            ast::Type::Single(single_type) => single_type.format(f),
            ast::Type::Array(array_type) => array_type.format(f),
            ast::Type::Tuple(tuple_type) => tuple_type.format(f),
        }
    }
}

impl Format for ast::SingleType {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        f.arena.as_string(self.name.to_string())
    }
}

impl Format for ast::ArrayType {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        a.text("[")
            .append(self.inner.format(f))
            .append(a.text("]"))
            .group()
    }
}

impl Format for ast::TupleType {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        if self.inner.is_empty() {
            return a.text("()");
        }

        let inner = a.intersperse(
            self.inner.iter().map(|(id, ty)| match id {
                Some(id) => id
                    .format(f)
                    .append(a.text(": "))
                    .append(ty.format(f))
                    .group(),
                None => ty.format(f),
            }),
            a.text(",").append(a.softline()), // Break here if line is too long
        );

        // 3. Wrap in parentheses with nesting
        a.text("(")
            .append(a.softline().append(inner).nest(4))
            .append(a.softline())
            .append(a.text(")"))
            .group()
    }
}
