// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, LowerResult, ir, lower::for_each_statement};

use microcad_lang_base::PushDiag;
use microcad_lang_parse::ast;

impl Lower<ast::StatementList> for ir::SourceItems {
    fn lower(statements: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        for_each_statement(statements, context, |stmt, context| {
            let src_ref = context.src_ref(&stmt.span());
            Ok(match stmt {
                ast::Statement::Init(_)
                | ast::Statement::Return(_)
                | ast::Statement::Property(_)
                | ast::Statement::Error(_) => context
                    .diagnostics
                    .error(&src_ref, LowerError::StatementNotAllowed { src_ref })?,
                _ => {}
            })
        })?;

        Ok(Self {
            file_modules: ir::FileModules::lower(statements, context)?,
            inline_modules: ir::InlineModules::lower(statements, context)?,
            aliases: ir::Aliases::lower(statements, context)?,
            constants: ir::Constants::lower(statements, context)?,
            functions: ir::Functions::lower(statements, context)?,
            workbenches: ir::Workbenches::lower(statements, context)?,
        })
    }
}

impl Lower<ast::Source> for ir::Source {
    fn lower(node: &ast::Source, context: &mut LowerContext) -> super::LowerResult<Self> {
        let statements = &node.ast.value.statements;

        Ok(Self {
            attr: ir::InnerAttributes::lower(statements, context)?,
            items: ir::SourceItems::lower(statements, context)?,
            statements: ir::WorkbenchStatements::lower(statements, context)?,
            source: microcad_lang_base::Source {
                url: node.url.clone(),
                line_offset: node.line_offset,
                code: node.code.clone(),
            },
        })
    }
}
