// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;
use crate::value::*;

impl Assignment {
    /// Check if the specified type matches the found type.
    pub fn type_check(&self, found: Type) -> EvalResult<()> {
        if let Some(ty) = &self.specified_type {
            if ty.ty() != found {
                return Err(EvalError::TypeMismatch {
                    id: self.id.clone(),
                    expected: ty.ty(),
                    found,
                });
            }
        }

        Ok(())
    }
}

impl Eval<()> for AssignmentStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<()> {
        log::debug!("Evaluating assignment statement:\n{self}");
        self.grant(context)?;

        let assignment = &self.assignment;

        // evaluate assignment expression
        let new_value: Value = assignment.expression.eval(context)?;
        if let Err(err) = assignment.type_check(new_value.ty()) {
            context.error(self, err)?;
            return Ok(());
        }

        // apply any attributes to model value
        let new_value = match new_value {
            Value::Model(model) => {
                model.set_id(assignment.id.clone());
                let attributes = self.attribute_list.eval(context)?;
                model.borrow_mut().attributes = attributes.clone();
                Value::Model(model)
            }
            value => {
                // all other values can't have attributes
                if !self.attribute_list.is_empty() {
                    context.error(
                        &self.attribute_list,
                        AttributeError::CannotAssignAttribute(
                            self.assignment.expression.to_string(),
                        ),
                    )?;
                }
                value
            }
        };

        let mut abort = false;

        // lookup if we find any existing symbol
        if let Ok(symbol) = context.lookup(
            &QualifiedName::from_id(assignment.id.clone()),
            LookupTarget::Value,
        ) {
            let err = symbol.with_def_mut(|def| match def {
                SymbolDef::Constant(_, id, value) => {
                    if value.is_invalid() {
                        *value = new_value.clone();
                        None
                    } else {
                        Some((
                            assignment.id.clone(),
                            EvalError::ValueAlreadyDefined {
                                location: assignment.src_ref(),
                                name: id.clone(),
                                value: value.to_string(),
                                previous_location: id.src_ref(),
                            },
                        ))
                    }
                }
                SymbolDef::Assignment(..) => {
                    abort = true;
                    None
                }
                _ => Some((
                    assignment.id.clone(),
                    EvalError::NotAnLValue(assignment.id.clone()),
                )),
            });
            // because logging is blocked while `symbol.borrow_mut()` it must run outside the borrow
            if let Some((id, err)) = err {
                context.error(&id, err)?;
            }
        }

        if !abort {
            // now check what to do with the value
            match assignment.qualifier() {
                Qualifier::Const => {
                    if context.get_property(&assignment.id).is_ok() {
                        todo!("property with that name exists")
                    }

                    let symbol = context.lookup(&assignment.id.clone().into(), LookupTarget::Value);
                    match symbol {
                        Ok(symbol) => {
                            if let Err(err) = symbol.set_value(new_value) {
                                context.error(self, err)?
                            }
                        }
                        Err(err) => context.error(self, err)?,
                    }
                }
                Qualifier::Value => {
                    let result = if context.get_property(&assignment.id).is_ok() {
                        if context.is_init() {
                            context.init_property(assignment.id.clone(), new_value)
                        } else {
                            todo!("property with that name exists")
                        }
                    } else {
                        context.set_local_value(assignment.id.clone(), new_value)
                    };
                    if let Err(err) = result {
                        context.error(self, err)?;
                    }
                }
                Qualifier::Prop => {
                    if context.get_local_value(&assignment.id).is_ok() {
                        todo!("local value with that name exists")
                    }
                    if let Err(err) = context.init_property(assignment.id.clone(), new_value) {
                        context.error(self, err)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl Eval<Value> for Assignment {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        self.expression.eval(context)
    }
}
