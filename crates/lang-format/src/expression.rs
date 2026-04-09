// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Format, FormatConfig, Node, node};

use microcad_syntax::ast;

impl Format for ast::Operator {
    fn format(&self, _: &FormatConfig) -> Node {
        self.operation.as_str().into()
    }
}

impl Format for ast::UnaryOperator {
    fn format(&self, _: &FormatConfig) -> Node {
        self.operation.as_str().into()
    }
}

pub(crate) fn format_body(body: &ast::StatementList, f: &FormatConfig) -> Node {
    match (body.statements.is_empty(), &body.tail) {
        (true, Some(tail)) => node!("{ " tail.format(f) " }"),
        (true, None) => node!("{}"),
        _ => node!(
            "{" Node::Hardline
                Node::indent(f.indent_width, body.format(f))
            "}"
        ),
    }
}

impl Format for ast::Expression {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::Expression::Literal(literal) => literal.format(f),
            ast::Expression::Bracketed(bracket, _) => node!('(' bracket.format(f) ')'),
            ast::Expression::Tuple(tuple_expression) => tuple_expression.format(f),
            ast::Expression::ArrayRange(array_range_expression) => array_range_expression.format(f),
            ast::Expression::ArrayList(array_list_expression) => array_list_expression.format(f),
            ast::Expression::String(format_string) => format_string.format(f),
            ast::Expression::QualifiedName(qualified_name) => qualified_name.format(f),
            ast::Expression::Marker(identifier) => format!("@{}", identifier.name).into(),
            ast::Expression::BinaryOperation(binary_operation) => binary_operation.format(f),
            ast::Expression::UnaryOperation(unary_operation) => unary_operation.format(f),
            ast::Expression::Block(body) => format_body(body, f),
            ast::Expression::Call(call) => call.format(f),
            ast::Expression::ElementAccess(element_access) => element_access.format(f),
            ast::Expression::If(i) => i.format(f),
            ast::Expression::Error(_) => Node::Nil,
        }
    }
}

impl Format for ast::StringPart {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::StringPart::Char(string_character) => string_character.format(f),
            ast::StringPart::Content(string_literal) => string_literal.format(f),
            ast::StringPart::Expression(string_expression) => string_expression.format(f),
        }
    }
}

impl Format for ast::StringCharacter {
    fn format<'a>(&self, _: &FormatConfig) -> Node {
        self.character.into()
    }
}

impl Format for ast::StringExpression {
    fn format<'a>(&self, f: &FormatConfig) -> Node {
        node!(f => '{' self.specification self.expression '}')
    }
}

impl Format for ast::StringFormatSpecification {
    fn format<'a>(&self, _f: &FormatConfig) -> Node {
        match (&self.precision, &self.width) {
            (Some(Ok(width)), Some(Ok(precision))) => format!("0{width}.{precision}").into(),
            (None, Some(Ok(precision))) => format!(".{precision}").into(),
            (Some(Ok(width)), None) => format!("0{width}").into(),
            _ => Node::Nil,
        }
    }
}

impl Format for ast::FormatString {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(
            '"'
            self.parts
                .iter()
                .map(|part| part.format(f))
                .collect::<Vec<_>>()
            '"'
        )
    }
}

impl Format for ast::TupleItem {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self.name {
            Some(name) => node!(f => name " = " self.value),
            None => self.value.format(f),
        }
    }
}

impl Format for ast::TupleExpression {
    fn format(&self, f: &FormatConfig) -> Node {
        let nodes: Vec<Node> = self.values.iter().map(|item| item.format(f)).collect();
        let width: usize = nodes.iter().map(|node| node.estimate_width()).sum();
        let can_break = self.values.len() > 4
            || width > f.max_width
            || nodes.iter().any(|node| node.contains_hardline());

        Node::braces(Node::list(nodes, ',', can_break), f.indent_width, can_break)
    }
}

impl Format for ast::ArrayItem {
    fn format(&self, f: &FormatConfig) -> Node {
        self.expression.format(f)
    }
}

impl Format for ast::ArrayRangeExpression {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f => '[' self.start ".." self.end ']')
    }
}

impl Format for ast::ArrayListExpression {
    fn format(&self, f: &FormatConfig) -> Node {
        let nodes: Vec<Node> = self.items.iter().map(|item| item.format(f)).collect();
        let width: usize = nodes.iter().map(|node| node.estimate_width()).sum();
        let can_break = width > f.max_width || nodes.iter().any(|node| node.contains_hardline());

        if can_break {
            node!(
                "[" Node::Hardline
                    Node::indent(
                        f.indent_width,
                        Node::hlist(nodes, node!("," Node::Hardline))
                    ) "," Node::Hardline
                "]"
            )
        } else {
            node!("[" Node::hlist(nodes, ", ") "]")
        }
    }
}

impl Format for ast::QualifiedName {
    fn format(&self, _: &FormatConfig) -> Node {
        self.parts
            .iter()
            .map(|identifier| identifier.name.clone())
            .collect::<Vec<_>>()
            .join("::")
            .into()
    }
}

impl Format for ast::BinaryOperation {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f => self.lhs ' ' self.operation ' ' self.rhs)
    }
}

impl Format for ast::UnaryOperation {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f => self.operation self.rhs)
    }
}

impl Format for ast::Argument {
    fn format(&self, f: &FormatConfig) -> Node {
        match self {
            ast::Argument::Unnamed(arg) => arg.format(f),
            ast::Argument::Named(arg) => arg.format(f),
        }
    }
}

impl Format for ast::UnnamedArgument {
    fn format(&self, f: &FormatConfig) -> Node {
        self.value.format(f)
    }
}

impl Format for ast::NamedArgument {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f => self.name " = " self.value)
    }
}

impl Format for ast::ArgumentList {
    fn format(&self, f: &FormatConfig) -> Node {
        let nodes: Vec<Node> = self.arguments.iter().map(|item| item.format(f)).collect();
        let width: usize = nodes.iter().map(|node| node.estimate_width()).sum();
        let can_break = self.arguments.len() > 4
            || width > f.max_width
            || nodes.iter().any(|node| node.contains_hardline());

        Node::braces(Node::list(nodes, ',', can_break), f.indent_width, can_break)
    }
}

impl Format for ast::Call {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(
            f =>
            self.name
            self.arguments
        )
    }
}

impl Format for ast::Element {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for Vec<ast::Element> {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::ElementAccess {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::If {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}
