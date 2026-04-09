// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Format, FormatConfig, Node, format_extra_trailing, node};

use microcad_syntax::ast::{self, ItemExtra, Visibility};

impl Format for Option<ast::Visibility> {
    fn format(&self, _: &FormatConfig) -> Node {
        match &self {
            Some(Visibility::Public) => "pub ".into(),
            _ => Node::Nil,
        }
    }
}

impl Format for ast::Parameter {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            match (&self.ty, &self.default) {
                (None, None) => self.name.format(f),
                (None, Some(def)) => node!(f => self.name " = " def),
                (Some(ty), None) => node!(f => self.name ": " ty),
                (Some(ty), Some(def)) => node!(f => self.name ": " ty " = " def),
            }
        )
    }
}

impl Format for ast::ParameterList {
    fn format(&self, f: &FormatConfig) -> Node {
        let nodes: Vec<Node> = self.parameters.iter().map(|item| item.format(f)).collect();
        let width: usize = nodes.iter().map(|node| node.estimate_width()).sum();
        let can_break = self.parameters.len() > 4
            || width > f.max_width
            || nodes.iter().any(|node| node.contains_hardline());

        node!(f, self.extras =>
            '(' Node::list(nodes, ',', can_break, f.indent_width) ')'
        )
    }
}

impl Format for ast::WorkbenchKind {
    fn format(&self, _: &FormatConfig) -> Node {
        self.to_string().into()
    }
}

impl Format for ast::WorkbenchDefinition {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.doc
            self.attributes
            self.visibility self.kind ' ' self.name
            self.plan ' '
            self.body
        )
    }
}

impl Format for ast::ModuleDefinition {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.doc
            self.attributes
            self.visibility "mod " self.name
            match &self.body {
                Some(body) => node!(f => ' ' body),
                None => node!(';')
            }
        )
    }
}

impl Format for ast::FunctionDefinition {
    fn format(&self, f: &FormatConfig) -> Node {
        let return_type = match &self.return_type {
            Some(ty) => node!(f => "-> " ty " "),
            None => Node::Nil,
        };

        node!(f, self.extras =>
            self.doc
            self.attributes
            self.visibility "fn " self.name self.parameters " " return_type
            self.body
        )
    }
}

impl Format for ast::UseStatementPart {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::UseStatementPart::Identifier(identifier) => identifier.format(f),
            ast::UseStatementPart::Glob(_) => '*'.into(),
            ast::UseStatementPart::Error(_) => Node::Nil,
        }
    }
}

impl Format for ast::UseName {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            Node::hlist(self.parts.iter().map(|part| part.format(f)), "::")
        )
    }
}

impl Format for ast::UseStatement {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.visibility "use " self.name
            self.use_as.as_ref().map(|ident| node!(f => " as " ident))
        )
    }
}

impl Format for ast::ConstAssignment {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.visibility "const " self.name " = " self.value
        )
    }
}

impl Format for ast::InitDefinition {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.doc
            self.attributes
            "init" self.parameters " " self.body
        )
    }
}

impl Format for ast::Return {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            "return"
            match &self.value {
                Some(value) => node!(f => ' ' value),
                None => Node::Nil
            }
        )
    }
}

impl Format for ast::LocalAssignment {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.name
            match &self.ty {
                Some(ty) => node!(f => ": " ty),
                None => Node::Nil,
            }
            " = " self.value
        )
    }
}

impl Format for ast::PropertyAssignment {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.doc
            self.attributes
            "prop " self.name
            self.ty.as_ref().map(|ty| node!(f => ": " ty))
            " = " self.value
        )
    }
}

impl Format for ast::ExpressionStatement {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.attributes
            self.expression
        )
    }
}

impl Format for ast::AttributeCommand {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::AttributeCommand::Ident(identifier) => identifier.format(f),
            ast::AttributeCommand::Assignment(local_assignment) => local_assignment.format(f),
            ast::AttributeCommand::Call(call) => call.format(f),
        }
    }
}

impl Format for ast::Attribute {
    fn format(&self, f: &FormatConfig) -> Node {
        let (prefix, suffix) = if self.is_inner {
            ("#![", node!(']' Node::Hardline))
        } else {
            ("#[", node!(']'))
        };

        let nodes: Vec<Node> = self.commands.iter().map(|attr| attr.format(f)).collect();
        let width: usize = nodes.iter().map(|node| node.estimate_width()).sum();
        let can_break = width > f.max_width || nodes.iter().any(|node| node.contains_hardline());

        node!(f, self.extras =>
            prefix Node::list(nodes, ',', can_break, 0) suffix
        )
    }
}

impl Format for Vec<ast::Attribute> {
    fn format(&self, f: &FormatConfig) -> Node {
        if self.is_empty() {
            Node::Nil
        } else {
            Node::vlist(self.iter().map(|attr| attr.format(f)), Node::Nil, 0)
        }
    }
}

impl Format for ast::Statement {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::Statement::Workbench(workbench_definition) => workbench_definition.format(f),
            ast::Statement::Module(module_definition) => module_definition.format(f),
            ast::Statement::Function(function_definition) => function_definition.format(f),
            ast::Statement::InnerDocComment(comment) => comment.format(f),
            ast::Statement::Use(use_statement) => use_statement.format(f),
            ast::Statement::Const(const_assignment) => const_assignment.format(f),
            ast::Statement::Init(init_definition) => init_definition.format(f),
            ast::Statement::Return(r) => r.format(f),
            ast::Statement::InnerAttribute(attribute) => attribute.format(f),
            ast::Statement::LocalAssignment(local_assignment) => local_assignment.format(f),
            ast::Statement::Property(property_assignment) => property_assignment.format(f),
            ast::Statement::Expression(expression_statement) => expression_statement.format(f),
            ast::Statement::Error(_) => Node::Nil,
        }
    }
}

impl Format for Vec<(ast::Statement, Vec<ItemExtra>)> {
    fn format(&self, f: &FormatConfig) -> Node {
        // Join statements with a hardline so they sit on separate lines
        self.iter()
            .map(
                |(statement, extras)| match statement.ends_with_semicolon() {
                    true => {
                        node!(
                            statement.format(f) ";"
                            if extras.iter().any(|extra| matches!(extra, ast::ItemExtra::Comment(_))) { " " } else { "" }
                            format_extra_trailing(extras, f)
                        )
                    }
                    false => node!(
                        statement.format(f)
                        Node::Hardline
                        format_extra_trailing(extras, f)
                    ),
                },
            )
            .collect::<Vec<Node>>()
            .into()
    }
}

impl Format for ast::StatementList {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            match (self.statements.is_empty(), &self.tail) {
                (true, Some(tail)) => tail.format(f),
                (false, Some(tail)) => {
                    node!(
                        self.statements.format(f)
                        tail.format(f) Node::Hardline
                    )
                }
                (false, None) => self.statements.format(f),
                (true, None) => Node::Nil,
            }
        )
    }
}
