// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Ir, Lower, LowerContext, LowerError, LowerResult, ir, lower::for_each_statement};

use microcad_lang_base::SpanToSrcRef;
use microcad_lang_parse::{Ast, ast};

impl Lower<ast::StatementList> for ir::SourceItems {
    fn lower(statements: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.span_to_src_ref(&stmt.span());
            match stmt {
                ast::Statement::Init(_)
                | ast::Statement::Return(_)
                | ast::Statement::Property(_)
                | ast::Statement::Error(_) => {
                    context.diag(LowerError::StatementNotAllowed { src_ref })
                }
                _ => {}
            }
            Ok(())
        })?;

        Ok(Self {
            file_modules: Box::lower(statements, context)?,
            inline_modules: Box::lower(statements, context)?,
            aliases: ir::Aliases::lower(statements, context)?,
            constants: Box::lower(statements, context)?,
            functions: Box::lower(statements, context)?,
            workbenches: Box::lower(statements, context)?,
        })
    }
}

impl Lower<Ast> for Ir {
    fn lower(node: &Ast, context: &mut LowerContext) -> LowerResult<Self> {
        let statements = &node.statements;

        Ok(Self {
            attr: ir::InnerAttributes::lower(statements, context)?,
            items: ir::SourceItems::lower(statements, context)?,
            statements: Box::lower(statements, context)?,
        })
    }
}
