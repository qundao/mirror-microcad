// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir, lower::extract_statements};

use microcad_lang_parse::ast;

impl Lower<ast::InlineModule> for ir::InlineModule {
    fn lower(node: &ast::InlineModule, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            outer_attr: crate::lower::attribute::outer_with_doc(
                &node.doc,
                &node.attributes,
                context,
            )?,
            visibility: ir::Visibility::lower(&node.visibility, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            id: ir::Identifier::lower(&node.name, context)?,
            inner_attr: ir::Attributes::lower(&node.body.statements, context)?,
            modules: ir::InlineModules::lower(&node.body.statements, context)?,
            aliases: ir::Aliases::lower(&node.body.statements, context)?,
            constants: ir::Constants::lower(&node.body.statements, context)?,
            functions: ir::Functions::lower(&node.body.statements, context)?,
            workbenches: ir::Workbenches::lower(&node.body.statements, context)?,
        })
    }
}

impl Lower<ast::StatementList> for ir::InlineModules {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(ir::InlineModules(extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::InlineModule(inline_module) => {
                    Some(ir::InlineModule::lower(inline_module, context)?)
                }
                _ => None,
            })
        })?))
    }
}
