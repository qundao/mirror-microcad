// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in crate.

use microcad_builtin_proc_macros::builtin_mod;
use microcad_lang_base::{BuiltinId, HashMap};
use microcad_lang_types::{Value, ValueError};
use thiserror::Error;

use crate::args::unpack_binary_args;

mod args;

pub struct BuiltinEvalContext {
    current_fn: String,
}

impl BuiltinEvalContext {
    pub fn current_fn(&self) -> String {
        self.current_fn.clone()
    }
}

#[derive(Debug, Error)]
pub enum BuiltinError {
    #[error("Value error: {0}")]
    ValueError(#[from] ValueError),

    #[error("Builtin '{name}' expected at least {expected} arguments, found {found}")]
    InvalidArgumentCount {
        name: String,
        expected: usize,
        found: usize,
    },

    #[error("Type mismatch in builtin '{name}': expected {expected}, found {found}")]
    TypeMismatch {
        name: String,
        expected: &'static str,
        found: &'static str,
    },

    #[error("Builtin error in '{name}': {message}")]
    ExecutionFailed { name: String, message: String },
}

pub trait BuiltinHandler: Send + Sync {
    fn call(&self, args: Value, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError>;
}

// Allow closures to act as BuiltinHandlers
impl<F> BuiltinHandler for F
where
    F: Fn(Value, &mut BuiltinEvalContext) -> Result<Value, BuiltinError> + Send + Sync,
{
    fn call(&self, args: Value, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        self(args, ctx)
    }
}

pub struct BuiltinRegistry {
    handlers: HashMap<BuiltinId, Box<dyn BuiltinHandler>>,
}

impl BuiltinRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            handlers: HashMap::default(),
        };
        registry
    }

    pub fn register<H: BuiltinHandler + 'static>(&mut self, id: BuiltinId, handler: H) {
        self.handlers.insert(id, Box::new(handler));
    }

    pub fn get(&self, id: BuiltinId) -> Option<&dyn BuiltinHandler> {
        self.handlers.get(&id).map(|b| b.as_ref())
    }

    pub fn call(
        &self,
        id: BuiltinId,
        args: Value,
        ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        if let Some(handler) = self.get(id) {
            handler.call(args, ctx)
        } else {
            Err(BuiltinError::ExecutionFailed {
                name: ctx.current_fn.clone(),
                message: format!("No builtin registered for {:?}", id),
            })
        }
    }
}

/// Built-in execution function signature
pub type BuiltinFn = fn(Value, &mut BuiltinEvalContext) -> Result<Value, BuiltinError>;

/// A statically compiled built-in definition containing its ID and execution pointer.
#[derive(Copy, Clone)]
pub struct Builtin {
    pub id: BuiltinId,
    pub name: &'static str,
    pub func: BuiltinFn,
}

impl Builtin {
    pub const fn new(name: &'static str, func: BuiltinFn) -> Self {
        Self {
            id: BuiltinId::from_name(name),
            name,
            func,
        }
    }
}

#[builtin_mod]
pub mod core {
    use microcad_lang_types::Value;

    use crate::{BuiltinError, BuiltinEvalContext, args::unpack_binary_args};

    // Individual standalone function implementations
    pub fn add(args: Value, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = unpack_binary_args(args, ctx)?;
        Ok((lhs + rhs)?)
    }

    pub fn sub(args: Value, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = unpack_binary_args(args, ctx)?;
        Ok((lhs + rhs)?)
    }
}
