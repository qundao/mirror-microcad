// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Format, FormatConfig, node::Node};

use microcad_syntax::ast::{self, Visibility};

impl Format for Option<ast::Visibility> {
    fn format(&self, _: &FormatConfig) -> Node {
        match &self {
            Some(Visibility::Public) => "pub ".into(),
            _ => Node::Nil,
        }
    }
}

impl Format for ast::Parameter {
    fn format(&self, _: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::ParameterList {
    fn format(&self, _: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::WorkbenchKind {
    fn format(&self, _: &FormatConfig) -> Node {
        self.to_string().into()
    }
}

impl Format for ast::WorkbenchDefinition {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::ModuleDefinition {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::FunctionDefinition {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::UseStatementPart {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::UseStatementPart::Identifier(identifier) => identifier.format(f),
            ast::UseStatementPart::Glob(_) => "*".into(),
            ast::UseStatementPart::Error(_) => Node::Nil,
        }
    }
}

impl Format for ast::UseName {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::UseStatement {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::ConstAssignment {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::InitDefinition {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::Return {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::LocalAssignment {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::PropertyAssignment {
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for ast::ExpressionStatement {
    fn format(&self, f: &FormatConfig) -> Node {
        //vec![self.attributes.format(f), self.expression.format(f)].into()
        self.expression.format(f)
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
    fn format(&self, _f: &FormatConfig) -> Node {
        todo!()
    }
}

impl Format for Vec<ast::Attribute> {
    fn format(&self, f: &FormatConfig) -> Node {
        if self.is_empty() {
            Node::Nil
        } else {
            Node::interspersed(self.iter().map(|attr| attr.format(f)), Node::Hardline)
        }
    }
}

impl Format for ast::Statement {
    fn format(&self, f: &FormatConfig) -> Node {
        match &self {
            ast::Statement::Workbench(workbench_definition) => workbench_definition.format(f),
            ast::Statement::Module(module_definition) => module_definition.format(f),
            ast::Statement::Function(function_definition) => function_definition.format(f),
            ast::Statement::InnerDocComment(comment) => todo!(),
            ast::Statement::Comment(comment) => todo!(),

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

impl Format for Vec<(ast::Statement, Option<String>)> {
    fn format(&self, f: &FormatConfig) -> Node {
        // Join statements with a hardline so they sit on separate lines
        self.iter()
            .flat_map(
                |(statement, whitespace)| match statement.ends_with_semicolon() {
                    true => {
                        let whitespace = whitespace.as_ref().cloned().unwrap_or_default();
                        let newline_count = whitespace.chars().filter(|&c| c == '\n').count();
                        vec![
                            statement.format(f),
                            ";".into(),
                            if newline_count < 2 {
                                Node::Nil
                            } else {
                                Node::Hardline
                            },
                            Node::Hardline,
                        ]
                        .into()
                    }
                    false => vec![statement.format(f), Node::Hardline],
                },
            )
            .collect::<Vec<Node>>()
            .into()
    }
}

impl Format for ast::StatementList {
    fn format(&self, f: &FormatConfig) -> Node {
        self.statements.format(f)
    }
}
