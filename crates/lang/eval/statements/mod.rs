// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation
// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

mod assignment_statement;
mod expression_statement;
mod if_statement;
mod marker;
mod return_statement;

impl Eval for Statement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        log::trace!("Evaluating statement: {self}");
        match self {
            Self::Assignment(a) => a.eval(context),
            Self::Module(m) => m.eval(context),
            Self::Expression(e) => e.eval(context),
            Self::Return(r) => r.eval(context),

            Self::Workbench(..)
            | Self::Function(..)
            | Self::InnerAttribute(..)
            | Self::InnerDocComment(..)
            | Self::Use(..)
            | Self::Init(..) => Ok(Value::None),
        }
    }
}

impl Eval for StatementList {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let mut models = Models::default();
        let mut output_type = OutputType::NotDetermined;

        let mut last_statement_result = None;
        let result = self
            .statements
            .iter()
            .find_map(|statement| {
                match statement.eval(context) {
                    // early return
                    ret @ Ok(Value::Return(..)) => Some(ret),
                    // accumulate any models
                    Ok(Value::Model(model)) => match output_type {
                        OutputType::InvalidMixed => Some(
                            context
                                .error(statement, EvalError::CannotMixGeometry)
                                .map(|_| Value::None)
                                .map_err(EvalError::DiagError),
                        ),
                        _ => {
                            output_type = output_type.merge(model.deduce_output_type());
                            if OutputType::InvalidMixed != output_type {
                                models.push(model);
                                None
                            } else {
                                Some(
                                    context
                                        .error(statement, EvalError::CannotMixGeometry)
                                        .map(|_| Value::None)
                                        .map_err(EvalError::DiagError),
                                )
                            }
                        }
                    },
                    // continue while no result
                    Ok(Value::None) => None,
                    Ok(value) => {
                        if let Statement::Expression(stmt) = statement {
                            if matches!(stmt.expression, Expression::If(..)) {
                                assert!(last_statement_result.is_none());
                                last_statement_result = Some(value);
                            }
                        }
                        None
                    }
                    Err(err) => Some(Err(err)),
                }
            })
            .unwrap_or(Ok(Value::None))?;

        // HACK: last_statement_result is not needed if if statement ends in tail
        let tail = self.tail_expression();
        let result = if let Some(last_statement_result) = last_statement_result {
            assert!(tail.is_none(), "{:?} {last_statement_result}", tail);
            last_statement_result
        } else {
            // see if there is any result in tail
            tail.map(|exp| exp.eval(context)).unwrap_or(Ok(result))?
        };

        match (result, models.is_empty()) {
            (Value::None, false) => Ok(Value::Model(
                ModelBuilder::new(Element::Group, self.src_ref())
                    .add_children(models)?
                    .build(),
            )),
            (result @ Value::Model(..), _) => Ok(result),
            (_, false) => {
                context.error(
                    &tail.map(|t| t.src_ref()).unwrap_or(self.src_ref()),
                    EvalError::WorkbenchValueResult,
                )?;
                Ok(Value::Model(
                    ModelBuilder::new(Element::Group, self.src_ref())
                        .add_children(models)?
                        .build(),
                ))
            }
            (result, _) => Ok(result),
        }
    }
}

/// Parse inner attributes of a statement list.
impl Eval<Attributes> for StatementList {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Attributes> {
        let mut attributes = Vec::new();
        for statement in self.iter() {
            if let Statement::InnerAttribute(attribute) = statement {
                attributes.append(&mut attribute.eval(context)?);
            }
        }

        Ok(Attributes(attributes))
    }
}
