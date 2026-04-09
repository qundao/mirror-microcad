// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Format, FormatConfig, Node, node};

use microcad_syntax::ast;

impl Format for ast::Type {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::Type::Single(single_type) => single_type.format(f),
            ast::Type::Array(array_type) => array_type.format(f),
            ast::Type::Tuple(tuple_type) => tuple_type.format(f),
        }
    }
}

impl Format for ast::SingleType {
    fn format(&self, _: &FormatConfig) -> Node {
        self.name.clone().into()
    }
}

impl Format for ast::ArrayType {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f => '[' self.inner ']')
    }
}

impl Format for ast::TupleType {
    fn format(&self, f: &FormatConfig) -> Node {
        let nodes: Vec<Node> = self
            .inner
            .iter()
            .map(|item| match &item.0 {
                Some(name) => node!(f => name " = " item.1),
                None => item.1.format(f),
            })
            .collect();

        let width: usize = nodes.iter().map(|node| node.estimate_width()).sum();
        let can_break = self.inner.len() > 4
            || width > f.max_width
            || nodes.iter().any(|node| node.contains_hardline());

        node!('(' Node::list(nodes, ',', can_break, f.indent_width) ')')
    }
}
