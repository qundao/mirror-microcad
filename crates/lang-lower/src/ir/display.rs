// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display implementations of IR items

use crate::ir;

impl<Expr: ir::ExprSpec> std::fmt::Display for ir::ArgumentList<Expr>
where
    Expr: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        })
    }
}

impl std::fmt::Display for ir::DocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.content
            .lines()
            .try_for_each(|line| writeln!(f, "/// {line}"))
    }
}

impl std::fmt::Display for ir::Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{id}: {ty}", id = self.id, ty = self.ty)?;
        match &self.default_value {
            Some(v) => write!(f, " = {v}"),
            _ => Ok(()),
        }
    }
}

impl std::fmt::Display for ir::ParameterList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.parameters
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl<Expr> std::fmt::Display for ir::If<Expr>
where
    Expr: ir::ExprSpec + std::fmt::Display,
    Expr::Body: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "if {cond} {body}", cond = self.cond, body = self.body)?;
        if let Some(next) = &self.next_if {
            writeln!(f, "else {next}")?;
        }
        if let Some(body) = &self.body_else {
            writeln!(f, "else {body}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for ir::ConstantExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ir::ConstantExpression::Value(literal) => write!(f, "{literal}"),
            ir::ConstantExpression::Path(path) => write!(f, "{path}"),
            _ => unimplemented!(),
        }
    }
}

impl<Expr> std::fmt::Display for ir::LocalAssignment<Expr>
where
    Expr: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{id}: {ty} = {expr}",
            id = self.id,
            ty = self.ty,
            expr = self.expression
        )
    }
}

impl std::fmt::Display for ir::FunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}){}",
            self.parameters,
            if let Some(ret) = &self.return_type {
                format!("-> {ret}")
            } else {
                String::default()
            }
        )
    }
}
