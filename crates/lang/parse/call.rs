// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_syntax::ast;
use crate::{ord_map::*, parse::*, parser::*, syntax::*};

impl FromAst for Call {
    type AstNode = ast::Call;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Call {
            src_ref: context.src_ref(&node.span),
            name: QualifiedName::from_ast(&node.name, context)?,
            argument_list: ArgumentList::from_ast(&node.arguments, context)?,
        })
    }
}

impl FromAst for ArgumentList {
    type AstNode = ast::ArgumentList;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let mut argument_list = ArgumentList(Refer::new(OrdMap::default(), context.src_ref(&node.span)));
        for arg in &node.arguments {
            argument_list
                .try_push(Argument::from_ast(arg, context)?)
                .map_err(ParseError::DuplicateArgument)?;
        }
        Ok(argument_list)
    }
}

impl FromAst for Argument {
    type AstNode = ast::Argument;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Argument {
            id: node.name().map(|name| Identifier::from_ast(name, context)).transpose()?,
            src_ref: context.src_ref(node.span()),
            expression: Expression::from_ast(node.value(), context)?
        })
    }
}

impl FromAst for MethodCall {
    type AstNode = ast::Call;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(MethodCall {
            name: QualifiedName::from_ast(&node.name, context)?,
            argument_list: ArgumentList::from_ast(&node.arguments, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}
