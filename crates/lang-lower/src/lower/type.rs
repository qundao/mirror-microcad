// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir};

use microcad_lang_base::Refer;
use microcad_lang_parse::ast;
use microcad_lang_types::{Type, ty};

impl Lower for Type {
    type AstNode = ast::Type;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> LowerResult<Self> {
        use std::str::FromStr;
        Ok(match node {
            ast::Type::Single(ty) => Type::from_str(ty.name.as_str())
                .map_err(|err| Refer::new(err, context.src_ref(&node.span())))?,
            ast::Type::Array(ty) => Type::Array(Box::new(Type::lower(&ty.inner, context)?)),
            ast::Type::Tuple(ty) => Type::Tuple(Box::new(ty::TupleType::lower(ty, context)?)),
        })
    }
}

impl Lower for ir::TypeAnnotation {
    type AstNode = ast::Type;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(ir::TypeAnnotation(Refer::new(
            Type::lower(node, context)?,
            context.src_ref(&node.span()),
        )))
    }
}
