// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::{HashMap, HashSet};
use crate::{parse::*, parser::*, syntax::*, ty::*};
use microcad_syntax::ast;

impl Parse for TupleType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        use crate::ty::Ty;
        Parser::ensure_rule(&pair, Rule::tuple_type);

        match pair.as_str() {
            "Color" => Ok(TupleType::new_color()),
            "Vec2" => Ok(TupleType::new_vec2()),
            "Vec3" => Ok(TupleType::new_vec3()),
            "Size2" => Ok(TupleType::new_size2()),
            _ => {
                let mut named = std::collections::HashMap::new();
                let mut unnamed = std::collections::HashSet::new();

                pair.inner().try_for_each(|pair| {
                    let mut inner = pair.inner();
                    let next = inner.next().expect("Identifier or type expected");
                    if next.as_rule() == Rule::identifier {
                        let id = Identifier::parse(next)?;
                        if named
                            .insert(
                                id.clone(),
                                TypeAnnotation::parse(
                                    inner.next().expect("Identifier or type expected"),
                                )?
                                .ty(),
                            )
                            .is_some()
                        {
                            return Err(ParseError::DuplicateTupleIdentifier(id));
                        }
                    } else {
                        let ty = TypeAnnotation::parse(next)?.ty();
                        if !unnamed.insert(ty.clone()) {
                            return Err(ParseError::DuplicateTupleType(Refer::new(
                                ty,
                                pair.clone().into(),
                            )));
                        }
                    }

                    Ok::<(), ParseError>(())
                })?;

                Ok(Self { named, unnamed })
            }
        }
    }
}

impl FromAst for TupleType {
    type AstNode = ast::TupleType;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(TupleType {
            named: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.as_ref().map(|name| (name, value)))
                .map(|(name, value)| {
                    let name = Identifier::from_ast(name, context)?;
                    let value = Type::from_ast(value, context)?;
                    Ok((name, value))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
            unnamed: node
                .inner
                .iter()
                .filter_map(|(name, value)| name.is_none().then_some(value))
                .map(|value| Type::from_ast(value, context))
                .collect::<Result<HashSet<_>, _>>()?,
        })
    }
}

impl Parse for MatrixType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::matrix_type);

        let mut m: Option<usize> = None;
        let mut n: Option<usize> = None;

        for p in pair.inner() {
            match p.as_rule() {
                Rule::int => match m {
                    None => m = Some(p.as_str().parse().expect("Valid integer")),
                    Some(_) => n = Some(p.as_str().parse().expect("Valid integer")),
                },
                _ => unreachable!(),
            }
        }

        let m = m.expect("M");

        Ok(Self {
            rows: m,
            columns: n.unwrap_or(m),
        })
    }
}

#[test]
fn array_type() {
    use crate::parser::{Parser, Rule};
    use crate::ty::Ty;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "[Integer]", 0).expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "[Integer]");
    assert_eq!(type_annotation.ty(), Type::Array(Box::new(Type::Integer)));
}

#[test]
fn matrix_type() {
    use crate::parser::*;
    use crate::ty::Ty;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "Matrix4x3", 0).expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "Matrix4x3");
    assert_eq!(
        type_annotation.ty(),
        Type::Matrix(MatrixType {
            rows: 4,
            columns: 3,
        })
    );
}
