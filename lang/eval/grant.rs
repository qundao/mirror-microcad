// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, syntax::*};

/// Grant statements depending on context
pub trait Grant {
    /// Check statement is granted within the given context
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()>;
}

impl Grant for WorkbenchDefinition {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            matches!(stack_frame, StackFrame::Source(..) | StackFrame::Module(..))
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported(self.kind.as_str()))?;
        }
        Ok(())
    }
}

impl Grant for ModuleDefinition {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            matches!(stack_frame, StackFrame::Source(..) | StackFrame::Module(..))
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported("Module"))?;
        }
        Ok(())
    }
}

impl Grant for FunctionDefinition {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            match stack_frame {
                // TODO: check if expression generates models (see test `source_expression``)
                StackFrame::Source(..) | StackFrame::Module(..) => true,
                StackFrame::Workbench(..) => self.visibility == Visibility::Private,
                _ => false,
            }
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported("Function"))?;
        }
        Ok(())
    }
}
impl Grant for InitDefinition {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            matches!(stack_frame, StackFrame::Workbench(..))
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported("Init"))?;
        }
        Ok(())
    }
}

impl Grant for UseStatement {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        match (&self.visibility, context.stack.current_frame()) {
            (Visibility::Private, _)
            | (Visibility::Public, Some(StackFrame::Source(..) | StackFrame::Module(..))) => (),
            _ => context.error(self, EvalError::StatementNotSupported("Public use"))?,
        }
        Ok(())
    }
}

impl Grant for ReturnStatement {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        if !context.is_within_function() {
            context.error(self, EvalError::StatementNotSupported("Return"))?;
        }
        Ok(())
    }
}

impl Grant for IfStatement {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(..)
                    | StackFrame::Workbench(..)
                    | StackFrame::Body(..)
                    | StackFrame::Function(..)
            )
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported("If"))?;
        }
        Ok(())
    }
}

impl Grant for AssignmentStatement {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            match self.assignment.qualifier() {
                Qualifier::Const => {
                    matches!(stack_frame, StackFrame::Source(..) | StackFrame::Module(..))
                        | (matches!(stack_frame, StackFrame::Workbench(..))
                            && self.assignment.visibility == Visibility::Private)
                }
                Qualifier::Value => {
                    matches!(
                        stack_frame,
                        StackFrame::Source(..)
                            | StackFrame::Module(..)
                            | StackFrame::Body(..)
                            | StackFrame::Workbench(..)
                            | StackFrame::Init(..)
                            | StackFrame::Function(..)
                    )
                }
                Qualifier::Prop => matches!(stack_frame, StackFrame::Workbench(..)),
            }
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported("Assignment"))?;
        }
        Ok(())
    }
}

impl Grant for ExpressionStatement {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(..)
                    | StackFrame::Body(..)
                    | StackFrame::Workbench(..)
                    | StackFrame::Function(..)
            )
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported("Expression"))?;
        }
        Ok(())
    }
}

impl Grant for Marker {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            matches!(stack_frame, StackFrame::Workbench(..))
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported("@input"))?;
        }
        Ok(())
    }
}

impl Grant for Attribute {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(..) | StackFrame::Body(..) | StackFrame::Workbench(..)
            )
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported("InnerAttribute"))?;
        }
        Ok(())
    }
}

impl Grant for Body {
    fn grant(&self, context: &mut EvalContext) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = context.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(..) | StackFrame::Body(..) | StackFrame::Workbench(..)
            )
        } else {
            false
        };
        if !granted {
            context.error(self, EvalError::StatementNotSupported("Body"))?;
        }
        Ok(())
    }
}
