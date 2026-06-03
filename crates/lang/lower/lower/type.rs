// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    lower::{Lower, LowerContext, LowerError, LowerResult, ir},
    ty::*,
};

use microcad_lang_base::{Refer, SrcRef};
use microcad_lang_parse::ast;
use std::str::FromStr;

impl Type {
    /// Parse a type from &str
    pub fn parse_str(ty: &str, src_ref: SrcRef) -> LowerResult<Self> {
        if let Some(dimensions) = ty.strip_prefix("Matrix") {
            let (x, y) = dimensions
                .split_once('x')
                .unwrap_or((dimensions, dimensions));
            let x = usize::from_str(x).map_err(|_| {
                LowerError::InvalidMatrixType(Refer::new(ty.to_string(), src_ref.clone()))
            })?;
            let y = usize::from_str(y).map_err(|_| {
                LowerError::InvalidMatrixType(Refer::new(ty.to_string(), src_ref.clone()))
            })?;
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
            "Model" => Ok(Type::Model),
            _ => Err(LowerError::UnknownType(Refer::new(ty.to_string(), src_ref))),
        }
    }
}

impl Lower for ir::Type {
    type AstNode = ast::Type;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(match node {
            ast::Type::Single(ty) => {
                Type::parse_str(ty.name.as_str(), context.src_ref(&node.span()))?
            }
            ast::Type::Array(ty) => Type::Array(Box::new(Type::lower(&ty.inner, context)?)),
            ast::Type::Tuple(ty) => Type::Tuple(Box::new(TupleType::lower(ty, context)?)),
        })
    }
}

impl Lower for ir::TypeAnnotation {
    type AstNode = ast::Type;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError> {
        Ok(ir::TypeAnnotation(Refer::new(
            Type::lower(node, context)?,
            context.src_ref(&node.span()),
        )))
    }
}
