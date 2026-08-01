// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluate function

use microcad_lang_base::Identifier;
use microcad_lang_types::{Array, Integer, Tuple, Ty, Type, Value, ValueList};
use microcad_package::rst;

use crate::{
    ArgumentMatch, ArgumentValue, ArgumentValueList, CallTrait, Eval, EvalContext, EvalError,
    EvalResult,
    context::{FunctionFrame, FunctionScopeFrame},
};

use std::ops::ControlFlow;

impl Eval<Value> for rst::function::FormatString {
    fn eval(&self, _context: &mut EvalContext) -> EvalResult<Value> {
        todo!()
    }
}

impl Eval<Value> for rst::function::RangeExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        Ok(
            match (self.first.eval(context)?, self.last.eval(context)?) {
                (Value::Integer(first), Value::Integer(last)) => {
                    if first > last {
                        context.diag(EvalError::BadRange {
                            first,
                            last,
                            src_ref: self.src_ref,
                        });
                    }

                    let first: i64 = first.into();
                    let last: i64 = last.into();

                    Value::Array(Array::from_values(
                        (first..last + 1)
                            .map(|i| Value::Integer(i.into()))
                            .collect(),
                        Type::Integer,
                    ))
                }
                (_, _) => Value::None,
            },
        )
    }
}

impl Eval<Value> for rst::function::ListExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let value_list = ValueList::new(
            self.0
                .iter()
                .map(|expr| expr.eval(context))
                .collect::<Result<_, _>>()?,
        );

        match value_list.types().common_type() {
            Some(common_type) => Ok(Value::Array(Array::from_values(value_list, common_type))),
            None => {
                context.diag(EvalError::ArrayElementsDifferentTypes(value_list.types()));
                Ok(Value::None)
            }
        }
    }
}

impl Eval<Value> for rst::function::ArrayExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        use rst::function::ArrayExpressionInner as Inner;
        let array = match &self.inner {
            Inner::Range(range_expression) => range_expression.eval(context),
            Inner::List(list_expression) => list_expression.eval(context),
        }?;

        match array {
            Value::Array(array) => match Value::Array(array) * self.unit {
                Ok(value) => Ok(value),
                Err(err) => {
                    context.diag(err);
                    Ok(Value::None)
                }
            },
            _ => unimplemented!("Error handling"),
        }
    }
}

impl Eval<Value> for rst::function::UnnamedArgument {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
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
                    ArgumentValue::new(arg.eval(context)?, None, arg.src_ref),
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
                            arg.expression.eval(context)?,
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

impl Eval<Value> for rst::function::TupleExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
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

        Ok(Value::Tuple(
            Tuple {
                named: named.into_iter().collect(),
                unnamed: unnamed.into_iter().map(|(_, v)| (v.ty(), v)).collect(),
                src_ref: self.src_ref,
            }
            .into(),
        ))
    }
}

impl Eval<ControlFlow<Value>> for rst::function::Scope {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ControlFlow<Value>> {
        context.scope(FunctionScopeFrame::new(), |context| self.0.eval(context))
    }
}

impl Eval<ControlFlow<Value>> for rst::FunctionExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ControlFlow<Value>> {
        use rst::FunctionExpression as Expr;
        match self {
            Expr::Invalid => todo!("Error handling"),
            Expr::Literal(literal) => Ok(ControlFlow::Break(literal.value().clone())),
            Expr::FormatString(f) => f.eval(context),
            Expr::ArrayExpression(a) => a.eval(context),
            Expr::TupleExpression(t) => t.eval(context),
            Expr::Scope(s) => s.eval(context),

            _ => todo!(),
        }
    }
}

impl Eval<ControlFlow<Value>> for rst::FunctionExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ControlFlow<Value>> {
        use rst::FunctionExpression;
        match self {
            FunctionExpression::Invalid => {
                todo!("Error handling")
            }
            _ => todo!(),
        }
    }
}

impl Eval<ControlFlow<Value>> for rst::FunctionStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ControlFlow<Value>> {
        use rst::FunctionStatement as Stmt;
        match self {
            Stmt::Expression(expr) => expr.eval(context),
            Stmt::Return(stmt) => match &stmt.expr {
                Some(expr) => Ok(ControlFlow::Break(expr.eval(context)?)),
                None => Ok(ControlFlow::Break(Value::None)),
            },
            Stmt::Local(l) => {
                todo!()
            }
        }
    }
}

impl Eval<ControlFlow<Value>> for Box<[rst::FunctionStatement]> {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ControlFlow<Value>> {
        for stmt in self {
            match stmt.eval(context) {
                Ok(ControlFlow::Break(value)) => {
                    return Ok(ControlFlow::Break(value));
                }
                Ok(ControlFlow::Continue(_)) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(ControlFlow::Continue(()))
    }
}

impl CallTrait<Value> for rst::Function {
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        match ArgumentMatch::find_match(args, &self.parameters) {
            Ok(args) => context.scope(FunctionFrame::new(args), |context| {
                for stmt in &self.statements {
                    match stmt.eval(context) {
                        Ok(ControlFlow::Break(value)) => {
                            return Ok(value);
                        }
                        Ok(ControlFlow::Continue(_)) => {}
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }

                unreachable!("Function did not return a value")
            }),
            Err(_) => todo!(),
        }
    }
}
