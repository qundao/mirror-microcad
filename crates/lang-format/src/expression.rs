// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    DocAllocator, DocBuilder, Format, Formatter, format_assignment, format_body, format_with_extras,
};

use microcad_syntax::ast;

impl Format for ast::Operator {
    fn format<'a>(&self, formatter: &Formatter<'a>) -> DocBuilder<'a> {
        formatter.arena.text(self.operation.as_str())
    }
}

impl Format for ast::UnaryOperator {
    fn format<'a>(&self, formatter: &Formatter<'a>) -> DocBuilder<'a> {
        formatter.arena.text(self.operation.as_str())
    }
}

impl Format for ast::Expression {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        match &self {
            ast::Expression::Literal(literal) => literal.format(f),
            ast::Expression::Bracketed(bracket, _) => bracket.format(f).parens(),
            ast::Expression::Tuple(tuple_expression) => tuple_expression.format(f),
            ast::Expression::ArrayRange(array_range_expression) => array_range_expression.format(f),
            ast::Expression::ArrayList(array_list_expression) => array_list_expression.format(f),
            ast::Expression::String(format_string) => format_string.format(f),
            ast::Expression::QualifiedName(qualified_name) => qualified_name.format(f),
            ast::Expression::Marker(identifier) => a.text(format!("@{}", identifier.name)),
            ast::Expression::BinaryOperation(binary_operation) => binary_operation.format(f),
            ast::Expression::UnaryOperation(unary_operation) => unary_operation.format(f),
            ast::Expression::Block(body) => format_body(body, f),
            ast::Expression::Call(call) => call.format(f),
            ast::Expression::ElementAccess(element_access) => element_access.format(f),
            ast::Expression::If(i) => i.format(f),
            ast::Expression::Error(_) => a.nil(),
        }
    }
}

impl Format for ast::StringPart {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        match &self {
            ast::StringPart::Char(s) => s.format(f),
            ast::StringPart::Content(s) => s.format(f),
            ast::StringPart::Expression(s) => s.format(f),
        }
    }
}

impl Format for ast::StringCharacter {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        f.arena.text(self.character.to_string())
    }
}

impl Format for ast::StringExpression {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        format_with_extras(
            self.specification
                .format(f)
                .append(self.expression.format(f)),
            &self.extras,
            f,
        )
    }
}

impl Format for ast::StringFormatSpecification {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        match (&self.width, &self.precision) {
            (Some(Ok(width)), Some(Ok(precision))) => a.text(format!("0{width}.{precision}:")),
            (None, Some(Ok(precision))) => a.text(format!(".{precision}:")),
            (Some(Ok(width)), None) => a.text(format!("0{width}")),
            _ => a.nil(),
        }
    }
}

impl Format for ast::FormatString {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let doc = self
            .parts
            .iter()
            .fold(a.nil(), |acc, part| acc.append(part.format(f)));
        format_with_extras(doc, &self.extras, f)
    }
}

impl Format for ast::TupleItem {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        format_with_extras(
            match &self.name {
                Some(name) => name
                    .format(f)
                    .append(a.space())
                    .append(a.text("="))
                    .append(a.space())
                    .append(self.value.format(f)),
                None => self.value.format(f),
            },
            &self.extras,
            f,
        )
    }
}

impl Format for ast::TupleExpression {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let items = self.values.iter().map(|item| item.format(f));
        let tuple_doc = a.intersperse(items, a.text(",")).parens();

        format_with_extras(tuple_doc, &self.extras, f)
    }
}

impl Format for ast::ArrayItem {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        format_with_extras(self.expression.format(f), &self.extras, f)
    }
}

impl Format for ast::ArrayRangeExpression {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let inner_parts = vec![self.start.format(f), a.text(".."), self.end.format(f)];

        let range_doc = a.intersperse(inner_parts, a.softline_()).brackets().group();

        format_with_extras(range_doc, &self.extras, f)
    }
}

impl Format for ast::ArrayListExpression {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let items = self.items.iter().map(|item| item.format(f));
        let array_doc = a.intersperse(items, a.text(",")).brackets().group();

        format_with_extras(array_doc, &self.extras, f)
    }
}

impl Format for ast::QualifiedName {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let name_doc = a.intersperse(self.parts.iter().map(|part| part.format(f)), a.text("::"));
        format_with_extras(name_doc, &self.extras, f)
    }
}

impl Format for ast::BinaryOperation {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        self.lhs
            .format(f)
            .append(a.space())
            .append(self.operation.format(f))
            .append(a.softline().append(self.rhs.format(f)).nest(4))
            .group()
    }
}

impl Format for ast::UnaryOperation {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        // 3. Wrap with leading/trailing extras (comments)
        format_with_extras(
            self.operation.format(f).append(self.rhs.format(f)),
            &self.extras,
            f,
        )
    }
}

impl Format for ast::Argument {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        match self {
            ast::Argument::Unnamed(arg) => arg.format(f),
            ast::Argument::Named(arg) => arg.format(f),
        }
    }
}

impl Format for ast::UnnamedArgument {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        // Unnamed arguments are just the expression itself
        let doc = self.value.format(f);
        format_with_extras(doc, &self.extras, f)
    }
}

impl Format for ast::NamedArgument {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        format_with_extras(
            format_assignment(&self.name, &None, Some(&self.value), f),
            &self.extras,
            f,
        )
    }
}

impl Format for ast::ArgumentList {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;

        let args = self.arguments.iter().map(|arg| arg.format(f));
        let args = a.intersperse(args, a.text(",").append(a.softline()));
        format_with_extras(args, &self.extras, f)
    }
}

impl Format for ast::Call {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let call_doc = self
            .name
            .format(f)
            .append(self.arguments.format(f).parens())
            .group();

        format_with_extras(call_doc, &self.extras, f)
    }
}

impl Format for ast::Element {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        match &self {
            ast::Element::Attribute(identifier) => a.text("#").append(identifier.format(f)),
            ast::Element::Tuple(identifier) => a.text(".").append(identifier.format(f)),
            ast::Element::Method(call) => a.text(".").append(call.format(f)),
            ast::Element::ArrayElement(expression) => a
                .text("[")
                .append(a.softline().append(expression.format(f)).nest(4))
                .append(expression.format(f))
                .append(a.softline_().append("]"))
                .group(),
        }
    }
}

impl Format for Vec<ast::Element> {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        a.intersperse(self.iter().map(|e| e.format(f)), a.hardline())
    }
}

impl Format for ast::ElementAccess {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        self.value
            .format(f)
            .append(a.softline_())
            .append(self.element_chain.format(f))
    }
}

impl Format for ast::If {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;

        let doc = a
            .text("if")
            .append(a.space())
            .append(self.condition.format(f))
            .append(a.space())
            .append(format_body(&self.body, f));

        format_with_extras(
            if let Some(next_if) = &self.next_if {
                doc.append(a.space())
                    .append(a.text("else"))
                    .append(a.space())
                    .append(next_if.format(f))
            } else if let Some(else_body) = &self.else_body {
                doc.append(a.space())
                    .append(a.text("else"))
                    .append(a.space())
                    .append(format_body(else_body, f))
            } else {
                doc
            },
            &self.extras,
            f,
        )
    }
}
