// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};

use microcad_lang_base::Refer;
use microcad_lang_parse::ast;

impl From<ast::WorkbenchKind> for ir::WorkbenchKind {
    fn from(value: ast::WorkbenchKind) -> Self {
        match value {
            ast::WorkbenchKind::Sketch => ir::WorkbenchKind::Sketch,
            ast::WorkbenchKind::Part => ir::WorkbenchKind::Part,
            ast::WorkbenchKind::Op => ir::WorkbenchKind::Operation,
        }
    }
}
impl Lower for std::rc::Rc<ir::WorkbenchDefinition> {
    type AstNode = ast::WorkbenchDefinition;

    fn lower(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(std::rc::Rc::new(ir::WorkbenchDefinition {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: ir::DocBlock::lower(&node.doc, context)?,
            attribute_list: ir::AttributeList::lower(&node.attributes, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|v| ir::Visibility::lower(v, context))
                .transpose()?
                .unwrap_or_default(),
            kind: Refer::new(node.kind.into(), context.src_ref(&node.span)),
            id: ir::Identifier::lower(&node.name, context)?,
            plan: ir::ParameterList::lower(&node.plan, context)?,
            body: ir::Body::lower(&node.body, context)?,
        }))
    }
}
