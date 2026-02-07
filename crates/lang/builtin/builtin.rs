// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin evaluation entity

use crate::{builtin::*, eval::*, syntax::*, value::*};

/// Builtin function type
pub type BuiltinFn =
    dyn Fn(&ParameterValueList, &ArgumentValueList, &mut EvalContext) -> EvalResult<Value>;

/// Builtin function struct
#[derive(Clone)]
pub struct Builtin {
    /// Name of the builtin function
    pub id: Identifier,

    /// Optional parameter value list to check the builtin signature.
    pub parameters: ParameterValueList,

    /// Built-in kind.
    pub kind: BuiltinKind,

    /// Functor to evaluate this function
    pub f: &'static BuiltinFn,

    /// Functor which returns documentation of this function
    pub doc: Option<DocBlock>,
}

/// Kind of the built-in.
#[derive(Clone)]
pub enum BuiltinKind {
    /// A built-in function: `fn ()`.
    Function,
    /// A built-in workpiece: operation, transform, sketch, part.
    Workbench(BuiltinWorkbenchKind),
}

impl BuiltinKind {
    /// return kind name
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Workbench(kind) => kind.as_str(),
        }
    }
}

impl std::fmt::Display for BuiltinKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function => write!(f, "Function"),
            Self::Workbench(kind) => write!(f, "{kind}"),
        }
    }
}

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "__builtin::{}", &self.id)
    }
}

impl Builtin {
    /// Return identifier.
    pub fn id(&self) -> Identifier {
        self.id.clone()
    }
}

impl CallTrait for Builtin {
    /// Call builtin function with given parameter.
    ///
    /// # Arguments
    /// - `args`: Function arguments.
    /// - `context`: Execution context.
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        (self.f)(&self.parameters, args, context)
    }
}
