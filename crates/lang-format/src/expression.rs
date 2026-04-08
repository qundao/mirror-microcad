// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Format, FormatConfig, node::Node};

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

impl Format for ast::Expression {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::Expression::Literal(literal) => literal.format(f),
            ast::Expression::Bracketed(_bracket, _) => todo!(),
            ast::Expression::Tuple(tuple_expression) => tuple_expression.format(f),
            ast::Expression::ArrayRange(array_range_expression) => array_range_expression.format(f),
            ast::Expression::ArrayList(array_list_expression) => array_list_expression.format(f),
            ast::Expression::String(format_string) => format_string.format(f),
            ast::Expression::QualifiedName(qualified_name) => qualified_name.format(f),
            ast::Expression::Marker(identifier) => format!("@{}", identifier.name).into(),
            ast::Expression::BinaryOperation(binary_operation) => binary_operation.format(f),
            ast::Expression::UnaryOperation(unary_operation) => unary_operation.format(f),
            ast::Expression::Block(_body) => todo!(),
            ast::Expression::Call(call) => call.format(f),
            ast::Expression::ElementAccess(element_access) => element_access.format(f),
            ast::Expression::If(i) => i.format(f),
            ast::Expression::Error(_) => Node::Nil,
        }
    }
}

impl Format for ast::StringPart {
    fn format(&self, _: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::StringCharacter {
    fn format<'a>(&self, _: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::StringExpression {
    fn format<'a>(&self, _: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::StringFormatSpecification {
    fn format<'a>(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::FormatString {
    fn format(&self, _: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::TupleItem {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::TupleExpression {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::ArrayItem {
    fn format(&self, f: &FormatConfig) -> Node {
        self.expression.format(f)
    }
}

impl Format for ast::ArrayRangeExpression {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::ArrayListExpression {
    fn format(&self, f: &FormatConfig) -> Node {
        let nodes: Vec<Node> = self
            .items
            .iter()
            .map(|item| item.expression.format(f))
            .collect();
        let width: usize = nodes.iter().map(|node| node.estimate_width()).sum();
        if width > f.max_width {
            vec![
                "[".into(),
                Node::Hardline,
                Node::Indent {
                    width: f.indent_width,
                    node: Box::new(
                        nodes
                            .into_iter()
                            .flat_map(|node| vec![node, ",".into(), Node::Hardline])
                            .collect::<Vec<_>>()
                            .into(),
                    ),
                },
                "]".into(),
            ]
            .into()
        } else {
            vec!["[".into(), Node::interspersed(nodes, ", "), "]".into()].into()
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
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::UnaryOperation {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
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
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::NamedArgument {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::ArgumentList {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::Call {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
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
