// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{BreakMode, Format, FormatConfig, Node, extras::leading_extras_without_newline, node};

use microcad_syntax::ast;

impl Format for Option<ast::Visibility> {
    fn format(&self, _: &FormatConfig) -> Node {
        match &self {
            Some(ast::Visibility::Public) => "pub ".into(),
            _ => Node::Nil,
        }
    }
}

impl Format for ast::Parameter {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, leading_extras_without_newline(&self.extras) =>
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
        let break_mode = BreakMode::from_layout(&nodes, 4, f);

        node!(f, leading_extras_without_newline(&self.extras) =>
            '(' Node::list(nodes, ',', break_mode) ')'
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

impl Format for ast::InlineModule {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.doc
            self.attributes
            self.visibility "mod " self.name ' '
            self.body
        )
    }
}

impl Format for ast::FileModule {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.doc
            self.attributes
            self.visibility "mod " self.name
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
            self.attributes
            self.visibility "use " self.name
            self.use_as.as_ref().map(|ident| node!(f => " as " ident))
        )
    }
}

impl Format for ast::ConstAssignment {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.doc
            self.attributes
            self.visibility "const " self.name " = " self.value
        )
    }
}

impl Format for ast::InitDefinition {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.doc
            self.attributes
            "init" self.parameters Node::Softline self.body
        )
    }
}

impl Format for ast::Return {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            "return"
            match &self.value {
                Some(value) => node!(f => Node::Softline value),
                None => Node::Nil
            }
        )
    }
}

impl Format for ast::LocalAssignment {
    fn format(&self, f: &FormatConfig) -> Node {
        let assignment = node!(f =>
            self.name
            match &self.ty {
                Some(ty) => node!(f => ':' Node::Softline ty),
                None => Node::Nil,
            }
            Node::Softline '=' Node::Softline
        );

        node!(f, self.extras =>
            self.attributes
            assignment Node::AdditionalIndent(assignment.estimate_width()) self.value
        )
    }
}

impl Format for ast::PropertyAssignment {
    fn format(&self, f: &FormatConfig) -> Node {
        node!(f, self.extras =>
            self.doc
            self.attributes
            "prop" Node::Softline self.name
            self.ty.as_ref().map(|ty| node!(f => ':' Node::Softline ty))
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

impl Format for ast::InnerDocComment {
    fn format(&self, _: &FormatConfig) -> Node {
        self.line.clone().into()
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
        let prefix = if self.is_inner { "#![" } else { "#[" };

        let nodes: Vec<Node> = self.commands.iter().map(|attr| attr.format(f)).collect();
        let break_mode = BreakMode::from_layout(&nodes, 0, f);

        node!(f, self.extras =>
            prefix Node::list(nodes, ',', break_mode) ']'
        )
    }
}

impl Format for Vec<ast::Attribute> {
    fn format(&self, f: &FormatConfig) -> Node {
        if self.is_empty() {
            Node::Nil
        } else {
            self.iter()
                .map(|attr| {
                    let node = attr.format(f);
                    if node.contains_hardline() {
                        node
                    } else {
                        node!(node Node::Hardline)
                    }
                })
                .collect::<Vec<_>>()
                .into()
        }
    }
}

impl Format for ast::Statement {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::Statement::Workbench(workbench_definition) => workbench_definition.format(f),
            ast::Statement::InlineModule(inline_module) => inline_module.format(f),
            ast::Statement::FileModule(file_module) => file_module.format(f),
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

impl Format for Vec<(ast::Statement, ast::TrailingExtras)> {
    fn format(&self, f: &FormatConfig) -> Node {
        // Join statements with a hardline so they sit on separate lines
        self.iter()
            .map(|(statement, extras)| {
                node!(f =>
                    statement
                    if statement.ends_with_semicolon() { node!(';' Node::Softline) } else { Node::Nil }
                    extras
                    Node::AdditionalIndent(0)
                )
            })
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
                    node!(f =>
                        self.statements
                        tail
                    )
                }
                (false, None) => self.statements.format(f),
                (true, None) => Node::Nil,
            }
        )
    }
}
