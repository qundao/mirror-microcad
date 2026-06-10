// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, LowerResult, ir};

use microcad_lang_base::Refer;
use microcad_lang_parse::ast;

impl Lower<ast::InitDefinition> for ir::Init {
    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            doc: ir::DocBlock::lower(&node.doc, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            parameters: ir::ParameterList::lower(&node.parameters, context)?,
            body: ir::Body::lower(&node.body, context)?,
            src_ref: context.src_ref(&node.span),
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
            aliases: ir::Aliases::lower(&node, context)?,
            constants: ir::Constants::lower(&node, context)?,
            inits: ir::Inits::lower(&node, context)?,
            statements: ir::WorkbenchStatements::lower(&node, context)?,
        })
    }
}
