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
mod use_statement;

pub use use_statement::*;

impl Eval for Statement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            Self::Workbench(w) => {
                w.grant(context)?;
                Ok(Value::None)
            }
            Self::Module(m) => {
                m.grant(context)?;
                Ok(Value::None)
            }
            Self::Function(f) => {
                f.grant(context)?;
                Ok(Value::None)
            }
            Self::Use(u) => {
                u.eval(context)?;
                Ok(Value::None)
            }
            Self::Assignment(a) => {
                a.eval(context)?;
                Ok(Value::None)
            }
            Self::If(i) => i.eval(context),
            Self::Expression(e) => e.eval(context),
            Self::InnerAttribute(i) => {
                i.grant(context)?;
                Ok(Value::None)
            }
            Self::InnerDocComment(i) => {
                i.grant(context)?;
                Ok(Value::None)
            }
            Self::Init(i) => {
                i.grant(context)?;
                Ok(Value::None)
            }
            Self::Return(r) => r.eval(context),
        }
    }
}

impl Eval<Option<Model>> for Statement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<Model>> {
        let model: Option<Model> = match self {
            Self::Workbench(w) => {
                w.grant(context)?;
                None
            }
            Self::Module(m) => {
                m.eval(context)?;
                None
            }
            Self::Function(f) => {
                f.grant(context)?;
                None
            }
            Self::Init(i) => {
                i.grant(context)?;
                None
            }
            Self::Return(r) => {
                r.grant(context)?;
                None
            }
            Self::Use(u) => {
                u.eval(context)?;
                None
            }
            Self::Assignment(a) => {
                a.eval(context)?;
                None
            }
            Self::If(i) => i.eval(context)?,
            Self::Expression(e) => e.eval(context)?,
            Self::InnerAttribute(a) => {
                a.grant(context)?;
                None
            }
            Self::InnerDocComment(doc) => {
                doc.grant(context)?;
                None
            }
        };

        if let Some(ref model) = model {
            if model.deduce_output_type() == OutputType::InvalidMixed {
                context.error(self, EvalError::CannotMixGeometry)?;
            }
        }

        Ok(model)
    }
}

impl Eval<Value> for StatementList {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let mut result = Value::None;
        for statement in self.iter() {
            log::trace!("Evaluating statement: {statement}");
            match statement.eval(context)? {
                Value::Return(result) => {
                    return Ok(Value::Return(result));
                }
                value => result = value,
            }
        }
        Ok(result)
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

impl Eval<Models> for StatementList {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Models> {
        let mut models = Models::default();
        let mut output_type = OutputType::NotDetermined;

        for statement in self.iter() {
            if let Some(model) = statement.eval(context)? {
                output_type = output_type.merge(&model.deduce_output_type());
                if output_type == OutputType::InvalidMixed {
                    context.error(statement, EvalError::CannotMixGeometry)?;
                }
                models.push(model);
            }
        }
        models.deduce_output_type();
        Ok(models)
    }
}
