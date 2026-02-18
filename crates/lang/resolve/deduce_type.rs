use std::ops::{Div, Mul};

use crate::{builtin::Builtin, resolve::*, src_ref::*, syntax::*, ty::*};

/// Interface to deduce the result type of a statement
pub trait DeduceType {
    /// Deduce type of self.
    /// - `params`: Parameters and locals from outside.
    /// - `context`: Resolve context to fetch symbols
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type>;
}

impl DeduceType for Body {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        assert!(context.is_checked());

        let mut result = Type::Invalid;
        let mut result_src_ref = None;
        for statement in &self.0 {
            if let Some(result_src_ref) = result_src_ref {
                return Err(ResolveError::UnexpectedResult {
                    result: result_src_ref,
                    statement: statement.src_ref(),
                });
            }
            result = statement.deduce_type(params, context)?;
            if result != Type::Invalid {
                result_src_ref = Some(statement.src_ref());
            }
        }
        Ok(result)
    }
}

impl DeduceType for Statement {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        match self {
            Statement::Expression(expr) => expr.deduce_type(params, context),
            Statement::Workbench(wd) => wd.deduce_type(params, context),
            Statement::Function(f) => f.deduce_type(params, context),
            Statement::Init(id) => id.deduce_type(params, context),
            Statement::Return(rs) => rs.deduce_type(params, context),
            Statement::If(is) => is.deduce_type(params, context),
            Statement::Use(..)
            | Statement::Assignment(..)
            | Statement::Module(..)
            | Statement::InnerAttribute(..)
            | Statement::InnerDocComment(..) => Ok(Type::Invalid),
        }
    }
}

impl DeduceType for ExpressionStatement {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        self.expression.deduce_type(params, context)
    }
}

impl DeduceType for Expression {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        match &self {
            Expression::Invalid => Ok(Type::Invalid),
            Expression::Literal(l) => Ok(l.value().ty()),
            Expression::FormatString(..) => Ok(Type::String),
            Expression::ArrayExpression(ae) => ae.deduce_type(params, context),
            Expression::TupleExpression(te) => te.deduce_type(params, context),
            Expression::Body(b) => b.deduce_type(params, context),
            Expression::If(is) => is.deduce_type(params, context),
            Expression::Call(c) => c.deduce_type(params, context),
            Expression::QualifiedName(qn) => qn.deduce_type(params, context),
            Expression::Marker(m) => m.deduce_type(params, context),
            Expression::BinaryOp { lhs, op, rhs, .. } => match op.as_str() {
                "+" | "-" => lhs.deduce_type(params, context),
                "/" => Ok(lhs
                    .deduce_type(params, context)?
                    .div(rhs.deduce_type(params, context)?)),
                "*" => Ok(lhs
                    .deduce_type(params, context)?
                    .mul(rhs.deduce_type(params, context)?)),
                "<" | ">" | "≤" | "≥" | "&" | "|" => Ok(Type::Bool),
                _ => unreachable!(),
            },
            Expression::UnaryOp { rhs, .. } => rhs.deduce_type(params, context),
            Expression::ArrayElementAccess(expression, ..) => {
                if let Type::Array(ty) = expression.deduce_type(params, context)? {
                    return Ok(*ty);
                }
                unreachable!("no array type")
            }
            Expression::PropertyAccess(expression, identifier, src_ref) => todo!(),
            Expression::AttributeAccess(expression, identifier, src_ref) => todo!(),
            Expression::MethodCall(expression, method_call, src_ref) => todo!(),
        }
    }
}

impl DeduceType for ArrayExpression {
    fn deduce_type(&self, _: &mut ParameterList, _: &mut ResolveContext) -> ResolveResult<Type> {
        Ok(Type::Array(self.unit.ty().into()))
    }
}

impl DeduceType for TupleExpression {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        Ok(Type::Tuple(Box::new(
            self.args
                .iter()
                .map(|arg| match arg.expression.deduce_type(params, context) {
                    Ok(ty) => Ok((arg.id.clone().unwrap_or(Identifier::none()), ty)),
                    Err(err) => Err(err),
                })
                .collect::<ResolveResult<_>>()?,
        )))
    }
}

impl DeduceType for WorkbenchDefinition {
    fn deduce_type(
        &self,
        _: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        self.body.deduce_type(&mut self.plan.clone(), context)
    }
}

impl DeduceType for FunctionDefinition {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        self.body.deduce_type(params, context)
    }
}

impl DeduceType for InitDefinition {
    fn deduce_type(
        &self,
        _: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        self.body.deduce_type(&mut self.parameters.clone(), context)
    }
}

impl DeduceType for ReturnStatement {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        if let Some(result) = &self.result {
            result.deduce_type(params, context)
        } else {
            Ok(Type::Invalid)
        }
    }
}

impl DeduceType for IfStatement {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        let body = self.body.deduce_type(params, context)?;
        if let Some(body_else) = &self.body_else {
            if body != body_else.deduce_type(params, context)? {
                todo!("error")
            }
        }
        if let Some(next_if) = &self.next_if {
            if body != next_if.deduce_type(params, context)? {
                todo!("error")
            }
        }
        Ok(body)
    }
}

impl DeduceType for Call {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        context
            .root
            .lookup(&self.name, LookupTarget::Function)?
            .deduce_type(params, context)
    }
}

impl DeduceType for QualifiedName {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        todo!()
    }
}

impl DeduceType for Marker {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        todo!()
    }
}

impl DeduceType for Symbol {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        self.with_def(|def| match def {
            SymbolDef::Workbench(wd) => wd.deduce_type(params, context),
            SymbolDef::Function(fd) => fd.deduce_type(params, context),
            SymbolDef::Builtin(bi) => todo!(),
            SymbolDef::Constant(.., v) | SymbolDef::Argument(.., v) => todo!(),
            SymbolDef::Root
            | SymbolDef::SourceFile(..)
            | SymbolDef::Module(..)
            | SymbolDef::Alias(..)
            | SymbolDef::UseAll(..)
            | SymbolDef::Assignment(..) => Ok(Type::Invalid),

            #[cfg(test)]
            SymbolDef::Tester(identifier) => todo!(),
        })
    }
}

impl DeduceType for Builtin {
    fn deduce_type(
        &self,
        params: &mut ParameterList,
        context: &mut ResolveContext,
    ) -> ResolveResult<Type> {
        match &self.kind {
            crate::builtin::BuiltinKind::Function => todo!(),
            crate::builtin::BuiltinKind::Workbench(..) => Ok(Type::Model),
        }
    }
}
