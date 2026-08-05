// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    CastInto, Lower, LowerContext, LowerError, LowerResult, ir,
    lower::{attribute::outer_with_doc, extract_statements, for_each_statement},
};

use microcad_lang_base::{__mu, Refer, SpanToSrcRef, SrcRef};
use microcad_lang_parse::ast;

impl Lower<ast::Init> for ir::Init {
    fn lower(node: &ast::Init, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(&node.body.statements, context, |stmt, context| {
            let src_ref = context.span_to_src_ref(&stmt.span());
            use ast::Statement::*;
            match stmt {
                FileModule(_) | InlineModule(_) | Function(_) | Workbench(_) | Return(_)
                | Use(_) | Property(_) | Const(_) | InnerDocComment(_) | InnerAttribute(_)
                | Error(_) => context.diag(LowerError::StatementNotAllowed { src_ref }),
                _ => {}
            }
            Ok(())
        })?;

        Ok(Self {
            attr: crate::lower::attribute::outer_with_doc(&node.doc, &node.attr, context)?,
            keyword_ref: context.span_to_src_ref(&node.keyword_span),
            parameters: ir::ParameterList::lower(&node.parameters, context)?,
            statements: Box::lower(&node.body.statements, context)?,
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl Lower<ast::Body> for ir::Group {
    fn lower(node: &ast::Body, context: &mut LowerContext) -> LowerResult<Self> {
        let statements = &node.statements;
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.span_to_src_ref(&stmt.span());
            use ast::Statement::*;
            match stmt {
                FileModule(_) | Const(_) | Use(_) | InlineModule(_) | Init(_) | Workbench(_)
                | Function(_) | Return(_) | InnerAttribute(_) | InnerDocComment(_) | Error(_) => {
                    context.diag(LowerError::StatementNotAllowed { src_ref })
                }
                _ => {}
            }
            Ok(())
        })?;

        Ok(Self {
            src_ref: context.span_to_src_ref(&node.span),
            attr: ir::InnerAttributes::lower(statements, context)?,
            statements: Box::lower(statements, context)?,
        })
    }
}

impl Lower<ast::Expression> for ir::WorkbenchExpression {
    fn lower(node: &ast::Expression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Expression::Call(expr) => Self::Call(ir::Call::lower(expr, context)?),
            ast::Expression::Bracketed(expr, _) => Self::lower(expr.as_ref(), context)?,
            ast::Expression::Literal(ast::Literal {
                literal: ast::LiteralKind::String(s),
                ..
            }) => Self::Literal(ir::Literal::from_value(s.content.clone())),
            ast::Expression::Literal(expr) => Self::Literal(ir::Literal::lower(expr, context)?),
            ast::Expression::String(s) => Self::Call(ir::Call::lower(s, context)?.cast_into()),
            ast::Expression::Tuple(t) => Self::Call(ir::Call::lower(t, context)?),
            ast::Expression::ArrayRange(a) => Self::lower(a, context)?,
            ast::Expression::ArrayList(a) => Self::lower(a, context)?,
            ast::Expression::SymbolPath(n) => Self::Name(ir::SymbolPath::lower(n, context)?),
            ast::Expression::BinaryOperation(binop) => Self::Call(ir::Call::lower(binop, context)?),
            ast::Expression::UnaryOperation(unop) => Self::Call(ir::Call::lower(unop, context)?),
            ast::Expression::Marker(identifier) => {
                Self::Marker(ir::Marker::lower(identifier, context)?)
            }
            ast::Expression::Body(body) => Self::Group(ir::Group::lower(body, context)?),
            ast::Expression::ElementAccess(access) => access.element_chain.iter().try_fold(
                Self::lower(access.expr.as_ref(), context)?,
                |lhs, element| -> LowerResult<Self> {
                    use ast::ElementInner::*;
                    let src_ref = context.span_to_src_ref(&access.span);

                    Ok(match &element.inner {
                        Attribute(a) => Self::Call(ir::Call {
                            name: __mu("attribute_access").into(),
                            args: ir::ArgumentList::from_iter([
                                lhs,
                                Self::Name(ir::SymbolPath::from(a.name.to_string())),
                            ]),
                            src_ref,
                        }),
                        Tuple(t) => Self::Call(ir::Call {
                            name: __mu("property_access").into(),
                            args: ir::ArgumentList::from_iter([
                                lhs,
                                Self::Name(ir::SymbolPath::from(t.name.to_string())),
                            ]),
                            src_ref,
                        }),
                        Method(m) => Self::Call(ir::Call {
                            name: ir::SymbolPath::lower(&m.name, context)?,
                            args: ir::ArgumentList::lower(&m.arguments, context)?.prepended(lhs),
                            src_ref,
                        }),
                        ArrayElement(e) => Self::Call(ir::Call {
                            name: __mu("array_access").into(),
                            args: ir::ArgumentList::from_iter([
                                lhs,
                                Self::lower(e.as_ref(), context)?,
                            ]),
                            src_ref,
                        }),
                    })
                },
            )?,
            ast::Expression::If(if_expr) => Self::If(ir::If::lower(if_expr, context)?),
            ast::Expression::Error(_) => todo!(),
        })
    }
}

impl Lower<ast::StatementList> for Box<[ir::Init]> {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        fn is_init(stmt: &ast::Statement) -> bool {
            matches!(stmt, ast::Statement::Init(_))
        }

        // 1. Find the FIRST occurrence of an Initializer
        let statements = &node.statements;
        let first_idx = statements.iter().map(|(stmt, _)| stmt).position(is_init);

        // 2. Find the LAST occurrence of an Initializer
        let last_idx = statements.iter().map(|(stmt, _)| stmt).rposition(is_init);

        if let (Some(first), Some(last)) = (first_idx, last_idx) {
            // 3. Output an error on any WorkbenchStatement before an initializer
            statements[0..first]
                .iter()
                .try_for_each(|(stmt, _)| -> LowerResult<()> {
                    let src_ref = match stmt {
                        ast::Statement::LocalAssignment(local_assignment) => {
                            context.span_to_src_ref(&local_assignment.span)
                        }
                        ast::Statement::Property(property_assignment) => {
                            context.span_to_src_ref(&property_assignment.span)
                        }
                        ast::Statement::Expression(expression_statement) => {
                            context.span_to_src_ref(&expression_statement.span)
                        }
                        ast::Statement::Error(span) => context.span_to_src_ref(span),
                        _ => SrcRef::none(),
                    };

                    if src_ref.is_some() {
                        let src_ref = context.span_to_src_ref(&stmt.span());
                        context.diag(LowerError::StatementNotAllowed { src_ref });
                    }
                    Ok(())
                })?;

            // 4. Output an error on any none initializer statement in between the range
            statements[first..=last]
                .iter()
                .try_for_each(|(stmt, _)| -> LowerResult<()> {
                    if !is_init(stmt) {
                        let src_ref = context.span_to_src_ref(&stmt.span());
                        context.diag(LowerError::StatementNotAllowed { src_ref });
                    }
                    Ok(())
                })?;
        }

        extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::Init(init) => Some(ir::Init::lower(init, context)?),
                _ => None,
            })
        })
    }
}

impl Lower<ast::LocalAssignment> for ir::WorkbenchStatement {
    fn lower(node: &ast::LocalAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: ir::OuterAttributes::lower(&node.attr, context)?,
            src_ref: context.span_to_src_ref(&node.span),
            visibility: ir::Visibility::Private,
            keyword_src_ref: SrcRef::none(),
            id: Some(ir::Identifier::lower(&node.id, context)?),
            ty: ir::Type::lower(&node.ty, context)?,
            expression: ir::WorkbenchExpression::lower(node.expr.as_ref(), context)?,
        })
    }
}

impl Lower<ast::PropertyAssignment> for ir::WorkbenchStatement {
    fn lower(node: &ast::PropertyAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: outer_with_doc(&node.doc, &node.attr, context)?,
            src_ref: context.span_to_src_ref(&node.span),
            visibility: ir::Visibility::Public,
            keyword_src_ref: context.span_to_src_ref(&node.keyword_span),
            id: Some(ir::Identifier::lower(&node.id, context)?),
            ty: ir::Type::lower(&node.ty, context)?,
            expression: ir::WorkbenchExpression::lower(node.value.as_ref(), context)?,
        })
    }
}

impl Lower<ast::ExpressionStatement> for ir::WorkbenchStatement {
    fn lower(node: &ast::ExpressionStatement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: ir::OuterAttributes::lower(&node.attr, context)?,
            src_ref: context.span_to_src_ref(&node.span),
            visibility: ir::Visibility::Public,
            keyword_src_ref: SrcRef::none(),
            id: None,
            ty: ir::Type::default(),
            expression: ir::WorkbenchExpression::lower(&node.expr, context)?,
        })
    }
}

impl Lower<ast::ExpressionStatement> for Option<ir::WorkbenchStatement> {
    fn lower(node: &ast::ExpressionStatement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Some(ir::WorkbenchStatement::lower(node, context)?))
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

impl Lower<ast::StatementList> for ir::WorkbenchItems {
    fn lower(statements: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.span_to_src_ref(&stmt.span());
            match stmt {
                ast::Statement::FileModule(_)
                | ast::Statement::InlineModule(_)
                | ast::Statement::Workbench(_)
                | ast::Statement::Return(_)
                | ast::Statement::Error(_) => {
                    context.diag(LowerError::StatementNotAllowed { src_ref })
                }
                _ => {}
            }
            Ok(())
        })?;

        Ok(Self {
            aliases: ir::Aliases::lower(statements, context)?,
            constants: Box::lower(statements, context)?,
            functions: Box::lower(statements, context)?,
        })
    }
}

impl Lower<ast::def::Workbench> for ir::Workbench {
    fn lower(node: &ast::def::Workbench, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            keyword_ref: context.span_to_src_ref(&node.keyword_span),
            outer_attr: crate::lower::attribute::outer_with_doc(&node.doc, &node.attr, context)?,
            visibility: ir::Visibility::lower(&node.vis, context)?,
            kind: Refer::new(node.kind, context.span_to_src_ref(&node.span)),
            id: ir::Identifier::lower(&node.id, context)?,
            parameters: ir::ParameterList::lower(&node.parameters, context)?,
            inner_attr: ir::InnerAttributes::lower(&node.body.statements, context)?,
            inits: Box::lower(&node.body.statements, context)?,
            items: ir::WorkbenchItems::lower(&node.body.statements, context)?,
            statements: Box::lower(&node.body.statements, context)?,
        })
    }
}

impl Lower<ast::Statement> for Option<ir::Workbench> {
    fn lower(node: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Statement::Workbench(w) => Some(ir::Workbench::lower(w, context)?),
            _ => None,
        })
    }
}

impl Lower<ast::ExpressionStatement> for Option<ir::Workbench> {
    fn lower(_: &ast::ExpressionStatement, _: &mut LowerContext) -> LowerResult<Self> {
        Ok(None)
    }
}
