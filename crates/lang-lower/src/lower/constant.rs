// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir, lower::extract_statements};

use microcad_lang_parse::ast;

impl Lower<ast::ConstAssignment> for ir::Constant {
    fn lower(node: &ast::ConstAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            attr: crate::lower::attribute::outer_with_doc(&node.doc, &node.attributes, context)?,
            visibility: ir::Visibility::lower(&node.visibility, context)?,
            keyword_src_ref: context.src_ref(&node.keyword_span),
            id: ir::Identifier::lower(&node.name, context)?,
            ty: Option::<ir::TypeAnnotation>::lower(&node.ty, context)?,
            expr: ir::ConstantExpression::lower(node.value.as_ref(), context)?,
        })
    }
}

impl Lower<ast::StatementList> for ir::Constants {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(extract_statements(node, |stmt| {
            Ok(match stmt {
                ast::Statement::Const(const_assignment) => {
                    Some(ir::Constant::lower(const_assignment, context)?)
                }
                _ => None,
            })
        })?))
    }
}
