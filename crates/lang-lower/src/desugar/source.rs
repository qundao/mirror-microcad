// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Desugar, LowerContext, LowerError, LowerResult, desugar::for_each_statement, ir};

use microcad_lang_base::{Identifier, SpanToSrcRef, SrcRef, element::Visibility};
use microcad_lang_parse::ast;

impl Desugar<ast::StatementList> for ir::desugared::SourceItems {
    fn desugar(statements: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
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
            file_modules: Box::desugar(statements, context)?,
            inline_modules: Box::desugar(statements, context)?,
            aliases: ir::desugared::Aliases::desugar(statements, context)?,
            constants: Box::desugar(statements, context)?,
            functions: Box::desugar(statements, context)?,
            workbenches: Box::desugar(statements, context)?,
        })
    }
}

impl Desugar<ast::Source> for ir::desugared::Source {
    fn desugar(node: &ast::Source, context: &mut LowerContext) -> LowerResult<Self> {
        let statements = &node.statements;

        Ok(Self {
            meta: ir::Meta {
                name: context.source.file_module_name().map(Identifier::from),
                vis: Visibility::Public,
                src_ref: context.span_to_src_ref(&node.span),
                keyword_src_ref: SrcRef::none(),
            },
            attr: ir::Attributes::desugar(statements, context)?,
            items: ir::desugared::SourceItems::desugar(statements, context)?,
            statements: Box::desugar(statements, context)?,
        })
    }
}
