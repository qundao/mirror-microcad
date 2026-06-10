// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir};

use microcad_lang_base::{OrdMap, PushDiag, Refer, SrcReferrer};
use microcad_lang_parse::ast;

impl Lower<ast::Parameter> for ir::Parameter {
    fn lower(node: &ast::Parameter, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: crate::lower::attribute::outer_with_doc(&node.doc, &node.attributes, context)?,
            src_ref: context.src_ref(&node.span),
            id: ir::Identifier::lower(&node.name, context)?,
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
        let mut parameters: OrdMap<_, _> = Default::default();

        for param in &node.parameters {
            let param = ir::Parameter::lower(param, context)?;
            match parameters.push(param) {
                Some(param) => {
                    context
                        .diagnostics
                        .error(&param.src_ref(), miette::miette!("Duplicated parameter"))
                        .ok();
                }
                None => {} // Ok
            }
        }

        Ok(ir::ParameterList(Refer::new(
            parameters,
            context.src_ref(&node.span),
        )))
    }
}
