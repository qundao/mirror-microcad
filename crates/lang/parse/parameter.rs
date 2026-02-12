// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{ord_map::*, parse::*, parser::*};
use microcad_syntax::ast;

impl FromAst for Parameter {
    type AstNode = ast::ArgumentDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Parameter {
            src_ref: context.src_ref(&node.span),
            id: Identifier::from_ast(&node.name, context)?,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| TypeAnnotation::from_ast(ty, context))
                .transpose()?,
            default_value: node
                .default
                .as_ref()
                .map(|def| Expression::from_ast(def, context))
                .transpose()?,
        })
    }
}

impl FromAst for ParameterList {
    type AstNode = ast::ArgumentsDefinition;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let mut parameters: OrdMap<_, _> = Default::default();

        for param in &node.arguments {
            let param = Parameter::from_ast(param, context)?;
            parameters
                .try_push(param)
                .map_err(|(previous, id)| ParseError::DuplicateArgument {
                    previous,
                    id,
                })?;
        }
        Ok(ParameterList(Refer::new(
            parameters,
            context.src_ref(&node.span),
        )))
    }
}
