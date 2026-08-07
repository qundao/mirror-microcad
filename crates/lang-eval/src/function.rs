// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluate function

use microcad_builtin::{Builtin, BuiltinEvalContext};
use microcad_lang_base::{SrcRef, SrcReferrer};
use microcad_lang_types::{ArgumentValue, ArgumentValueList, Array, Type, Value, tuple};
use microcad_package::{builtin, rst};

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

pub fn builtin_range(
    call: &rst::function::Call,
    context: &mut EvalContext,
) -> EvalResult<FlowSignal> {
    let args = &call.args;
    let first_arg = args.args.first().unwrap();
    let last_arg = args.args.last().unwrap();

    match (
        first_arg.eval(context)?.value,
        last_arg.eval(context)?.value,
    ) {
        (Value::Integer(first), Value::Integer(last)) => {
            if first > last {
                context.diag(EvalError::BadRange {
                    first,
                    last,
                    src_ref: call.src_ref,
                });
            }

            let first: i64 = first.into();
            let last: i64 = last.into();

            Ok(FlowSignal::Yield(Value::Array(Array::new(
                (first..last + 1)
                    .map(|i| Value::Integer(i.into()))
                    .collect(),
                Type::Integer,
            ))))
        }
        (first, last) => {
            if !matches!(first, Value::Integer(_)) {
                context.diag(EvalError::InvalidRangeBoundaryType {
                    src_ref: first_arg.src_ref(),
                });
            }
            if !matches!(last, Value::Integer(_)) {
                context.diag(EvalError::InvalidRangeBoundaryType {
                    src_ref: last_arg.src_ref(),
                });
            }

            Ok(FlowSignal::Continue)
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
        let cond_val = cond_signal.expect_value(self.src_ref)?;

        // 2. Dispatch based on condition boolean
        if cond_val.as_bool_unchecked() {
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
        match &self.name {
            rst::ResolvedName::Local(_) => unimplemented!("Not callable"),
            rst::ResolvedName::Symbol(symbol) => {
                unimplemented!("context.call_symbol(symbol, self.arguments)")
            }
            rst::ResolvedName::Builtin(builtin) => {
                let args = self.args.eval(context)?;

                match context.builtins.get(*builtin) {
                    Some(Builtin::Function(f)) => {
                        let args = find_match(&args, &(f.ty)(), &tuple!())?;

                        Ok(FlowSignal::Yield((f.f)(
                            args,
                            &mut BuiltinEvalContext {
                                current_fn: String::new(),
                            },
                        )?))
                    }
                    None => unimplemented!("Function not found"),
                    _ => todo!(),
                }
            }
            rst::ResolvedName::Error(symbol_path) => {
                context.diag(EvalError::SymbolCanNotBeCalled {
                    symbol_path: symbol_path.clone(),
                    src_ref: self.src_ref,
                });
                Ok(FlowSignal::Continue)
            }
        }
    }
}

impl Eval<FlowSignal> for rst::ResolvedName {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        match self {
            rst::ResolvedName::Local(identifier) => {
                use crate::context::StackRead;
                match context.get_local(identifier) {
                    Some(local) => Ok(FlowSignal::Yield(local.clone())),
                    None => {
                        context.diag(EvalError::LocalNotFound(identifier.clone()));
                        Ok(FlowSignal::Continue)
                    }
                }
            }
            rst::ResolvedName::Builtin(builtin) => todo!(),
            rst::ResolvedName::Symbol(refer) => todo!(),
            rst::ResolvedName::Error(symbol_path) => todo!(),
        }
    }
}

impl Eval<FlowSignal> for rst::FunctionExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        use rst::FunctionExpression as Expr;
        match self {
            Expr::Invalid => todo!("Error handling"),
            Expr::Literal(literal) => Ok(FlowSignal::Yield(literal.value().clone())),
            Expr::Name(name) => name.eval(context),
            Expr::Scope(s) => s.eval(context),
            Expr::If(if_) => if_.eval(context),
            Expr::Call(call) => call.eval(context),
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
                        context.top_mut().put_local(l.id.clone(), value);

                        Ok(FlowSignal::Continue)
                    }
                    // Return value immediately
                    FlowSignal::Return(value) => Ok(FlowSignal::Return(value)),
                }
            }
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
