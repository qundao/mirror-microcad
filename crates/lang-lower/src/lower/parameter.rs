// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir, lower::sort_and_check};

use microcad_lang_base::Refer;
use microcad_lang_parse::ast;

impl Lower<ast::Parameter> for ir::Parameter {
    fn lower(node: &ast::Parameter, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: crate::lower::attribute::outer_with_doc(&node.doc, &node.attr, context)?,
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
                .map(|def| ir::ConstantExpression::lower(def, context))
                .transpose()?,
        })
    }
}

impl Lower<ast::ParameterList> for ir::ParameterList {
    fn lower(node: &ast::ParameterList, context: &mut LowerContext) -> LowerResult<Self> {
        let mut parameters = Vec::new();

        for param in &node.parameters {
            parameters.push(ir::Parameter::lower(param, context)?);
        }

        Ok(ir::ParameterList(Refer::new(
            sort_and_check(parameters, context)?,
            context.src_ref(&node.span),
        )))
    }
}
