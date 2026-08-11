// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluate function

use microcad_builtin::Builtin;
use microcad_lang_base::{SrcRef, SrcReferrer, ToCompactString};
use microcad_lang_types::{ArgumentValue, ArgumentValueList, Value, tuple};
use microcad_package::rst;

use crate::{
    CallTrait, Eval, EvalContext, EvalError, EvalResult,
    context::{FunctionFrame, FunctionScopeFrame},
    find_match,
};

pub enum FlowSignal {
    /// When encountering a `Call` or `LocalAssignment` statement
    Continue,
    /// When encountering a `Body` or `If` statement or single expression
    Yield(Value),
    /// When encountering a `Return` statement
    Return(Value),
}

impl FlowSignal {
    /// Extract the inner `Value` if the signal is `Break` or `Return`.
    pub fn into_value(self) -> Value {
        match self {
            FlowSignal::Yield(val) | FlowSignal::Return(val) => val,
            FlowSignal::Continue => Value::None,
        }
    }

    /// Unwrap the inner `Value` or return an evaluation error if it was `Continue`.
    pub fn expect_value(self, src_ref: SrcRef) -> Result<Value, EvalError> {
        match self {
            FlowSignal::Yield(val) | FlowSignal::Return(val) => Ok(val),
            FlowSignal::Continue => Err(EvalError::ExpectedExpression { src_ref }),
        }
    }
}

impl Eval<ArgumentValue> for rst::function::Argument {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ArgumentValue> {
        Ok(match self {
            rst::function::Argument::Unnamed(expr) => {
                ArgumentValue::new(expr.eval(context)?.into_value(), None)
            }
            rst::function::Argument::Named { name, expr, .. }
            | rst::function::Argument::AutoNamed { name, expr } => {
                ArgumentValue::new(expr.eval(context)?.into_value(), Some(name.clone()))
            }
        })
    }
}

impl Eval<ArgumentValueList> for rst::function::ArgumentList {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ArgumentValueList> {
        let map: Vec<ArgumentValue> = self
            .args
            .iter()
            .map(|arg| arg.eval(context))
            .collect::<EvalResult<Vec<_>>>()?;

        Ok(ArgumentValueList {
            args: map,
            src_ref: self.src_ref,
        })
    }
}

impl Eval<FlowSignal> for rst::function::Scope {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        context.scope(FunctionScopeFrame::new(), |context| {
            self.statements.eval(context)
        })
    }
}

impl Eval<FlowSignal> for rst::function::If {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        // 1. Evaluate condition expression
        let cond_signal = self.cond.eval(context)?;
        let cond_val: bool = cond_signal.expect_value(self.src_ref)?.try_into()?;

        // 2. Dispatch based on condition boolean
        if cond_val {
            self.body.eval(context)
        } else if let Some(next_if) = &self.next_if {
            // Handle `else if ...` chain
            next_if.eval(context)
        } else if let Some(body_else) = &self.body_else {
            // Handle `else { ... }` block
            body_else.eval(context)
        } else {
            // If condition is false and no else/else-if branch exists,
            // continue execution without yielding a value.
            Ok(FlowSignal::Continue)
        }
    }
}

impl Eval<FlowSignal> for rst::function::Call {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        match &self.path {
            rst::Path::Resolved(rst::SymbolId::Builtin(builtin_id)) => {
                let args = self.args.eval(context)?;

                match context.builtins.get(*builtin_id) {
                    Some(Builtin::Function(f)) => {
                        let args = find_match(&args, &f.ty(), &tuple!())?;

                        Ok(FlowSignal::Yield(f.call_isolated(args)?))
                    }
                    None => unimplemented!("Function not found"),
                    _ => todo!(),
                }
            }
            path => {
                context.diag(EvalError::SymbolCannotBeCalled {
                    path: path.to_string(),
                    src_ref: self.src_ref,
                });
                Ok(FlowSignal::Continue)
            }
        }
    }
}

impl Eval<FlowSignal> for rst::Path {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        match self {
            rst::Path::Resolved(rst::SymbolId::Local(identifier)) => {
                use crate::context::StackRead;
                match context.get_local(identifier) {
                    Some(local) => Ok(FlowSignal::Yield(local.clone())),
                    None => {
                        context.diag(EvalError::LocalNotFound(identifier.clone()));
                        Ok(FlowSignal::Continue)
                    }
                }
            }
            rst::Path::Resolved(rst::SymbolId::Builtin(builtin)) => {
                match context.builtins.get(*builtin) {
                    Some(Builtin::Constant(c)) => Ok(FlowSignal::Yield(c.value())),
                    _ => todo!("Error handling"),
                }
            }
            _ => todo!("Error handling"),
        }
    }
}

impl Eval<FlowSignal> for rst::FunctionExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        use rst::FunctionExpression as Expr;
        match self {
            Expr::Invalid => todo!("Error handling"),
            Expr::Literal(literal) => Ok(FlowSignal::Yield(literal.value().clone())),
            Expr::Path(name) => name.eval(context),
            Expr::Scope(s) => s.eval(context),
            Expr::If(if_) => if_.eval(context),
            Expr::Call(call) => call.eval(context),
            _ => todo!(),
        }
    }
}

impl Eval<FlowSignal> for rst::FunctionStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        use rst::FunctionStatement as Stmt;
        match self {
            Stmt::Scope(scope) => scope.eval(context),
            Stmt::Call(call) => {
                let value = call.eval(context)?.expect_value(call.src_ref)?;
                if !value.is_none() {
                    context.diag(EvalError::CallReturnValueIgnored(call.src_ref))
                }

                Ok(FlowSignal::Continue)
            }
            Stmt::Return(stmt) => match &stmt.expr {
                Some(expr) => Ok(FlowSignal::Return(
                    expr.eval(context)?.expect_value(self.src_ref())?,
                )),
                None => Ok(FlowSignal::Continue),
            },
            Stmt::Tail(stmt) => stmt.eval(context),
            Stmt::If(if_stmt) => if_stmt.eval(context),
            Stmt::Local(l) => {
                match l.expression.eval(context)? {
                    FlowSignal::Continue => {
                        context.diag(EvalError::LocalExpressionDidNotProduceAValue {
                            id: l.id.clone(),
                            src_ref: l.src_ref,
                            expr_src_ref: l.expression.src_ref(),
                        });

                        Ok(FlowSignal::Continue)
                    }
                    // Save yielded value in local table
                    FlowSignal::Yield(value) => {
                        context.top_mut().put_local(l.id.to_compact_string(), value);

                        Ok(FlowSignal::Continue)
                    }
                    // Return value immediately
                    FlowSignal::Return(value) => Ok(FlowSignal::Return(value)),
                }
            }
            _ => todo!(),
        }
    }
}

impl Eval<FlowSignal> for Box<[rst::FunctionStatement]> {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        for stmt in self {
            match stmt.eval(context) {
                Ok(FlowSignal::Continue) => {
                    continue;
                }
                ret => {
                    return ret;
                }
            }
        }

        Ok(FlowSignal::Continue)
    }
}

impl CallTrait<Value> for rst::Function {
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        match crate::find_match(args, &self.ty, &self.default_parameters) {
            Ok(args) => context.scope(FunctionFrame::new(args), |context| {
                Ok(self.statements.eval(context)?.into_value())
            }),
            Err(_) => todo!(),
        }
    }
}
