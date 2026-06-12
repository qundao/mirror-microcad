// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Lower, LowerContext, LowerError, LowerResult,
    ir::{self, WorkbenchStatement},
    lower::{
        attribute::outer_with_doc, extract_statements, extract_statements_with_tail,
        for_each_statement,
    },
};

use microcad_lang_base::{PushDiag, Refer, SrcRef};
use microcad_lang_parse::ast;

impl Lower<ast::InitDefinition> for ir::Init {
    fn lower(node: &ast::InitDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(&node.body.statements, context, |stmt, context| {
            let src_ref = context.src_ref(&stmt.span());
            use ast::Statement::*;
            Ok(match stmt {
                FileModule(_) | InlineModule(_) | Function(_) | Workbench(_) | Return(_)
                | Use(_) | Property(_) | Const(_) | InnerDocComment(_) | InnerAttribute(_)
                | Error(_) => context
                    .diagnostics
                    .error(&src_ref, LowerError::StatementNotAllowed { src_ref })?,
                _ => {}
            })
        })?;

        Ok(Self {
            attr: crate::lower::attribute::outer_with_doc(&node.doc, &node.attributes, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            parameters: ir::ParameterList::lower(&node.parameters, context)?,
            statements: ir::WorkbenchStatements::lower(&node.body.statements, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower<ast::Body> for ir::Group {
    fn lower(node: &ast::Body, context: &mut LowerContext) -> LowerResult<Self> {
        let statements = &node.statements;
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.src_ref(&stmt.span());
            use ast::Statement::*;
            Ok(match stmt {
                FileModule(_) | Const(_) | Use(_) | InlineModule(_) | Init(_) | Workbench(_)
                | Function(_) | Return(_) | InnerAttribute(_) | InnerDocComment(_) | Error(_) => {
                    context
                        .diagnostics
                        .error(&src_ref, LowerError::StatementNotAllowed { src_ref })?
                }
                _ => {}
            })
        })?;

        Ok(Self(Refer::new(
            ir::WorkbenchStatements::lower(statements, context)?,
            context.src_ref(&node.span),
        )))
    }
}

impl Lower<ast::Expression> for ir::WorkbenchExpression {
    fn lower(node: &ast::Expression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Expression::Call(expr) => Self::Call(ir::Call::lower(expr, context)?),
            ast::Expression::Bracketed(expr, _) => Self::lower(expr, context)?,
            ast::Expression::Literal(ast::Literal {
                literal: ast::LiteralKind::String(s),
                ..
            }) => Self::FormatString(ir::FormatString::lower(s, context)?),
            ast::Expression::Literal(expr) => Self::Literal(ir::Literal::lower(expr, context)?),
            ast::Expression::String(s) => Self::FormatString(ir::FormatString::lower(s, context)?),
            ast::Expression::Tuple(t) => {
                Self::TupleExpression(ir::TupleExpression::lower(t, context)?)
            }
            ast::Expression::ArrayRange(a) => Self::ArrayExpression(ir::ArrayExpression {
                inner: ir::ArrayExpressionInner::Range(ir::RangeExpression::lower(a, context)?),
                unit: ir::Unit::lower(&a.unit, context)?,
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::ArrayList(a) => Self::ArrayExpression(ir::ArrayExpression {
                inner: ir::ArrayExpressionInner::List(ir::ListExpression::lower(a, context)?),
                unit: ir::Unit::lower(&a.unit, context)?,
                src_ref: context.src_ref(&a.span),
            }),
            ast::Expression::QualifiedName(n) => Self::Name(ir::QualifiedName::lower(n, context)?),
            ast::Expression::BinaryOperation(binop) => {
                Self::BinaryOp(ir::BinaryOp::lower(binop, context)?)
            }
            ast::Expression::UnaryOperation(unop) => {
                Self::UnaryOp(ir::UnaryOp::lower(unop, context)?)
            }
            ast::Expression::Marker(identifier) => {
                Self::Marker(ir::Marker::lower(identifier, context)?)
            }
            ast::Expression::Body(body) => Self::Group(ir::Group::lower(body, context)?),
            ast::Expression::ElementAccess(access) => access.element_chain.iter().try_fold(
                Self::lower(&access.value, context)?,
                |acc, element| -> LowerResult<Self> {
                    use ast::ElementInner::*;
                    let src_ref = context.src_ref(&access.span);
                    let lhs = Box::new(acc);

                    Ok(match &element.inner {
                        Attribute(a) => Self::MetaAccess(ir::ElementAccess {
                            lhs,
                            element: ir::Identifier::lower(a, context)?,
                            src_ref,
                        }),
                        Tuple(t) => Self::PropertyAccess(ir::ElementAccess {
                            lhs,
                            element: ir::Identifier::lower(t, context)?,
                            src_ref,
                        }),
                        Method(m) => Self::MethodCall(ir::ElementAccess {
                            lhs,
                            element: ir::Call::lower(m, context)?,
                            src_ref,
                        }),
                        ArrayElement(e) => Self::ArrayAccess(ir::ElementAccess {
                            lhs,
                            element: Box::new(ir::ConstantExpression::lower(e, context)?),
                            src_ref,
                        }),
                    })
                },
            )?,
            ast::Expression::If(if_expr) => Self::If(ir::If::lower(&if_expr, context)?),
            ast::Expression::Error(_) => todo!(),
        })
    }
}

impl Lower<ast::StatementList> for ir::Inits {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::Init(init) => Some(ir::Init::lower(init, context)?),
                _ => None,
            })
        })?))
    }
}

impl Lower<ast::LocalAssignment> for ir::WorkbenchStatement {
    fn lower(node: &ast::LocalAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: ir::OuterAttributes::lower(&node.attributes, context)?,
            src_ref: context.src_ref(&node.span),
            visibility: ir::Visibility::Private,
            keyword_src_ref: SrcRef::none(),
            id: Some(ir::Identifier::lower(&node.name, context)?),
            ty: Option::<ir::TypeAnnotation>::lower(&node.ty, context)?,
            expression: ir::WorkbenchExpression::lower(node.value.as_ref(), context)?,
        })
    }
}

impl Lower<ast::PropertyAssignment> for ir::WorkbenchStatement {
    fn lower(node: &ast::PropertyAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: outer_with_doc(&node.doc, &node.attributes, context)?,
            src_ref: context.src_ref(&node.span),
            visibility: ir::Visibility::Public,
            keyword_src_ref: context.src_ref(&node.keyword_span),
            id: Some(ir::Identifier::lower(&node.name, context)?),
            ty: Option::<ir::TypeAnnotation>::lower(&node.ty, context)?,
            expression: ir::WorkbenchExpression::lower(node.value.as_ref(), context)?,
        })
    }
}

impl Lower<ast::ExpressionStatement> for ir::WorkbenchStatement {
    fn lower(node: &ast::ExpressionStatement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: ir::OuterAttributes::lower(&node.attributes, context)?,
            src_ref: context.src_ref(&node.span),
            visibility: ir::Visibility::Public,
            keyword_src_ref: SrcRef::none(),
            id: None,
            ty: None,
            expression: ir::WorkbenchExpression::lower(&node.expression, context)?,
        })
    }
}

impl Lower<ast::Statement> for Option<ir::WorkbenchStatement> {
    fn lower(stmt: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match stmt {
            ast::Statement::LocalAssignment(local_assignment) => {
                Some(ir::WorkbenchStatement::lower(local_assignment, context)?)
            }
            ast::Statement::Property(property_assignment) => {
                Some(ir::WorkbenchStatement::lower(property_assignment, context)?)
            }
            ast::Statement::Expression(expression_statement) => Some(
                ir::WorkbenchStatement::lower(expression_statement, context)?,
            ),
            _ => None,
        })
    }
}

impl Lower<ast::StatementList> for ir::WorkbenchStatements {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(extract_statements_with_tail(
            node,
            context,
            // Extract statements
            |stmt, context| Option::<ir::WorkbenchStatement>::lower(stmt, context),
            // Extract tail expression
            |expr, context| WorkbenchStatement::lower(expr, context),
        )?))
    }
}

impl Lower<ast::StatementList> for ir::WorkbenchItems {
    fn lower(statements: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.src_ref(&stmt.span());
            Ok(match stmt {
                ast::Statement::FileModule(_)
                | ast::Statement::InlineModule(_)
                | ast::Statement::Function(_)
                | ast::Statement::Workbench(_)
                | ast::Statement::Return(_)
                | ast::Statement::Error(_) => context
                    .diagnostics
                    .error(&src_ref, LowerError::StatementNotAllowed { src_ref })?,
                _ => {}
            })
        })?;

        Ok(Self {
            aliases: ir::Aliases::lower(statements, context)?,
            constants: ir::Constants::lower(statements, context)?,
        })
    }
}

impl Lower<ast::WorkbenchDefinition> for ir::Workbench {
    fn lower(node: &ast::WorkbenchDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        if let Some(tail) = node.body.statements.tail.as_ref() {
            context
                .warning(LowerError::ImplicitWorkbenchReturn {
                    src_ref: context.src_ref(&tail.span),
                })
                .ok();
        }

        Ok(Self {
            keyword_ref: context.src_ref(&node.keyword_span),
            outer_attr: crate::lower::attribute::outer_with_doc(
                &node.doc,
                &node.attributes,
                context,
            )?,
            visibility: ir::Visibility::lower(&node.visibility, context)?,
            kind: Refer::new(node.kind.into(), context.src_ref(&node.span)),
            id: ir::Identifier::lower(&node.name, context)?,
            parameters: ir::ParameterList::lower(&node.plan, context)?,
            inner_attr: ir::InnerAttributes::lower(&node.body.statements, context)?,
            inits: ir::Inits::lower(&node.body.statements, context)?,
            items: ir::WorkbenchItems::lower(&node.body.statements, context)?,
            statements: ir::WorkbenchStatements::lower(&node.body.statements, context)?,
        })
    }
}

impl Lower<ast::StatementList> for ir::Workbenches {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::Workbench(w) => Some(ir::Workbench::lower(w, context)?),
                _ => None,
            })
        })?))
    }
}
