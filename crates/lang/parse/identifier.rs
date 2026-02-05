// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, src_ref::*, syntax::*};
use microcad_syntax::ast;

impl Parse for IdentifierList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut vec = Vec::new();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::identifier {
                vec.push(Identifier::parse(pair)?);
            }
        }
        Ok(Self(Refer::new(vec, pair.into())))
    }
}

impl FromAst for Identifier {
    type AstNode = ast::Identifier;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(Self(Refer::new(node.name.clone(), context.src_ref(&node.span))))
    }
}

impl Parse for Identifier {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::identifier);
        Ok(Self(Refer::new(pair.as_str().into(), pair.into())))
    }
}

impl FromAst for QualifiedName {
    type AstNode = ast::QualifiedName;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let parts = node.parts.iter()
            .map(|ident| Identifier::from_ast(ident, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(parts, context.src_ref(&node.span)))
    }
}

impl Parse for QualifiedName {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self::new(
            pair.inner()
                .map(|pair| Identifier::parse(pair).expect("Expected identifier"))
                .collect(),
            pair.into(),
        ))
    }
}
