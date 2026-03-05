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
        match self {
            Self::Assignment(a) => {
                a.eval(context)?;
                Ok(Value::None)
            }
            Self::If(i) => i.eval(context),
            Self::Expression(e) => e.eval(context),
            Self::Return(r) => r.eval(context),

            Self::Workbench(..)
            | Self::Module(..)
            | Self::Function(..)
            | Self::InnerAttribute(..)
            | Self::InnerDocComment(..)
            | Self::Use(..)
            | Self::Init(..) => Ok(Value::None),
        }
    }
}

impl Eval<Option<Model>> for Statement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<Model>> {
        let model: Option<Model> = match self {
            Self::Module(m) => {
                m.eval(context)?;
                None
            }
            Self::Assignment(a) => {
                a.eval(context)?;
                None
            }
            Self::If(i) => i.eval(context)?,
            Self::Expression(e) => e.eval(context)?,

            Self::Workbench(..)
            | Self::Function(..)
            | Self::Use(..)
            | Self::Init(..)
            | Self::Return(..)
            | Self::InnerAttribute(..)
            | Self::InnerDocComment(..) => None,
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

        // Check if we are in a workbench and try to get the workbench kind.
        let kind = context
            .current_symbol()
            .map(|symbol| {
                symbol.with_def(|def| match def {
                    SymbolDef::Workbench(workbench_definition) => {
                        let frame = context.stack.current_frame().expect("Some stack frame");
                        match frame {
                            StackFrame::Workbench(..) => Some(workbench_definition.kind.value),
                            _ => None,
                        }
                    }
                    SymbolDef::SourceFile(_) | SymbolDef::Builtin(_) => None,
                    _ => unreachable!(),
                })
            })
            .unwrap_or_default();

        for statement in self.iter() {
            if let Some(model) = statement.eval(context)? {
                output_type = output_type.merge(&model.deduce_output_type());

                // We are in a workbench. Check if the workbench kind matches the current output type.
                if let Some(kind) = kind {
                    let expected_output_type = match kind {
                        WorkbenchKind::Part => OutputType::Geometry3D,
                        WorkbenchKind::Sketch => OutputType::Geometry2D,
                        WorkbenchKind::Operation => OutputType::NotDetermined,
                    };

                    if expected_output_type != OutputType::NotDetermined
                        && output_type != expected_output_type
                    {
                        context.error(
                            statement,
                            EvalError::WorkbenchInvalidOutput {
                                kind,
                                produced: output_type,
                                expected: expected_output_type,
                            },
                        )?;
                    }
                }

                if output_type == OutputType::InvalidMixed {
                    context.error(statement, EvalError::CannotMixGeometry)?;
                }
                models.push(model);
            }
        }
        Ok(models)
    }
}
