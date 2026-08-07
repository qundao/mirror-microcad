// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in crate.

use microcad_builtin_proc_macros::builtin_mod;
use microcad_lang_base::{BuiltinId, HashMap};
use microcad_lang_types::{Arguments, Value, ValueError};
use thiserror::Error;

pub struct BuiltinEvalContext {
    pub current_fn: String,
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
    fn call(&self, args: Arguments, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError>;
}

// Allow closures to act as BuiltinHandlers
impl<F> BuiltinHandler for F
where
    F: Fn(Arguments, &mut BuiltinEvalContext) -> Result<Value, BuiltinError> + Send + Sync,
{
    fn call(&self, args: Arguments, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        self(args, ctx)
    }
}

#[derive(Default)]
pub struct BuiltinRegistry {
    handlers: HashMap<BuiltinId, Box<dyn BuiltinHandler>>,
}

impl std::fmt::Debug for BuiltinRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuiltinRegistry")
    }
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
        args: Arguments,
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
pub type BuiltinFn = fn(Arguments, &mut BuiltinEvalContext) -> Result<Value, BuiltinError>;

/// Metadata for an individual parameter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    /// Name of the parameter (for named/keyword binding and diagnostics).
    pub name: &'static str,
    // ty: Type,
}

impl Param {
    /// Reusable static slice for standard binary operations (lhs, rhs)
    pub const BINARY: &'static [Param] = &[Param::new("lhs"), Param::new("rhs")];

    /// Reusable static slice for standard unary operations (operand)
    pub const UNARY: &'static [Param] = &[Param::new("operand")];

    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone)]
pub struct BuiltinSignature {
    /// Expected parameter metadata in order.
    pub params: &'static [Param],
    /// Accepts arbitrary positional arguments (e.g. `core::list`, `core::format`).
    pub variadic: bool,
}

impl BuiltinSignature {
    /// Reusable static slice for standard unary operations (operand)
    pub const EMPTY: &'static [Param] = &[];

    pub const fn bin_op() -> Self {
        Self {
            params: Param::BINARY,
            variadic: false,
        }
    }

    pub const fn variadic() -> Self {
        Self {
            params: Self::EMPTY,
            variadic: true,
        }
    }
}

/// A statically compiled built-in definition containing its ID and execution pointer.
#[derive(Debug, Clone)]
pub struct Builtin {
    pub id: BuiltinId,
    pub name: &'static str,
    pub signature: BuiltinSignature,
    pub func: BuiltinFn,
}

impl Builtin {
    pub const fn new(name: &'static str, signature: BuiltinSignature, func: BuiltinFn) -> Self {
        Self {
            id: BuiltinId::from_name(name),
            name,
            signature,
            func,
        }
    }

    pub const fn hash(&self) -> u64 {
        self.id.0
    }
}

pub mod core {

    use microcad_lang_base::BuiltinId;
    use microcad_lang_types::{Arguments, Tuple, Value};

    use crate::{Builtin, BuiltinError, BuiltinEvalContext, BuiltinSignature};

    // 1. Binary Math Operation: add(lhs, rhs)
    pub static ADD: Builtin = Builtin::new("core::add", BuiltinSignature::bin_op(), add);

    // Individual standalone function implementations
    // #[builtin_fn(lhs: Any, rhs: Any) -> Any]
    pub fn add(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs + rhs)?)
    }

    pub fn greater_than(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs > rhs).into())
    }

    pub fn sub(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs - rhs)?)
    }
}

#[test]
fn greater_than() {
    let mut context = BuiltinEvalContext {
        current_fn: String::new(),
    };
    assert_eq!(
        core::greater_than(tuple!(lhs = 3, rhs = 5), &mut context).unwrap(),
        Value::from(false)
    );
    assert_eq!(
        core::greater_than(tuple!(lhs = 5, rhs = 3), &mut context).unwrap(),
        Value::from(true)
    );
}
