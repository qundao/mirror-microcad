// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{BreakMode, Format, FormatConfig, Node, node};

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

impl Format for ast::Body {
    fn format(&self, f: &FormatConfig) -> Node {
        let body = &self.statements;

        match (body.statements.is_empty(), &body.tail) {
            (true, Some(tail)) => node!(f => "{ " tail " }"),
            (true, None) => node!("{}"),
            _ => node!(
                "{" Node::Hardline
                    Node::indent(f.indent_width, body.format(f))
                "}"
            ),
        }
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
            ast::Expression::Block(body) => body.format(f),
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
        node!(f, self.extras =>
            '{' self.specification self.expression '}'
        )
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
        node!(f, self.extras =>
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
        node!(f, self.extras =>
            match &self.name {
                Some(name) => node!(f => name " = " self.value),
                None => node!(f => self.value)
            }
        )
    }
}

impl Format for ast::TupleExpression {
    fn format(&self, f: &FormatConfig) -> Node {
        let nodes: Vec<Node> = self.values.iter().map(|item| item.format(f)).collect();
        let break_mode = BreakMode::from_layout(&nodes, 4, f);
        node!(f, self.extras =>
            '(' Node::list(nodes, ',', break_mode) ')'
        )
    }
}

impl Format for ast::ArrayItem {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras => self.expression)
    }
}

impl Format for ast::ArrayRangeExpression {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            '[' self.start ".." self.end ']'
        )
    }
}

impl Format for ast::ArrayListExpression {
    fn format(&self, f: &FormatConfig) -> Node {
        let nodes: Vec<Node> = self.items.iter().map(|item| item.format(f)).collect();
        let break_mode = BreakMode::from_layout(&nodes, 0, f);

        node!(f, self.extras =>
            '[' Node::list(nodes, ',', break_mode) ']'
        )
    }
}

impl Format for ast::QualifiedName {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            Node::hlist(self.parts.iter().map(|identifier| identifier.format(f)), "::")
        )
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
        node!(f, self.extras => self.value)
    }
}

impl Format for ast::NamedArgument {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras => self.name " = " self.value)
    }
}

impl Format for ast::ArgumentList {
    fn format(&self, f: &FormatConfig) -> Node {
        let nodes: Vec<Node> = self.arguments.iter().map(|item| item.format(f)).collect();
        let break_mode = BreakMode::from_layout(&nodes, 4, f);

        node!(f, self.extras => '(' Node::list(nodes, ',', break_mode) ')')
    }
}

impl Format for ast::Call {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras => self.name self.arguments)
    }
}

impl Format for ast::ElementInner {
    fn format(&self, f: &FormatConfig) -> Node {
        use ast::ElementInner::*;
        match &self {
            Attribute(identifier) => node!(f => '#' identifier),
            Tuple(identifier) => node!(f => '.' identifier),
            Method(call) => node!(f => '.' call),
            ArrayElement(expression) => node!(f => '[' expression ']'),
        }
    }
}

impl Format for ast::Element {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras => self.inner)
    }
}

impl Format for Vec<ast::Element> {
    fn format(&self, f: &FormatConfig) -> Node {
        fn element_line_break(f: &FormatConfig, element: &ast::Element, is_last: bool) -> Node {
            let has_hardline = element.format(f).ends_with_hardline();
            node!(f =>
                element
                if matches!(element.inner, ast::ElementInner::ArrayElement(_)) || is_last || has_hardline {
                    Node::Nil
                } else {
                    Node::Hardline
                }
            )
        }

        let nodes: Vec<Node> = self
            .iter()
            .enumerate()
            .map(|(i, element)| element_line_break(f, element, i >= self.len() - 1))
            .collect();
        let width: usize = nodes.iter().map(|node| node.estimate_width()).sum();
        let can_break = nodes.len() > 3
            || width > f.max_width
            || nodes.iter().any(|node| node.contains_hardline());
        if can_break {
            node!(
                Node::Hardline
                Node::indent(f.indent_width, nodes)
            )
        } else {
            Node::hlist(nodes, Node::Nil)
        }
    }
}

impl Format for ast::ElementAccess {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f => self.value self.element_chain)
    }
}

impl Format for ast::If {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            "if " self.condition ' ' self.body
            self.else_body.as_ref().map(|body| node!(f => " else " body))
            self.next_if.as_ref().map(|next_if| node!(f => " else " next_if))
        )
    }
}
