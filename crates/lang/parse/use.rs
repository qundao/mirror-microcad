// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};
use microcad_syntax::ast;
use microcad_syntax::ast::UseStatementPart;

impl Parse for UseDeclaration {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::use_declaration);

        let mut inner = pair.inner();
        let first = inner.next().expect("Expected use declaration element");

        match first.as_rule() {
            Rule::qualified_name => Ok(Self::Use(QualifiedName::parse(first)?)),
            Rule::use_all => {
                let inner = first.inner().next().expect("Expected qualified name");
                Ok(Self::UseAll(QualifiedName::parse(inner)?))
            }
            Rule::use_alias => {
                let mut inner = first.inner();
                let name = QualifiedName::parse(inner.next().expect("Expected qualified name"))?;
                let alias = Identifier::parse(inner.next().expect("Expected identifier"))?;
                Ok(Self::UseAs(name, alias))
            }
            _ => unreachable!("Invalid use declaration"),
        }
    }
}

impl FromAst for UseStatement {
    type AstNode = ast::UseStatement;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        let glob_index = node
            .name
            .parts
            .iter()
            .enumerate()
            .find(|(_, part)| matches!(part, UseStatementPart::Glob(_)))
            .map(|(i, _)| i);
        if let Some(i) = glob_index {
            if i < node.name.parts.len() - 1 {
                return Err(ParseError::InvalidGlobPattern(
                    context.src_ref(&node.name.span),
                ));
            }
        }
        let name = node
            .name
            .parts
            .iter()
            .filter_map(|part| match part {
                UseStatementPart::Identifier(ident) => Some(Identifier::from_ast(ident, context)),
                UseStatementPart::Glob(_) => None,
            })
            .collect::<Result<Vec<_>, _>>()?;
        let name = QualifiedName::new(name, context.src_ref(&node.name.span));

        let decl = match (glob_index.is_some(), &node.use_as) {
            (false, None) => UseDeclaration::Use(name),
            (true, None) => UseDeclaration::UseAll(name),
            (true, Some(_)) => {
                return Err(ParseError::UseGlobAlias(context.src_ref(&node.span)));
            }
            (false, Some(alias)) => {
                UseDeclaration::UseAs(name, Identifier::from_ast(alias, context)?)
            }
        };
        let visibility = node
            .visibility
            .as_ref()
            .map(|visibility| Visibility::from_ast(visibility, context))
            .transpose()?;
        Ok(UseStatement {
            src_ref: context.src_ref(&node.span),
            visibility: visibility.unwrap_or_default(),
            decl,
        })
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::use_statement);

        Ok(Self {
            visibility: crate::find_rule!(pair, visibility)?,
            decl: crate::find_rule_exact!(pair, use_declaration)?,
            src_ref: pair.into(),
        })
    }
}

impl Parse for Visibility {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::visibility);

        let s = pair.as_str();
        match s {
            "pub" => Ok(Self::Public),
            "" => Ok(Self::Private),
            _ => unreachable!("Invalid visibility"),
        }
    }
}

impl FromAst for Visibility {
    type AstNode = ast::Visibility;

    fn from_ast(node: &Self::AstNode, _context: &ParseContext) -> Result<Self, ParseError> {
        Ok(match node {
            ast::Visibility::Public => Self::Public,
        })
    }
}
