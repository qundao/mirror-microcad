// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::PushDiag;
use microcad_lang_base::SrcReferrer;

use crate::{eval::*, lower::ir, symbol::SymbolDef, value::*};

impl ir::LocalAssignment {
    /// Check if the specified type matches the found type.
    pub fn type_check(&self, found: Type) -> EvalResult<()> {
        use crate::lower::Identifiable;

        if let Some(ty) = &self.specified_type {
            if ty.ty() != found {
                return Err(EvalError::TypeMismatch {
                    id: self.id(),
                    expected: ty.ty(),
                    found,
                }
                .into());
            }
        }

        Ok(())
    }
}

impl Eval<()> for ir::LocalAssignmentStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<()> {
        use crate::lower::Identifiable;
        log::debug!("Evaluating assignment statement:\n{self}");

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
                model.set_id(assignment.id());
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
        if let Ok(symbol) = context.lookup(&assignment.id().into(), LookupTarget::Value) {
            let err = symbol.with_def_mut(|def| match def {
                SymbolDef::Value(id, value) => {
                    if value.is_invalid() {
                        *value = new_value.clone();
                        None
                    } else {
                        Some((
                            assignment.id(),
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
                _ => Some((assignment.id(), EvalError::NotAnLValue(assignment.id()))),
            });
            // because logging is blocked while `symbol.borrow_mut()` it must run outside the borrow
            if let Some((id, err)) = err {
                context.error(&id, err)?;
            }
        }

        if !abort {
            let result = if context.get_property(assignment.id_ref()).is_ok() {
                if context.is_init() {
                    context.init_property(assignment.id(), new_value)
                } else {
                    todo!("property with that name exists")
                }
            } else {
                context.set_local_value(assignment.id(), new_value)
            };
            if let Err(err) = result {
                context.error(self, err)?;
            }
        }

        Ok(())
    }
}

impl Eval<Value> for ir::LocalAssignment {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        self.expression.eval(context)
    }
}
