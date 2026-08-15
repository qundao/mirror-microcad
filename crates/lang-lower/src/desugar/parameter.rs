// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Desugar, LowerContext, LowerResult, desugar::sort_and_check, ir};

use microcad_lang_base::SpanToSrcRef;
use microcad_lang_parse::ast;

impl Desugar<ast::Parameter> for ir::Parameter {
    fn desugar(node: &ast::Parameter, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            attr: crate::desugar::attribute::outer_with_doc(&node.doc, &node.attr, context)?,
            src_ref: context.span_to_src_ref(&node.span),
            id: ir::Identifier::desugar(&node.id, context)?,
            ty: ir::Type::desugar(&node.ty, context)?,
            default_value: node
                .default
                .as_ref()
                .map(|def| ir::ConstantExpression::desugar(def, context))
                .transpose()?,
        })
    }
}

impl Desugar<ast::ParameterList> for ir::ParameterList {
    fn desugar(node: &ast::ParameterList, context: &mut LowerContext) -> LowerResult<Self> {
        let mut parameters = Vec::new();

        for param in &node.parameters {
            parameters.push(ir::Parameter::desugar(param, context)?);
        }

        Ok(ir::ParameterList {
            parameters: sort_and_check(parameters, context)?,
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}
