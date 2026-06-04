// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerError, ir};

use microcad_lang_parse::ast;

impl Lower for ir::UseStatement {
    type AstNode = ast::UseStatement;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        let glob_index = node
            .name
            .parts
            .iter()
            .enumerate()
            .find(|(_, part)| matches!(part, ast::UseStatementPart::Glob(_)))
            .map(|(i, _)| i);
        if let Some(i) = glob_index {
            if i < node.name.parts.len() - 1 {
                return Err(LowerError::InvalidGlobPattern(
                    context.src_ref(&node.name.span),
                ));
            }
        }
        let name = node
            .name
            .parts
            .iter()
            .filter_map(|part| match part {
                ast::UseStatementPart::Identifier(ident) => {
                    Some(ir::Identifier::lower(ident, context))
                }
                ast::UseStatementPart::Glob(_) => None,
                ast::UseStatementPart::Error(_) => None,
            })
            .collect::<Result<Vec<_>, _>>()?;
        let name = ir::QualifiedName::new(name, context.src_ref(&node.name.span));

        let decl = match (glob_index.is_some(), &node.use_as) {
            (false, None) => ir::UseDeclaration::Use(name),
            (true, None) => ir::UseDeclaration::UseAll(name),
            (true, Some(_)) => {
                return Err(LowerError::UseGlobAlias(context.src_ref(&node.span)));
            }
            (false, Some(alias)) => {
                ir::UseDeclaration::UseAs(name, ir::Identifier::lower(alias, context)?)
            }
        };
        let visibility = node
            .visibility
            .as_ref()
            .map(|visibility| ir::Visibility::lower(visibility, context))
            .transpose()?;
        Ok(ir::UseStatement {
            keyword_ref: context.src_ref(&node.keyword_span),
            src_ref: context.src_ref(&node.span),
            visibility: visibility.unwrap_or_default(),
            decl,
        })
    }
}

impl Lower for ir::Visibility {
    type AstNode = ast::Visibility;

    fn lower(node: &Self::AstNode, _context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(match node {
            ast::Visibility::Public => Self::Public,
        })
    }
}
