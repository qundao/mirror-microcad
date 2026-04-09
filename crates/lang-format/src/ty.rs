// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Format, FormatConfig, node::Node};

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
        self.name.to_string().into()
    }
}

impl Format for ast::ArrayType {
    fn format(&self, _: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::TupleType {
    fn format(&self, _: &FormatConfig) -> Node {
        todo!()
    }
}
