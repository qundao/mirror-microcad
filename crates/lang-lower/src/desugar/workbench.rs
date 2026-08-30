// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    CastInto, Desugar, LowerContext, LowerError, LowerResult,
    desugar::{attribute::outer_with_doc, extract_statements, for_each_statement},
    ir::{self, ReturnStatement, WorkbenchExpression},
};

use microcad_builtin::__mu;
use microcad_lang_base::{SpanToSrcRef, SrcRef};
use microcad_lang_parse::ast;

impl Desugar<ast::Statement> for Option<ir::InitStatement> {
    fn desugar(node: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Statement::LocalAssignment(local_assignment) => {
                Some(ir::InitStatement::desugar(local_assignment, context)?)
            }
            _ => {
                context.diag(LowerError::StatementNotAllowed {
                    src_ref: context.span_to_src_ref(&node.span()),
                });

                None
            }
        })
    }
}

impl Desugar<ast::ExpressionStatement> for Option<ir::InitStatement> {
    fn desugar(node: &ast::ExpressionStatement, context: &mut LowerContext) -> LowerResult<Self> {
        context.diag(LowerError::StatementNotAllowed {
            src_ref: context.span_to_src_ref(&node.span),
        });
        Ok(None)
    }
}

impl Desugar<ast::LocalAssignment> for ir::InitStatement {
    fn desugar(node: &ast::LocalAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        let stmt = Self {
            name: ir::Identifier::desugar(&node.id, context)?,
            expression: ir::WorkbenchExpression::desugar(node.expr.as_ref(), context)?,
            src_ref: context.span_to_src_ref(&node.span),
        };

        if let Some(ty) = &node.ty {
            context.diag(LowerError::InvalidInitStatement {
                src_ref: context.span_to_src_ref(&ty.span()),
                stmt_src_ref: stmt.src_ref,
            });
        }

        Ok(stmt)
    }
}

impl ir::Init {
    /// Fill up missing parameters or with default parameters.
    pub(crate) fn fill_up_and_check(
        &mut self,
        inputs: &ir::ParameterList,
        context: &mut LowerContext,
    ) {
        if inputs == &self.parameters {
            context.diag(LowerError::DuplicatedDefaultInitializer {
                src_ref: self.src_ref,
            });
            return;
        }

        self.statements.iter().for_each(|stmt| {
            if inputs.get_by_name(&stmt.name).is_none() {
                context.diag(LowerError::NotAnInputProperty {
                    name: stmt.name.clone(),
                    possible_inputs: inputs.names().cloned().collect(),
                })
            }
        });

        let mut new_statements = Vec::new();

        self.parameters.iter().for_each(|param| {
            match (self.find_statement(&param.id), &param.default_value) {
                (None, Some(default_value)) => {
                    let expr: WorkbenchExpression = default_value.clone().cast_into();
                    new_statements.push(ir::InitStatement::new(&param.id, expr));
                }
                _ => {}
            }
        });

        self.statements = self
            .statements
            .to_vec()
            .into_iter()
            .chain(new_statements)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let mut new_statements = Vec::new();

        inputs.iter().for_each(|param| {
            match (self.find_statement(&param.id), &param.default_value) {
                (None, Some(default_value)) => {
                    let expr: WorkbenchExpression = default_value.clone().cast_into();
                    new_statements.push(ir::InitStatement::new(&param.id, expr));
                }
                (None, None) => {
                    context.diag(LowerError::InputNotInitialized {
                        name: param.id.clone(),
                        src_ref: self.src_ref,
                        param_src_ref: param.src_ref,
                    });
                }
                (Some(_), _) => {}
            }
        });

        self.statements = self
            .statements
            .to_vec()
            .into_iter()
            .chain(new_statements)
            .collect::<Vec<_>>()
            .into_boxed_slice();
    }
}

impl Desugar<ast::Init> for ir::Init {
    fn desugar(node: &ast::Init, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(&node.body.statements, context, |stmt, context| {
            let src_ref = context.span_to_src_ref(&stmt.span());
            use ast::Statement::*;
            match stmt {
                FileModule(_) | InlineModule(_) | Function(_) | Workbench(_) | Return(_)
                | Use(_) | Const(_) | InnerDocComment(_) | InnerAttribute(_) | Error(_) => {
                    context.diag(LowerError::StatementNotAllowed { src_ref })
                }
                _ => {}
            }
            Ok(())
        })?;

        let init = Self {
            attr: crate::desugar::attribute::outer_with_doc(&node.doc, &node.attr, context)?,
            keyword_ref: context.span_to_src_ref(&node.keyword_span),
            parameters: ir::ParameterList::desugar(&node.parameters, context)?,
            statements: Box::desugar(&node.body.statements, context)?,
            src_ref: context.span_to_src_ref(&node.span),
        };

        Ok(init)
    }
}

impl Desugar<ast::Body> for ir::Group {
    fn desugar(node: &ast::Body, context: &mut LowerContext) -> LowerResult<Self> {
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
            attr: ir::Attributes::desugar(statements, context)?,
            statements: Box::desugar(statements, context)?,
        })
    }
}

impl Desugar<ast::Expression> for ir::WorkbenchExpression {
    fn desugar(node: &ast::Expression, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Expression::Call(expr) => Self::Call(ir::Call::desugar(expr, context)?),
            ast::Expression::Bracketed(expr, _) => Self::desugar(expr.as_ref(), context)?,
            ast::Expression::Literal(ast::Literal {
                literal: ast::LiteralKind::String(s),
                ..
            }) => Self::Value(ir::ConstantValue::from_value(s.content.clone())),
            ast::Expression::Literal(expr) => {
                Self::Value(ir::ConstantValue::desugar(expr, context)?)
            }
            ast::Expression::String(s) => Self::Call(ir::Call::desugar(s, context)?.cast_into()),
            ast::Expression::Tuple(t) => Self::Call(ir::Call::desugar(t, context)?),
            ast::Expression::Range(a) => Self::desugar(a, context)?,
            ast::Expression::List(a) => Self::desugar(a, context)?,
            ast::Expression::SymbolPath(n) => Self::Path(ir::Path::desugar(n, context)?),
            ast::Expression::BinaryOperation(binop) => {
                Self::Call(ir::Call::desugar(binop, context)?)
            }
            ast::Expression::UnaryOperation(unop) => Self::Call(ir::Call::desugar(unop, context)?),
            ast::Expression::Marker(identifier) => {
                Self::Marker(ir::Marker::desugar(identifier, context)?)
            }
            ast::Expression::Body(body) => Self::Group(ir::Group::desugar(body, context)?),
            ast::Expression::ElementAccess(access) => access.element_chain.iter().try_fold(
                Self::desugar(access.expr.as_ref(), context)?,
                |lhs, element| -> LowerResult<Self> {
                    use ast::ElementInner::*;
                    let src_ref = context.span_to_src_ref(&access.span);

                    Ok(match &element.inner {
                        Attribute(a) => Self::Call(ir::Call {
                            path: __mu!(core::attribute_access),
                            args: ir::ArgumentList::from_iter([
                                lhs,
                                Self::Path(ir::Path::from(a.name.to_string())),
                            ]),
                            src_ref,
                        }),
                        Tuple(t) => Self::Call(ir::Call {
                            path: __mu!(core::member_access),
                            args: ir::ArgumentList::from_iter([
                                lhs,
                                Self::Path(ir::Path::from(t.name.to_string())),
                            ]),
                            src_ref,
                        }),
                        Method(m) => Self::Call(ir::Call {
                            path: ir::Path::desugar(&m.path, context)?,
                            args: ir::ArgumentList::desugar(&m.arguments, context)?
                                .desugar_method(lhs),
                            src_ref,
                        }),
                        ListElement(e) => Self::Call(ir::Call {
                            path: __mu!(core::list_access),
                            args: ir::ArgumentList::from_iter([
                                lhs,
                                Self::desugar(e.as_ref(), context)?,
                            ]),
                            src_ref,
                        }),
                    })
                },
            )?,
            ast::Expression::If(if_expr) => Self::If(ir::If::desugar(if_expr, context)?),
            ast::Expression::Error(_) => todo!(),
        })
    }
}

impl Desugar<ast::StatementList> for Vec<ir::Init> {
    fn desugar(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
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

        Ok(Vec::from_iter(extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::Init(init) => Some(ir::Init::desugar(init, context)?),
                _ => None,
            })
        })?))
    }
}

impl Desugar<ast::LocalAssignment> for ir::WorkbenchStatement {
    fn desugar(node: &ast::LocalAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: ir::Attributes::desugar(&node.attr, context)?,
            src_ref: context.span_to_src_ref(&node.span),
            visibility: ir::Visibility::Private,
            keyword_src_ref: SrcRef::none(),
            name: Some(ir::Identifier::desugar(&node.id, context)?),
            ty: ir::Type::desugar(&node.ty, context)?,
            expression: ir::WorkbenchExpression::desugar(node.expr.as_ref(), context)?,
        })
    }
}

impl Desugar<ast::PropertyAssignment> for ir::WorkbenchStatement {
    fn desugar(node: &ast::PropertyAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: outer_with_doc(&node.doc, &node.attr, context)?,
            src_ref: context.span_to_src_ref(&node.span),
            visibility: ir::Visibility::Public,
            keyword_src_ref: context.span_to_src_ref(&node.keyword_span),
            name: Some(ir::Identifier::desugar(&node.id, context)?),
            ty: ir::Type::desugar(&node.ty, context)?,
            expression: ir::WorkbenchExpression::desugar(node.value.as_ref(), context)?,
        })
    }
}

impl Desugar<ast::ExpressionStatement> for ir::WorkbenchStatement {
    fn desugar(node: &ast::ExpressionStatement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: ir::Attributes::desugar(&node.attr, context)?,
            src_ref: context.span_to_src_ref(&node.span),
            visibility: ir::Visibility::Public,
            keyword_src_ref: SrcRef::none(),
            name: None,
            ty: ir::Type::default(),
            expression: ir::WorkbenchExpression::desugar(&node.expr, context)?,
        })
    }
}

impl Desugar<ast::ExpressionStatement> for Option<ir::WorkbenchStatement> {
    fn desugar(node: &ast::ExpressionStatement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Some(ir::WorkbenchStatement::desugar(node, context)?))
    }
}

impl Desugar<ast::Statement> for Option<ir::WorkbenchStatement> {
    fn desugar(stmt: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match stmt {
            ast::Statement::LocalAssignment(local_assignment) => {
                Some(ir::WorkbenchStatement::desugar(local_assignment, context)?)
            }
            ast::Statement::Property(property_assignment) => Some(ir::WorkbenchStatement::desugar(
                property_assignment,
                context,
            )?),
            ast::Statement::Expression(expression_statement) => Some(
                ir::WorkbenchStatement::desugar(expression_statement, context)?,
            ),
            _ => None,
        })
    }
}

impl Desugar<ast::StatementList> for ir::desugared::WorkbenchItems {
    fn desugar(statements: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
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
            aliases: ir::desugared::Aliases::desugar(statements, context)?,
            constants: Box::desugar(statements, context)?,
            functions: Box::desugar(statements, context)?,
        })
    }
}

impl Desugar<ast::def::Workbench> for ir::desugared::Workbench {
    fn desugar(node: &ast::def::Workbench, context: &mut LowerContext) -> LowerResult<Self> {
        let parameters = ir::ParameterList::desugar(&node.parameters, context)?;
        let mut inits: Vec<ir::Init> = Vec::desugar(&node.body.statements, context)?;

        inits
            .iter_mut()
            .for_each(|init| init.fill_up_and_check(&parameters, context));

        // Add parameters as default initializer.
        // This makes evaluation easier, because then we can simply trait the workbench parameters as default initializer.
        inits.push(ir::Init::default_init(parameters.clone()));

        Ok(Self {
            meta: ir::Meta {
                name: Some(ir::Identifier::desugar(&node.id, context)?),
                keyword_src_ref: context.span_to_src_ref(&node.keyword_span),
                vis: ir::Visibility::desugar(&node.vis, context)?,
                src_ref: context.span_to_src_ref(&node.span),
            },
            attr: crate::desugar::attribute::outer_with_doc(&node.doc, &node.attr, context)?
                .extend(ir::Attributes::desugar(&node.body.statements, context)?),
            kind: node.kind,
            parameters,
            inits: inits.into_boxed_slice(),
            items: ir::desugared::WorkbenchItems::desugar(&node.body.statements, context)?,
            statements: Box::desugar(&node.body.statements, context)?,
        })
    }
}

impl Desugar<ast::Statement> for Option<ir::desugared::Workbench> {
    fn desugar(node: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            ast::Statement::Workbench(w) => Some(ir::desugared::Workbench::desugar(w, context)?),
            _ => None,
        })
    }
}

impl Desugar<ast::ExpressionStatement> for Option<ir::desugared::Workbench> {
    fn desugar(_: &ast::ExpressionStatement, _: &mut LowerContext) -> LowerResult<Self> {
        Ok(None)
    }
}
