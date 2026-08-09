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
    use microcad_lang_types::{BinaryOperator, Integer};

    use super::*;

    /// Calculate the sum of two values
    #[builtin_fn(core::add(lhs: Any, rhs: Any) -> Any)]
    pub fn add(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs + rhs)?)
    }

    /// Calculate the difference of two values.
    #[builtin_fn(core::sub(lhs: Any, rhs: Any) -> Any)]
    pub fn sub(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs - rhs)?)
    }

    /// Calculate the product of two values.
    #[builtin_fn(core::mul(lhs: Any, rhs: Any) -> Any)]
    pub fn mul(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs * rhs)?)
    }

    /// Division of two values.
    #[builtin_fn(core::div(lhs: Any, rhs: Any) -> Any)]
    pub fn div(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs / rhs)?)
    }

    /// Union of two values.
    #[builtin_fn(core::union(lhs: Any, rhs: Any) -> Any)]
    pub fn union(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs | rhs)?)
    }

    /// Union of two values.
    #[builtin_fn(core::intersect(lhs: Any, rhs: Any) -> Any)]
    pub fn intersect(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs & rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::gt(lhs: Any, rhs: Any) -> Bool)]
    pub fn gt(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok(lhs.cmp(BinaryOperator::GreaterThan, &rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::lt(lhs: Any, rhs: Any) -> Bool)]
    pub fn lt(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok(lhs.cmp(BinaryOperator::LessThan, &rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::ge(lhs: Any, rhs: Any) -> Bool)]
    pub fn ge(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok(lhs.cmp(BinaryOperator::GreaterEqual, &rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::le(lhs: Any, rhs: Any) -> Bool)]
    pub fn le(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok(lhs.cmp(BinaryOperator::LessEqual, &rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::eq(lhs: Any, rhs: Any) -> Bool)]
    pub fn eq(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok(lhs.cmp(BinaryOperator::Equal, &rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::near(lhs: Any, rhs: Any) -> Bool)]
    pub fn near(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok(lhs.cmp(BinaryOperator::Near, &rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::not_equal(lhs: Any, rhs: Any) -> Bool)]
    pub fn not_equal(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok(lhs.cmp(BinaryOperator::NotEqual, &rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::and(lhs: Any, rhs: Any) -> Bool)]
    pub fn and(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs == rhs).into())
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::or(lhs: Any, rhs: Any) -> Bool)]
    pub fn or(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok((lhs | rhs)?)
    }

    /// Compare to values if they are greater_than
    #[builtin_fn(core::xor(lhs: Any, rhs: Any) -> Bool)]
    pub fn xor(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let (lhs, rhs) = args.get_binary();
        Ok(lhs.pow(&rhs)?)
    }

    /// Negative value.
    #[builtin_fn(core::neg(rhs: Any) -> Any)]
    pub fn neg(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let rhs = args.get_unary();
        Ok((-rhs)?)
    }

    /// Positive value.
    #[builtin_fn(core::plus(rhs: Any) -> Any)]
    pub fn plus(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let rhs = args.get_unary();
        Ok(rhs)
    }

    /// Logical NOT.
    #[builtin_fn(core::not(rhs: Any) -> Any)]
    pub fn not(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let rhs = args.get_unary();
        Ok((!rhs)?)
    }

    #[builtin_fn(core::array_access(lhs: Any, index: Integer) -> Any)]
    pub fn array_access(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        let lhs = args.get("lhs");
        let index: Integer = match args.get("index") {
            Value::Integer(i) => *i,
            _ => unreachable!(),
        };

        match lhs {
            Value::Array(arr) => match arr.get(index.to_num::<usize>()) {
                Some(value) => Ok(value.clone()),
                None => Err(BuiltinError::ValueError(todo!())),
            },
            _ => unreachable!(),
        }
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
