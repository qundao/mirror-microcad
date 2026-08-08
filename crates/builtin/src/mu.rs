// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in library definitions.

use microcad_builtin_proc_macros::{builtin_constant, builtin_fn, builtin_mod};
use microcad_lang_types::{Arguments, Type, Value};

use crate::{
    Builtin, BuiltinError, BuiltinEvalContext, builtin_constant_helper, builtin_function_helper,
};

#[builtin_mod]
pub mod core {
    use super::*;

    /// Calculate the sum of two values
    #[builtin_fn(core::add(lhs: Any, rhs: Any) -> Any)]
    pub fn add(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs + rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::greater_than(lhs: Any, rhs: Any) -> Any)]
    pub fn greater_than(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs > rhs).into())
    }

    /// Calculate the difference of two values.
    #[builtin_fn(core::sub(lhs: Any, rhs: Any) -> Any)]
    pub fn sub(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs - rhs)?)
    }
}

#[builtin_mod]
pub mod math {
    use super::*;

    /// Pi
    #[builtin_constant(math::PI)]
    pub static PI: Builtin = std::f64::consts::PI;
}
