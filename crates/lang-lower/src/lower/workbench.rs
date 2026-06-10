// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, LowerResult, ir, lower::extract_statements};

use microcad_lang_base::Refer;
use microcad_lang_parse::ast;

impl Lower<ast::InitDefinition> for ir::Init {
    fn lower(node: &ast::InitDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: crate::lower::attribute::outer_with_doc(&node.doc, &node.attributes, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            parameters: ir::ParameterList::lower(&node.parameters, context)?,
            body: ir::InitBody::lower(&node.body.statements, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower<ast::Expression> for ir::WorkbenchExpression {
    fn lower(node: &ast::Expression, context: &mut LowerContext) -> LowerResult<Self> {
        todo!()
    }
}

impl Lower<ast::LocalAssignment> for ir::LocalAssignment<ir::WorkbenchExpression> {
    fn lower(node: &ast::LocalAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        todo!()
    }
}

impl Lower<ast::StatementList> for ir::InitBody {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::LocalAssignment(l) => Some(ir::LocalAssignment::<
                    ir::WorkbenchExpression,
                >::lower(l, context)?),
                _ => None,
            })
        })?))
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

impl Lower<ast::WorkbenchDefinition> for ir::Workbench {
    fn lower(node: &ast::WorkbenchDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        if let Some(tail) = node.body.statements.tail.as_ref() {
            context
                .warning(LowerError::ImplicitWorkbenchReturn {
                    src_ref: context.src_ref(&tail.span),
                })
                .ok();
        }

        Ok(ir::Workbench {
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
            inner_attr: ir::Attributes::lower(&node.body.statements, context)?,
            aliases: ir::Aliases::lower(&node.body.statements, context)?,
            constants: ir::Constants::lower(&node.body.statements, context)?,
            inits: ir::Inits::lower(&node.body.statements, context)?,
            statements: ir::WorkbenchStatements::lower(&node.body.statements, context)?,
        })
    }
}

impl Lower<ast::StatementList> for ir::WorkbenchStatements {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        todo!()
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
