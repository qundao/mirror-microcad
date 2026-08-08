// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in crate.

use derive_more::{Debug, Display, From};
use microcad_builtin_proc_macros::builtin_mod;
use microcad_lang_base::{BuiltinId, BuiltinName, HashMap};
use microcad_lang_types::{
    ArgumentValueList, Arguments, FunctionType, Type, Value, ValueError, arguments, tuple,
};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct BuiltinEvalContext<'a> {
    pub current_fn: Option<&'a BuiltinFunction>,
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

#[derive(Default)]
pub struct BuiltinRegistry {
    handlers: HashMap<BuiltinId, Builtin>,
}

impl std::fmt::Debug for BuiltinRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuiltinRegistry")
    }
}

impl BuiltinRegistry {
    pub fn new() -> Self {
        let registry = Self {
            handlers: HashMap::default(),
        };
        registry
    }

    pub fn register(&mut self, builtin: Builtin) {
        self.handlers.insert(builtin.id(), builtin);
    }

    pub fn get(&self, id: BuiltinId) -> Option<&Builtin> {
        self.handlers.get(&id)
    }
}

/// Built-in execution function signature
pub type BuiltinFunctionFn = fn(Arguments, &mut BuiltinEvalContext) -> Result<Value, BuiltinError>;

/// A type of a function returning a T as builtin.
pub type BuiltinFn<T> = fn() -> T;

#[derive(Debug, Clone, Display)]
#[debug("{}", name)]
#[display("{}", name)]
pub struct BuiltinInfo {
    pub name: BuiltinName,
    pub doc: Option<&'static str>,
}

impl BuiltinInfo {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name: BuiltinName::new(name),
            doc: None,
        }
    }

    pub const fn with_doc(mut self, doc: &'static str) -> Self {
        self.doc = Some(doc);
        self
    }

    pub const fn hash(&self) -> u64 {
        self.id().0
    }

    pub const fn id(&self) -> BuiltinId {
        self.name.id
    }
}

#[derive(Debug, Clone)]
#[debug("{}", info)]
pub struct BuiltinFunction {
    pub info: BuiltinInfo,
    pub ty: BuiltinFn<FunctionType>,
    pub f: BuiltinFunctionFn,
}

impl BuiltinFunction {
    /// Construct a new BuiltinFunction
    pub const fn new(info: BuiltinInfo, ty: BuiltinFn<FunctionType>, f: BuiltinFunctionFn) -> Self {
        Self { info: info, ty, f }
    }

    /// Get the function type
    pub fn ty(&self) -> FunctionType {
        (self.ty)()
    }

    /// Call the function with a default context.
    pub fn call_isolated(&self, args: Arguments) -> Result<Value, BuiltinError> {
        (self.f)(
            args,
            &mut BuiltinEvalContext {
                current_fn: Some(self),
            },
        )
    }
}

#[derive(Debug, Clone)]
#[debug("{}", info)]
pub struct BuiltinConstant {
    pub info: BuiltinInfo,
    pub f: BuiltinFn<Value>,
}

impl BuiltinConstant {
    pub const fn new(info: BuiltinInfo, f: BuiltinFn<Value>) -> Self {
        Self { info: info, f }
    }
}

#[derive(Debug, Clone, From)]
pub enum Builtin {
    Constant(BuiltinConstant),
    Function(BuiltinFunction),
}

impl Builtin {
    pub const fn function(f: BuiltinFunction) -> Self {
        Self::Function(f)
    }

    pub const fn constant(c: BuiltinConstant) -> Self {
        Self::Constant(c)
    }

    pub fn id(&self) -> BuiltinId {
        match self {
            Builtin::Constant(c) => c.info.id(),
            Builtin::Function(f) => f.info.id(),
        }
    }

    pub fn call_fn(
        &self,
        args: Arguments,
        ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        match self {
            Builtin::Constant(_c) => unimplemented!("Cannot call a constant"),
            Builtin::Function(f) => (f.f)(args, ctx),
        }
    }
}

pub mod core {
    use microcad_lang_types::{Arguments, Type, Value, function_type};

    use crate::{Builtin, BuiltinError, BuiltinEvalContext, BuiltinFunction, BuiltinInfo};

    // 1. Binary Math Operation: add(lhs, rhs)
    pub static ADD: Builtin = Builtin::function(BuiltinFunction::new(
        BuiltinInfo::new("__mu::core::add"),
        || function_type!((lhs: Type::Any, rhs: Type::Any) -> Type::Any),
        add,
    ));

    // Individual standalone function implementations
    // #[builtin_fn(lhs: Any, rhs: Any) -> Any]
    pub fn add(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs + rhs)?)
    }

    pub static GREATER_THAN: Builtin = Builtin::function(BuiltinFunction::new(
        BuiltinInfo::new("__mu::core::greater_than"),
        || function_type!((lhs: Type::Any, rhs: Type::Any) -> Type::Bool),
        greater_than,
    ));

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
    let mut context = BuiltinEvalContext::default();
    assert_eq!(
        core::greater_than(arguments!(lhs = 3, rhs = 5), &mut context).unwrap(),
        Value::from(false)
    );
    assert_eq!(
        core::greater_than(arguments!(lhs = 5, rhs = 3), &mut context).unwrap(),
        Value::from(true)
    );
}
