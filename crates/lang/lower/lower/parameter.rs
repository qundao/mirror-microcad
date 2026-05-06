// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{FromAst, LowerContext, LowerError, ir};

use microcad_lang_base::{OrdMap, Refer};
use microcad_syntax::ast;

impl FromAst for ir::Parameter {
    type AstNode = ast::Parameter;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Parameter {
            src_ref: context.src_ref(&node.span),
            id: ir::Identifier::from_ast(&node.name, context)?,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| ir::TypeAnnotation::from_ast(ty, context))
                .transpose()?,
            default_value: node
                .default
                .as_ref()
                .map(|def| ir::Expression::from_ast(def, context))
                .transpose()?,
        })
    }
}

impl FromAst for ir::ParameterList {
    type AstNode = ast::ParameterList;

    fn from_ast(node: &Self::AstNode, context: &LowerContext) -> Result<Self, LowerError> {
        let mut parameters: OrdMap<_, _> = Default::default();

        for param in &node.parameters {
            let param = ir::Parameter::from_ast(param, context)?;
            parameters
                .try_push(param)
                .map_err(|(previous, id)| LowerError::DuplicateArgument { previous, id })?;
        }
        Ok(ir::ParameterList(Refer::new(
            parameters,
            context.src_ref(&node.span),
        )))
    }
}
