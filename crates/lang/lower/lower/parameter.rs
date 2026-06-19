// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::{Lower, LowerContext, LowerError, ir};

use microcad_lang_base::{OrdMap, Refer};
use microcad_lang_parse::ast;

impl Lower for ir::Parameter {
    type AstNode = ast::Parameter;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::Parameter {
            src_ref: context.src_ref(&node.span),
            id: ir::Identifier::lower(&node.id, context)?,
            specified_type: node
                .ty
                .as_ref()
                .map(|ty| ir::TypeAnnotation::lower(ty, context))
                .transpose()?,
            default_value: node
                .default
                .as_ref()
                .map(|def| ir::Expression::lower(def, context))
                .transpose()?,
        })
    }
}

impl Lower for ir::ParameterList {
    type AstNode = ast::ParameterList;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        let mut parameters: OrdMap<_, _> = Default::default();

        for param in &node.parameters {
            let param = ir::Parameter::lower(param, context)?;
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
