// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Lower, LowerContext, LowerError, LowerResult, ir,
    lower::{attribute::outer_with_doc, for_each_statement},
};

use microcad_lang_base::PushDiag;
use microcad_lang_parse::ast;

impl Lower<ast::def::FileModule> for ir::FileModule {
    fn lower(node: &ast::def::FileModule, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            attr: outer_with_doc(&node.doc, &node.attr, context)?,
            visibility: ir::Visibility::lower(&node.vis, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            id: ir::Identifier::lower(&node.id, context)?,
        })
    }
}

impl Lower<ast::Statement> for Option<ir::FileModule> {
    fn lower(stmt: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match stmt {
            ast::Statement::FileModule(file_module) => {
                Some(ir::FileModule::lower(file_module, context)?)
            }
            _ => None,
        })
    }
}

impl Lower<ast::StatementList> for ir::InlineModuleItems {
    fn lower(statements: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.src_ref(&stmt.span());
            use ast::Statement::*;
            Ok(match stmt {
                FileModule(_) | Return(_) | Expression(_) | LocalAssignment(_) | Property(_)
                | Error(_) => context
                    .diagnostics
                    .error(&src_ref, LowerError::StatementNotAllowed { src_ref })?,
                _ => {}
            })
        })?;

        Ok(Self {
            modules: Box::lower(statements, context)?,
            aliases: ir::Aliases::lower(statements, context)?,
            constants: Box::lower(statements, context)?,
            functions: Box::lower(statements, context)?,
            workbenches: Box::lower(statements, context)?,
        })
    }
}

impl Lower<ast::def::InlineModule> for ir::InlineModule {
    fn lower(node: &ast::def::InlineModule, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            outer_attr: crate::lower::attribute::outer_with_doc(&node.doc, &node.attr, context)?,
            visibility: ir::Visibility::lower(&node.vis, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            id: ir::Identifier::lower(&node.id, context)?,
            inner_attr: ir::InnerAttributes::lower(&node.body.statements, context)?,
            items: ir::InlineModuleItems::lower(&node.body.statements, context)?,
        })
    }
}

impl Lower<ast::Statement> for Option<ir::InlineModule> {
    fn lower(stmt: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match stmt {
            ast::Statement::InlineModule(inline_module) => {
                Some(ir::InlineModule::lower(inline_module, context)?)
            }
            _ => None,
        })
    }
}
