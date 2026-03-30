// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    DocAllocator, DocBuilder, Format, Formatter, format_assignment, format_body,
    format_symbol_outer, format_with_extras,
};

use microcad_syntax::ast::{self, Visibility};

impl Format for Option<ast::Visibility> {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        match &self {
            Some(Visibility::Public) => a.text("pub").append(a.space()),
            _ => a.nil(),
        }
    }
}

impl Format for ast::Parameter {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        format_with_extras(
            format_assignment(&self.name, &self.ty, self.default.as_ref(), f),
            &self.extras,
            f,
        )
    }
}

impl Format for ast::ParameterList {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let parameters = a.intersperse(
            self.parameters.iter().map(|param| param.format(f)),
            a.text(",").append(a.space()),
        );

        format_with_extras(parameters.parens(), &self.extras, f)
    }
}

impl Format for ast::WorkbenchKind {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        f.arena.text(self.to_string())
    }
}

impl Format for ast::WorkbenchDefinition {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let doc = format_symbol_outer(&self.doc, &self.attributes, f)
            .append(self.visibility.format(f))
            .append(self.kind.format(f))
            .append(a.space())
            .append(self.name.format(f))
            .append(self.plan.format(f))
            .append(a.space())
            .append(format_body(&self.body, f));

        format_with_extras(doc, &self.extras, f).append(a.hardline())
    }
}

impl Format for ast::ModuleDefinition {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let doc = format_symbol_outer(&self.doc, &self.attributes, f)
            .append(self.visibility.format(f))
            .append("mod")
            .append(a.space())
            .append(self.name.format(f))
            .append(match &self.body {
                Some(body) => a.space().append(format_body(body, f)),
                None => a.text(";"),
            });

        format_with_extras(doc, &self.extras, f).append(if self.body.is_some() {
            a.hardline()
        } else {
            a.nil()
        })
    }
}

impl Format for ast::FunctionDefinition {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let doc = format_symbol_outer(&self.doc, &self.attributes, f)
            .append(self.visibility.format(f))
            .append("fn")
            .append(a.space())
            .append(self.name.format(f))
            .append(a.space())
            .append(self.parameters.format(f))
            .append(a.space())
            .append(format_body(&self.body, f));

        format_with_extras(doc, &self.extras, f).append(a.hardline())
    }
}

impl Format for ast::UseStatementPart {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        match &self {
            ast::UseStatementPart::Identifier(identifier) => identifier.format(f),
            ast::UseStatementPart::Glob(_) => f.arena.text("*"),
            ast::UseStatementPart::Error(_) => f.arena.nil(),
        }
    }
}

impl Format for ast::UseName {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let parts = a.intersperse(self.parts.iter().map(|p| p.format(f)), "::");
        format_with_extras(parts, &self.extras, f)
    }
}

impl Format for ast::UseStatement {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let doc = self
            .attributes
            .format(f)
            .append(self.visibility.format(f))
            .append("use")
            .append(a.space())
            .append(self.name.format(f));

        format_with_extras(doc, &self.extras, f)
    }
}

impl Format for ast::ConstAssignment {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let doc = format_symbol_outer(&self.doc, &self.attributes, f)
            .append(self.visibility.format(f))
            .append("const")
            .append(a.space())
            .append(format_assignment(
                &self.name,
                &self.ty,
                Some(self.value.as_ref()),
                f,
            ));

        format_with_extras(doc, &self.extras, f)
    }
}

impl Format for ast::InitDefinition {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let doc = format_symbol_outer(&self.doc, &self.attributes, f)
            .append(a.space())
            .append("init")
            .append(self.parameters.format(f))
            .append(a.space())
            .append(format_body(&self.body, f));

        format_with_extras(doc, &self.extras, f)
    }
}

impl Format for ast::Return {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        a.text("return").append(match &self.value {
            Some(value) => a.space().append(value.format(f)),
            None => a.nil(),
        })
    }
}

impl Format for ast::LocalAssignment {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        format_with_extras(
            self.attributes.format(f).append(format_assignment(
                &self.name,
                &self.ty,
                Some(&self.value),
                f,
            )),
            &self.extras,
            f,
        )
    }
}

impl Format for ast::PropertyAssignment {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        format_symbol_outer(&self.doc, &self.attributes, f)
            .append("prop")
            .append(a.space())
            .append(format_assignment(
                &self.name,
                &self.ty,
                Some(self.value.as_ref()),
                f,
            ))
    }
}

impl Format for ast::ExpressionStatement {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        format_with_extras(
            self.attributes.format(f).append(self.expression.format(f)),
            &self.extras,
            f,
        )
    }
}

impl Format for ast::AttributeCommand {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        match &self {
            ast::AttributeCommand::Ident(identifier) => identifier.format(f),
            ast::AttributeCommand::Assignment(local_assignment) => local_assignment.format(f),
            ast::AttributeCommand::Call(call) => call.format(f),
        }
    }
}

impl Format for ast::Attribute {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let commands = a.intersperse(self.commands.iter().map(|c| c.format(f)), ",");
        a.text(if self.is_inner { "#!" } else { "#" })
            .append(commands.brackets())
    }
}

impl Format for Vec<ast::Attribute> {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        if self.is_empty() {
            a.nil()
        } else {
            a.intersperse(self.iter().map(|attr| attr.format(f)), a.hardline())
                .append(a.hardline())
        }
    }
}

impl Format for ast::Statement {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        match &self {
            ast::Statement::Workbench(workbench_definition) => workbench_definition.format(f),
            ast::Statement::Module(module_definition) => module_definition.format(f),
            ast::Statement::Function(function_definition) => function_definition.format(f),
            ast::Statement::InnerDocComment(comment) => comment.format(f).append(a.hardline()),
            ast::Statement::Comment(comment) => comment.format(f),

            ast::Statement::Use(use_statement) => use_statement.format(f),
            ast::Statement::Const(const_assignment) => const_assignment.format(f),
            ast::Statement::Init(init_definition) => init_definition.format(f),
            ast::Statement::Return(r) => r.format(f),
            ast::Statement::InnerAttribute(attribute) => attribute.format(f),
            ast::Statement::LocalAssignment(local_assignment) => local_assignment.format(f),
            ast::Statement::Property(property_assignment) => property_assignment.format(f),
            ast::Statement::Expression(expression_statement) => expression_statement.format(f),
            ast::Statement::Error(_) => a.nil(),
        }
    }
}

impl Format for Vec<ast::Statement> {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;

        // Join statements with a hardline so they sit on separate lines
        a.intersperse(
            self.iter()
                .map(|statement| match statement.ends_with_semicolon() {
                    true => statement.format(f).append(";"),
                    false => statement.format(f),
                }),
            a.hardline(),
        )
    }
}

impl Format for ast::StatementList {
    fn format<'a>(&self, f: &Formatter<'a>) -> DocBuilder<'a> {
        let a = f.arena;
        let statements = self.statements.format(f);
        let doc = match (&self.statements.is_empty(), &self.tail) {
            (true, None) => a.nil(),
            (true, Some(tail)) => tail.format(f).group(),
            (false, None) => statements.group(),
            (false, Some(tail)) => statements
                .append(a.hardline())
                .append(tail.format(f))
                .group(),
        };

        format_with_extras(doc, &self.extras, f)
    }
}
