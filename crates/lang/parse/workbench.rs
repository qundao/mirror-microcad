// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};
use microcad_syntax::ast;

impl Parse for Refer<WorkbenchKind> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.as_str() {
            "part" => Ok(Refer::new(WorkbenchKind::Part, pair.into())),
            "sketch" => Ok(Refer::new(WorkbenchKind::Sketch, pair.into())),
            "op" => Ok(Refer::new(WorkbenchKind::Operation, pair.into())),
            _ => Err(ParseError::UnexpectedToken(pair.into())),
        }
    }
}

impl Parse for Rc<WorkbenchDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(WorkbenchDefinition {
            doc: crate::find_rule_opt!(pair, doc_block)?,
            visibility: crate::find_rule!(pair, visibility)?,
            attribute_list: crate::find_rule!(pair, attribute_list)?,
            kind: crate::find_rule_exact!(pair, workbench_kind)?,
            id: crate::find_rule!(pair, identifier)?,
            plan: crate::find_rule!(pair, parameter_list)?,
            body: crate::find_rule!(pair, body)?,
        }
        .into())
    }
}

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
