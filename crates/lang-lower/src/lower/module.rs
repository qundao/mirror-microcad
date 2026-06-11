// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Lower, LowerContext, LowerResult, ir,
    lower::{attribute::outer_with_doc, extract_statements},
};

use microcad_lang_parse::ast;

impl Lower<ast::FileModule> for ir::FileModule {
    fn lower(node: &ast::FileModule, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            attr: outer_with_doc(&node.doc, &node.attributes, context)?,
            visibility: ir::Visibility::lower(&node.visibility, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            id: ir::Identifier::lower(&node.name, context)?,
        })
    }
}

impl Lower<ast::StatementList> for ir::FileModules {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::FileModule(file_module) => {
                    Some(ir::FileModule::lower(file_module, context)?)
                }
                _ => None,
            })
        })?))
    }
}

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
            inner_attr: ir::InnerAttributes::lower(&node.body.statements, context)?,
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
        Ok(Self(extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::InlineModule(inline_module) => {
                    Some(ir::InlineModule::lower(inline_module, context)?)
                }
                _ => None,
            })
        })?))
    }
}
