// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};
use microcad_syntax::ast;

impl From<ast::WorkbenchKind> for WorkbenchKind {
    fn from(value: ast::WorkbenchKind) -> Self {
        match value {
            ast::WorkbenchKind::Sketch => WorkbenchKind::Sketch,
            ast::WorkbenchKind::Part => WorkbenchKind::Part,
            ast::WorkbenchKind::Op => WorkbenchKind::Operation,
        }
    }
}
impl FromAst for Rc<WorkbenchDefinition> {
    type AstNode = ast::WorkbenchDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Rc::new(WorkbenchDefinition {
            keyword_ref: context.src_ref(&node.keyword_span),
            doc: node
                .doc
                .as_ref()
                .map(|doc| DocBlock::from_ast(doc, context))
                .transpose()?,
            attribute_list: AttributeList::from_ast(&node.attributes, context)?,
            visibility: node
                .visibility
                .as_ref()
                .map(|v| Visibility::from_ast(v, context))
                .transpose()?
                .unwrap_or_default(),
            kind: Refer::new(node.kind.into(), context.src_ref(&node.span)),
            id: Identifier::from_ast(&node.name, context)?,
            plan: ParameterList::from_ast(&node.arguments, context)?,
            body: Body::from_ast(&node.body, context)?,
        }))
    }
}
