// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in crate.

mod error;
pub use error::BuiltinError;

pub mod mu;
mod registry;
pub use registry::BuiltinRegistry;

use derive_more::{Debug, Display, From};
use microcad_lang_base::{BuiltinId, BuiltinName};
use microcad_lang_types::{Arguments, FunctionType, Value};

pub use microcad_builtin_proc_macros::__mu;

#[derive(Debug, Default)]
pub struct BuiltinEvalContext<'a> {
    pub current_fn: Option<&'a BuiltinFunction>,
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

/// A macro to generate built-in functions.
#[macro_export]
macro_rules! builtin_function_helper {
    // Syntax: builtin_function_helper!(
    //      "Doc string"
    //      mod::func(param1: type1, param2: type2, ...) -> return_type
    // )
    (
        $doc:literal
        $mod_name:ident::$fn_name:ident ( $func_ty:expr )
    ) => {
        $crate::Builtin::function($crate::BuiltinFunction::new(
            $crate::BuiltinInfo::new(concat!(
                "__mu::",
                stringify!($mod_name),
                "::",
                stringify!($fn_name)
            ))
            .with_doc($doc),
            || $func_ty,
            $fn_name,
        ))
    };
}

#[macro_export]
macro_rules! builtin_constant_helper {
    // Syntax: builtin_constant_helper!("A constant" math::PI = std::f64::consts::PI)
    (
        $doc:literal
        $mod_name:ident::$fn_name:ident = $value:expr
    ) => {
        $crate::Builtin::constant($crate::BuiltinConstant::new(
            $crate::BuiltinInfo::new(concat!(
                "__mu::",
                stringify!($mod_name),
                "::",
                stringify!($fn_name)
            ))
            .with_doc($doc),
            || $crate::Value::from($value),
        ))
    };
}
