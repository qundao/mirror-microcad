// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in library definitions.

use microcad_builtin_proc_macros::{builtin_constant, builtin_fn, builtin_mod};
use microcad_lang_types::{Arguments, Value};

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

    #[builtin_fn(core::array_access(lhs: Any, index: Any) -> Any)]
    pub fn array_access(
        _args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        todo!()
    }

    #[builtin_fn(core::property_access(lhs: Any, index: Any) -> Any)]
    pub fn property_access(
        _args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        todo!()
    }

    #[builtin_fn(core::tuple_access(lhs: Any, index: String) -> Any)]
    pub fn tuple_access(
        _args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        todo!()
    }

    #[builtin_fn(core::format(*) -> String)]
    pub fn format(_args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        todo!()
    }

    #[builtin_fn(core::format_spec(expr: Any, width: Integer, precision: Integer) -> String)]
    pub fn format_spec(
        _args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        todo!()
    }

    #[builtin_fn(core::range(start: Integer, end: Integer) -> Integer)] // TODO: Return [Integer]
    pub fn range(_args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        todo!()
    }

    #[builtin_fn(core::list(*) -> Any)]
    pub fn list(_args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        todo!()
    }

    #[builtin_fn(core::tuple(*) -> Any)]
    pub fn tuple(_args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        todo!()
    }

    #[builtin_fn(core::attribute_access(lhs: Any, name: String) -> Any)]
    pub fn attribute_access(
        _args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        todo!()
    }
}

#[builtin_mod]
pub mod math {
    use super::*;

    /// Pi
    #[builtin_constant(math::PI)]
    pub static PI: Builtin = std::f64::consts::PI;
}
