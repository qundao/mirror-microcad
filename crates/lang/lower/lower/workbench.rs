// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};

use microcad_lang_base::Refer;
use microcad_syntax::ast;

impl From<ast::WorkbenchKind> for ir::WorkbenchKind {
    fn from(value: ast::WorkbenchKind) -> Self {
        match value {
            ast::WorkbenchKind::Sketch => ir::WorkbenchKind::Sketch,
            ast::WorkbenchKind::Part => ir::WorkbenchKind::Part,
            ast::WorkbenchKind::Op => ir::WorkbenchKind::Operation,
        }
    }
}
impl FromAst for std::rc::Rc<ir::WorkbenchDefinition> {
    type AstNode = ast::WorkbenchDefinition;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(std::rc::Rc::new(ir::WorkbenchDefinition {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: ir::DocBlock::from_ast(&node.doc, context)?,
            attribute_list: ir::AttributeList::from_ast(&node.attributes, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|v| ir::Visibility::from_ast(v, context))
                .transpose()?
                .unwrap_or_default(),
            kind: Refer::new(node.kind.into(), context.src_ref(&node.span)),
            id: ir::Identifier::from_ast(&node.name, context)?,
            plan: ir::ParameterList::from_ast(&node.plan, context)?,
            body: ir::Body::from_ast(&node.body, context)?,
        }))
    }
}
