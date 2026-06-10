// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir, lower::extract_statements};

use microcad_lang_base::Identifier;
use microcad_lang_parse::ast;

impl Lower<ast::FunctionDefinition> for ir::Attributes {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        crate::lower::attribute::outer(&node.doc, &node.attributes, context)
    }
}

impl Lower<ast::FunctionDefinition> for ir::FunctionSignature {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            parameters: ir::ParameterList::lower(&node.parameters, context)?,
            return_type: Option::<ir::TypeAnnotation>::lower(&node.return_type, context)?,
        })
    }
}

impl Lower<ast::FunctionDefinition> for ir::Constants {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(
            extract_statements(&node.body.statements, |stmt| match stmt {
                ast::Statement::Const(const_assignment) => {
                    Some(ir::Constant::lower(&const_assignment, context))
                }
                _ => None,
            })?
            .into_boxed_slice(),
        ))
    }
}

impl Lower<ast::FunctionDefinition> for ir::Aliases {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(
            extract_statements(&node.body.statements, |stmt| match stmt {
                ast::Statement::Use(const_assignment) => {
                    Some(ir::Constant::lower(&const_assignment, context))
                }
                _ => None,
            })?
            .into_boxed_slice(),
        ))
    }
}

impl Lower<ast::FunctionDefinition> for Vec<ir::FunctionStatement> {}

impl Lower<ast::FunctionDefinition> for ir::Function {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            src_ref: context.src_ref(&node.span),
            outer_attr: ir::Attributes::lower(node, context)?,
            visibility: ir::Visibility::lower(&node.visibility, context)?,
            keyword_ref: context.src_ref(&node.keyword_span),
            id: Identifier::lower(&node.name, context)?,
            signature: ir::FunctionSignature::lower(&node, context)?,
            inner_attr: ir::Attributes::lower(&node.body.statements, context)?,
            aliases: ir::Aliases::lower(&node, context)?,
            constants: ir::Constants::lower(&node, context)?,
            statements: Vec::<ir::FunctionStatement>::lower(&node, context)?,
        })
    }
}
