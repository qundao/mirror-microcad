// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, src_ref::*, ty::*};
use microcad_syntax::ast;
use std::str::FromStr;

impl Type {
    fn parse_str(ty: &str, src_ref: SrcRef) -> ParseResult<Self> {
        if let Some(dimensions) = ty.strip_prefix("Matrix") {
            let (x, y) = dimensions
                .split_once('x')
                .unwrap_or((dimensions, dimensions));
            let x = usize::from_str(x)
                .map_err(|_| ParseError::InvalidMatrixType(Refer::new(ty.to_string(), src_ref.clone())))?;
            let y = usize::from_str(y)
                .map_err(|_| ParseError::InvalidMatrixType(Refer::new(ty.to_string(), src_ref.clone())))?;
            return Ok(Type::Matrix(MatrixType::new(x, y)));
        }

        match ty {
            "Color" => Ok(Type::Tuple(Box::new(TupleType::new_color()))),
            "Vec2" => Ok(Type::Tuple(Box::new(TupleType::new_vec2()))),
            "Vec3" => Ok(Type::Tuple(Box::new(TupleType::new_vec3()))),
            "Size2" => Ok(Type::Tuple(Box::new(TupleType::new_size2()))),
            "Integer" => Ok(Type::Integer),
            "Bool" => Ok(Type::Bool),
            "String" => Ok(Type::String),
            "Scalar" => Ok(Type::Quantity(QuantityType::Scalar)),
            "Length" => Ok(Type::Quantity(QuantityType::Length)),
            "Area" => Ok(Type::Quantity(QuantityType::Area)),
            "Angle" => Ok(Type::Quantity(QuantityType::Angle)),
            "Volume" => Ok(Type::Quantity(QuantityType::Volume)),
            "Weight" => Ok(Type::Quantity(QuantityType::Weight)),
            "Density" => Ok(Type::Quantity(QuantityType::Density)),
            _ => Err(ParseError::UnknownType(Refer::new(ty.to_string(), src_ref))),
        }
    }
}

impl Parse for Type {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::r#type);
        let inner = pair.inner().next().expect("Expected type");

        match inner.as_rule() {
            Rule::array_type => Ok(Type::Array(Box::new(Type::parse(
                inner.inner().next().expect("Type"),
            )?))),
            Rule::tuple_type => Ok(Type::Tuple(TupleType::parse(inner)?.into())),
            Rule::matrix_type => Ok(Type::Matrix(MatrixType::parse(inner)?)),
            Rule::quantity_type => Ok(Type::Quantity(QuantityType::parse(inner)?)),
            Rule::base_type => match inner.as_str() {
                // Builtin types.
                "Integer" => Ok(Type::Integer),
                "String" => Ok(Type::String),
                "Bool" => Ok(Type::Bool),
                "Model" => Ok(Type::Model),
                _ => Err(ParseError::UnknownType(Refer::new(
                    inner.to_string(),
                    pair.into(),
                ))),
            },
            _ => Err(ParseError::UnknownType(Refer::new(
                inner.to_string(),
                pair.into(),
            ))),
        }
    }
}

impl FromAst for Type {
    type AstNode = ast::Type;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(match node {
            ast::Type::Single(ty) => {
                Type::parse_str(ty.name.as_str(), context.src_ref(&node.span()))?
            }
            ast::Type::Array(ty) => Type::Array(Box::new(Type::from_ast(&ty.inner, context)?)),
            ast::Type::Tuple(ty) => Type::Tuple(Box::new(TupleType::from_ast(ty, context)?)),
        })
    }
}

impl Parse for QuantityType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::quantity_type);
        Ok(match pair.as_str() {
            "Scalar" => QuantityType::Scalar,
            "Length" => QuantityType::Length,
            "Area" => QuantityType::Area,
            "Angle" => QuantityType::Angle,
            "Volume" => QuantityType::Volume,
            "Weight" => QuantityType::Weight,
            "Density" => QuantityType::Density,
            _ => unreachable!("Expected type, found {:?}", pair.as_str()),
        })
    }
}

impl Parse for TypeAnnotation {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self(Refer::new(Type::parse(pair.clone())?, pair.into())))
    }
}

impl FromAst for TypeAnnotation {
    type AstNode = ast::Type;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError> {
        Ok(TypeAnnotation(Refer::new(Type::from_ast(node, context)?, context.src_ref(&node.span()))))
    }
}

#[test]
fn named_tuple_type() {
    use crate::parser::*;
    use crate::ty::Ty;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "(x: Integer, y: String)", 0)
            .expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "(x: Integer, y: String)");
    assert_eq!(
        type_annotation.ty(),
        Type::Tuple(
            TupleType {
                named: [("x", Type::Integer), ("y", Type::String)]
                    .into_iter()
                    .map(|(id, ty)| (id.into(), ty))
                    .collect(),
                ..Default::default()
            }
            .into()
        )
    );
}
