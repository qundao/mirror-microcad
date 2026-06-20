// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir};

use microcad_lang_parse::ast;

impl Lower<ast::def::Constant> for ir::Constant {
    fn lower(node: &ast::def::Constant, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            attr: crate::lower::attribute::outer_with_doc(&node.doc, &node.attr, context)?,
            visibility: ir::Visibility::lower(&node.vis, context)?,
            keyword_src_ref: context.src_ref(&node.keyword_span),
            id: ir::Identifier::lower(&node.id, context)?,
            ty: Option::<ir::TypeAnnotation>::lower(&node.ty, context)?,
            expr: ir::ConstantExpression::lower(node.expr.as_ref(), context)?,
        })
    }
}

impl Lower<ast::Statement> for Option<ir::Constant> {
    fn lower(stmt: &ast::Statement, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match stmt {
            ast::Statement::Const(const_assignment) => {
                Some(ir::Constant::lower(const_assignment, context)?)
            }
            _ => None,
        })
    }
}
