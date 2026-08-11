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
use microcad_lang_types::{Arguments, FunctionType, Model, ModelTree, Value};

pub use microcad_builtin_proc_macros::__mu;

#[derive(Debug, Default)]
pub struct BuiltinEvalContext<'a> {
    /// Current function being evaluated.
    pub current_fn: Option<&'a BuiltinFunction>,

    /// Diagnostics.
    pub diags: Vec<BuiltinError>,
}

impl<'a> BuiltinEvalContext<'a> {
    pub fn diag(&mut self, err: impl Into<BuiltinError>) {
        self.diags.push(err.into())
    }
}

/// Built-in execution function signature
pub type BuiltinEvalFn<T = Value> =
    fn(Arguments, &mut BuiltinEvalContext) -> Result<T, BuiltinError>;

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
    pub f: BuiltinEvalFn,
}

impl BuiltinFunction {
    /// Construct a new BuiltinFunction
    pub const fn new(info: BuiltinInfo, ty: BuiltinFn<FunctionType>, f: BuiltinEvalFn) -> Self {
        Self { info, ty, f }
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
                diags: vec![],
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
        Self { info, f }
    }
}

/// A primitive is a function that can produce a single model node.
#[derive(Debug, Clone)]
#[debug("{}", info)]
pub struct BuiltinPrimitive {
    pub info: BuiltinInfo,
    pub ty: BuiltinFn<FunctionType>,
    pub f: BuiltinEvalFn<Model>,
}

impl BuiltinPrimitive {
    pub const fn new(
        info: BuiltinInfo,
        ty: BuiltinFn<FunctionType>,
        f: BuiltinEvalFn<Model>,
    ) -> Self {
        Self { info, ty, f }
    }
}

#[derive(Debug, Clone)]
#[debug("{}", info)]
pub struct BuiltinOperation {
    pub info: BuiltinInfo,
    pub ty: BuiltinFn<FunctionType>,
    pub f: BuiltinEvalFn<ModelTree>,
}

#[derive(Debug, Clone, From)]
pub enum Builtin {
    /// Produces a constant Value
    Constant(BuiltinConstant),
    /// A function computing a Value
    Function(BuiltinFunction),
    /// A function computing a single model node
    Primitive(BuiltinPrimitive),
    /// A function computing a model tree from an existing one
    Operation(BuiltinOperation),
}

impl Builtin {
    pub fn id(&self) -> BuiltinId {
        match self {
            Builtin::Constant(c) => c.info.id(),
            Builtin::Function(f) => f.info.id(),
            Builtin::Primitive(p) => p.info.id(),
            Builtin::Operation(o) => o.info.id(),
        }
    }

    pub fn call_fn(
        &self,
        args: Arguments,
        ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        match self {
            Builtin::Function(f) => (f.f)(args, ctx),
            _ => unreachable!("Only functions can be called."),
        }
    }

    pub fn call_primitive(
        &self,
        args: Arguments,
        ctx: &mut BuiltinEvalContext,
    ) -> Result<Model, BuiltinError> {
        match self {
            Builtin::Primitive(p) => (p.f)(args, ctx),
            _ => unreachable!("Only functions can be called."),
        }
    }

    pub fn call_op(
        &self,
        args: Arguments,
        ctx: &mut BuiltinEvalContext,
    ) -> Result<ModelTree, BuiltinError> {
        match self {
            Builtin::Operation(o) => (o.f)(args, ctx),
            _ => unreachable!("Only functions can be called."),
        }
    }
}

/// A macro to generate built-in functions.
#[macro_export]
macro_rules! builtin {
    (
        @info
        $doc:literal
        $mod_name:ident::$fn_name:ident
    ) => {
        $crate::BuiltinInfo::new(concat!(
            "__mu::",
            stringify!($mod_name),
            "::",
            stringify!($fn_name)
        ))
        .with_doc($doc)
    };

    // Syntax: builtin!(
    //      Function
    //      "Doc string"
    //      mod::func(param1: type1, param2: type2, ...) -> return_type
    // )
    (
        Function
        $doc:literal
        $mod_name:ident::$fn_name:ident ( $func_ty:expr )
    ) => {
        $crate::Builtin::Function($crate::BuiltinFunction::new(
            $crate::builtin!(@info $doc $mod_name::$fn_name),
            || $func_ty,
            $fn_name,
        ))
    };
    // Syntax: builtin!(Constant "A constant" math::PI = std::f64::consts::PI)
    (
        Constant
        $doc:literal
        $mod_name:ident::$fn_name:ident = $value:expr
    ) => {
        $crate::Builtin::Constant($crate::BuiltinConstant::new(
            $crate::builtin!(@info $doc $mod_name::$fn_name),
            || $crate::Value::from($value),
        ))
    };
}
