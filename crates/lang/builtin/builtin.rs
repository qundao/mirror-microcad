// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin evaluation entity

use strum::Display;

use crate::{diag::PushDiag, eval::*, syntax::*, value::*};

/// Builtin function type
pub type BuiltinFn =
    dyn Fn(&ParameterValueList, &ArgumentValueList, &mut EvalContext) -> EvalResult<Value>;

/// Builtin function struct
#[derive(Clone)]
pub struct BuiltinFunction {
    /// Name of the builtin function
    pub(crate) id: Identifier,

    /// Optional parameter value list to check the builtin signature.
    pub parameters: ParameterValueList,

    /// Functor to evaluate this function
    pub f: &'static BuiltinFn,

    /// Functor which returns documentation of this function
    pub doc: Option<DocBlock>,
}

impl Identifiable for BuiltinFunction {
    fn id_ref(&self) -> &Identifier {
        &self.id
    }
}

/// Builtin workbench
#[derive(Clone)]
pub struct BuiltinWorkbench {
    /// Name of the builtin function
    pub(crate) id: Identifier,

    /// Optional parameter value list to check the builtin signature.
    pub parameters: ParameterValueList,

    /// Functor to evaluate this function
    pub f: &'static BuiltinFn,

    pub kind: BuiltinWorkbenchKind,

    /// Functor which returns documentation of this function
    pub doc: Option<DocBlock>,
}

impl Identifiable for BuiltinWorkbench {
    fn id_ref(&self) -> &Identifier {
        &self.id
    }
}

/// The kind of the built-in workbench determines its output.
#[derive(Debug, Clone, Display, PartialEq)]
pub enum BuiltinWorkbenchKind {
    /// A parametric 2D primitive.
    Primitive2D,
    /// A parametric 3D primitive.
    Primitive3D,
    /// An affine transformation.
    Transform,
    /// An operation on a model.
    Operation,
}

impl BuiltinWorkbenchKind {
    /// return kind name
    pub fn as_str(&self) -> &'static str {
        match self {
            BuiltinWorkbenchKind::Primitive2D => "primitive-2d",
            BuiltinWorkbenchKind::Primitive3D => "primitive-3d",
            BuiltinWorkbenchKind::Transform => "transform",
            BuiltinWorkbenchKind::Operation => "operator",
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuiltinConstant {
    pub id: Identifier,
    pub value: Value,
    pub doc: Option<DocBlock>,
}

impl Identifiable for BuiltinConstant {
    fn id_ref(&self) -> &Identifier {
        &self.id
    }
}

/// Builtin enum
#[derive(Clone)]
pub enum Builtin {
    /// Builtin function.
    Function(BuiltinFunction),
    /// Builtin workbench.
    Workbench(BuiltinWorkbench),
    /// Builtin constant
    Constant(BuiltinConstant),
}

impl Identifiable for Builtin {
    fn id_ref(&self) -> &Identifier {
        match &self {
            Builtin::Function(f) => f.id_ref(),
            Builtin::Workbench(w) => w.id_ref(),
            Builtin::Constant(c) => c.id_ref(),
        }
    }
}

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "__builtin::{}", &self.id())
    }
}

impl CallTrait for BuiltinFunction {
    /// Call builtin function with given parameter.
    ///
    /// # Arguments
    /// - `args`: Function arguments.
    /// - `context`: Execution context.
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        (self.f)(&self.parameters, args, context)
    }
}

impl CallTrait for BuiltinWorkbench {
    /// Call builtin function with given parameter.
    ///
    /// # Arguments
    /// - `args`: Function arguments.
    /// - `context`: Execution context.
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        (self.f)(&self.parameters, args, context)
    }
}

impl CallTrait for Builtin {
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            Builtin::Function(f) => f.call(args, context),
            Builtin::Workbench(w) => w.call(args, context),
            Builtin::Constant(c) => {
                context.error(
                    &crate::src_ref::SrcRef(None),
                    EvalError::BuiltinError(format!(
                        "Built-in constant `{}` cannot be called.",
                        c.id()
                    )),
                )?;
                Ok(Value::None)
            }
        }
    }
}
