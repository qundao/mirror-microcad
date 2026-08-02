// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluate function

use microcad_lang_base::{Identifier, SrcRef, SrcReferrer};
use microcad_lang_types::{Array, Integer, Operators, Tuple, Ty, Type, Value, ValueList};
use microcad_package::rst::{self, Function, ResolvedName};

use crate::{
    ArgumentMatch, ArgumentValue, ArgumentValueList, CallTrait, Eval, EvalContext, EvalError,
    EvalResult,
    context::{FunctionFrame, FunctionScopeFrame},
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

    /// Check if the signal is an explicit return.
    pub fn is_return(&self) -> bool {
        matches!(self, FlowSignal::Return(_))
    }
}

impl Eval<FlowSignal> for rst::function::FormatString {
    fn eval(&self, _context: &mut EvalContext) -> EvalResult<FlowSignal> {
        todo!()
    }
}

impl Eval<FlowSignal> for rst::function::RangeFirst {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        self.0.eval(context)
    }
}

impl Eval<FlowSignal> for rst::function::RangeLast {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        self.0.eval(context)
    }
}

impl Eval<FlowSignal> for rst::function::RangeExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        use microcad_lang_base::SrcReferrer;

        match (self.first.eval(context)?, self.last.eval(context)?) {
            (FlowSignal::Yield(Value::Integer(first)), FlowSignal::Yield(Value::Integer(last))) => {
                if first > last {
                    context.diag(EvalError::BadRange {
                        first,
                        last,
                        src_ref: self.src_ref,
                    });
                }

                let first: i64 = first.into();
                let last: i64 = last.into();

                Ok(FlowSignal::Yield(Value::Array(Array::from_values(
                    (first..last + 1)
                        .map(|i| Value::Integer(i.into()))
                        .collect(),
                    Type::Integer,
                ))))
            }
            (FlowSignal::Yield(first), FlowSignal::Yield(last)) => {
                if !matches!(first, Value::Integer(_)) {
                    context.diag(EvalError::InvalidRangeBoundaryType {
                        src_ref: self.first.0.src_ref(),
                    });
                }
                if !matches!(last, Value::Integer(_)) {
                    context.diag(EvalError::InvalidRangeBoundaryType {
                        src_ref: self.last.0.src_ref(),
                    });
                }

                Ok(FlowSignal::Continue)
            }
            (FlowSignal::Continue, _) | (_, FlowSignal::Continue) => {
                context.diag(EvalError::InvalidFlow(self.src_ref));
                Ok(FlowSignal::Continue)
            }
            (FlowSignal::Return(v), _) | (_, FlowSignal::Return(v)) => Ok(FlowSignal::Return(v)),
        }
    }
}

impl Eval<FlowSignal> for rst::function::ListExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        let value_list = ValueList::new(
            self.0
                .iter()
                .map(|expr| match expr.eval(context) {
                    Ok(FlowSignal::Continue) => todo!(),
                    Ok(FlowSignal::Return(_)) => todo!(),
                    Ok(FlowSignal::Yield(_)) => todo!(),
                    Err(_) => todo!(),
                })
                .collect::<EvalResult<_>>()?,
        );

        match value_list.types().common_type() {
            Some(common_type) => Ok(FlowSignal::Yield(Value::Array(Array::from_values(
                value_list,
                common_type,
            )))),
            None => {
                context.diag(EvalError::ArrayElementsDifferentTypes(value_list.types()));
                Ok(FlowSignal::Yield(Value::None))
            }
        }
    }
}

impl Eval<FlowSignal> for rst::function::ArrayExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        use rst::function::ArrayExpressionInner as Inner;
        let array = match &self.inner {
            Inner::Range(range_expression) => range_expression.eval(context),
            Inner::List(list_expression) => list_expression.eval(context),
        }?
        .expect_value(self.src_ref)?;

        match array {
            Value::Array(array) => match Value::Array(array) * self.unit {
                Ok(value) => Ok(FlowSignal::Yield(value)),
                Err(err) => {
                    context.diag(err);
                    Ok(FlowSignal::Yield(Value::None))
                }
            },
            _ => unimplemented!("Error handling"),
        }
    }
}

impl Eval<FlowSignal> for rst::function::UnnamedArgument {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        self.expression.eval(context)
    }
}

impl Eval<ArgumentValueList> for rst::function::ArgumentList {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ArgumentValueList> {
        let mut map: Vec<(Identifier, ArgumentValue)> = self
            .unnamed_args
            .iter()
            .map(|arg| {
                Ok((
                    Identifier::none(),
                    ArgumentValue::new(
                        arg.eval(context)?.expect_value(arg.src_ref)?,
                        None,
                        arg.src_ref,
                    ),
                ))
            })
            .collect::<EvalResult<Vec<_>>>()?;

        map.append(
            &mut self
                .named_args
                .iter()
                .map(|arg| {
                    Ok((
                        arg.id.clone(),
                        ArgumentValue::new(
                            arg.expression
                                .eval(context)?
                                .expect_value(arg.expression.src_ref())?,
                            Some(arg.id.clone()),
                            arg.src_ref,
                        ),
                    ))
                })
                .collect::<EvalResult<Vec<_>>>()?,
        );

        Ok(ArgumentValueList {
            map,
            src_ref: self.src_ref,
        })
    }
}

impl Eval<FlowSignal> for rst::function::TupleExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        let (unnamed, named): (Vec<_>, _) = Eval::<ArgumentValueList>::eval(&self.args, context)?
            .iter()
            .map(|(id, arg)| (id.clone(), arg.value.clone()))
            .partition(|(id, _)| id.is_empty());

        // check unnamed for ambiguous types
        let mut h = microcad_lang_base::HashSet::default();
        unnamed
            .iter()
            .map(|(_, value)| value.ty())
            .try_for_each(|ty| {
                if h.insert(ty.clone()) {
                    Ok(())
                } else {
                    Err(Box::new(EvalError::AmbiguousType {
                        ty,
                        src_ref: self.src_ref,
                    }))
                }
            })?;

        Ok(FlowSignal::Yield(Value::Tuple(
            Tuple {
                named: named.into_iter().collect(),
                unnamed: unnamed.into_iter().map(|(_, v)| (v.ty(), v)).collect(),
                src_ref: self.src_ref,
            }
            .into(),
        )))
    }
}

impl Eval<FlowSignal> for rst::function::Scope {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        context.scope(FunctionScopeFrame::new(), |context| self.0.eval(context))
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
            ResolvedName::Local(_) => unimplemented!("Not callable"),
            ResolvedName::Method(method) => {
                unimplemented!("context.get_builtin_method(method, self.arguments)")
            }
            ResolvedName::Symbol(symbol) => {
                unimplemented!("context.call_symbol(symbol, self.arguments)")
            }
            ResolvedName::Error(symbol_path) => {
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
            ResolvedName::Local(identifier) => {
                use crate::context::StackRead;
                match context.get_local(identifier) {
                    Some(local) => Ok(FlowSignal::Yield(local.clone())),
                    None => {
                        context.diag(EvalError::LocalNotFound(identifier.clone()));
                        Ok(FlowSignal::Continue)
                    }
                }
            }
            ResolvedName::Method(identifier) => todo!(),
            ResolvedName::Symbol(refer) => todo!(),
            ResolvedName::Error(symbol_path) => todo!(),
        }
    }
}

impl Eval<FlowSignal> for rst::function::BinaryOp {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        let lhs = self.lhs.eval(context)?;
        let lhs = match lhs {
            FlowSignal::Continue => {
                context.diag(EvalError::InvalidFlow(self.src_ref));
                return Ok(FlowSignal::Continue);
            }
            FlowSignal::Yield(value) => value,
            FlowSignal::Return(value) => {
                return Ok(FlowSignal::Return(value));
            }
        };

        let rhs = self.rhs.eval(context)?;
        let rhs = match rhs {
            FlowSignal::Continue => {
                context.diag(EvalError::InvalidFlow(self.src_ref));
                return Ok(FlowSignal::Continue);
            }
            FlowSignal::Yield(value) => value,
            FlowSignal::Return(value) => {
                return Ok(FlowSignal::Return(value));
            }
        };

        Ok(FlowSignal::Yield(
            lhs.binary_op(self.op, rhs)
                .map_err(|err| Box::new(EvalError::from(err)))?,
        ))
    }
}

impl Eval<FlowSignal> for rst::FunctionExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<FlowSignal> {
        use rst::FunctionExpression as Expr;
        match self {
            Expr::Invalid => todo!("Error handling"),
            Expr::Literal(literal) => Ok(FlowSignal::Yield(literal.value().clone())),
            Expr::Name(name) => name.eval(context),
            Expr::FormatString(f) => f.eval(context),
            Expr::ArrayExpression(a) => a.eval(context),
            Expr::TupleExpression(t) => t.eval(context),
            Expr::Scope(s) => s.eval(context),
            Expr::If(if_) => if_.eval(context),
            Expr::Call(call) => call.eval(context),
            Expr::BinaryOp(binary_op) => binary_op.eval(context),

            //Expr::ArrayAccess()
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
                if value.is_none() {
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
        match ArgumentMatch::find_match(args, &self.parameters) {
            Ok(args) => context.scope(FunctionFrame::new(args), |context| {
                Ok(self.statements.eval(context)?.into_value())
            }),
            Err(_) => todo!(),
        }
    }
}
