// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in library definitions.

use microcad_lang_types::{
    Arguments, Array, BinaryOperator, Identifier, Integer, Ty, TypeError, Value,
};
use microcad_macros::{builtin_constant, builtin_fn, builtin_mod};

use crate::{Builtin, BuiltinError, BuiltinEvalContext, builtin};

use serde::{Deserialize, Serialize};

/// The built-in core functions.
#[builtin_mod]
pub mod core {
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
        let index: Integer = args.try_get("index")?;
        let index = index.to_num::<usize>();

        match lhs {
            Value::Array(arr) => match arr.get(index) {
                Some(value) => Ok(value.clone()),
                None => Err(BuiltinError::BadArrayIndex {
                    index,
                    len: arr.len(),
                }),
            },
            value => Err(BuiltinError::TypeError(TypeError::NoArrayType(value.ty()))),
        }
    }

    #[builtin_fn(core::member_access(lhs: Any, name: String) -> Any)]
    pub fn member_access(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        let lhs = args.get("lhs");
        let name: String = args.try_get("name")?;

        match lhs {
            // Get field of a Tuple
            Value::Tuple(tuple) => match tuple.get_field(&Identifier::from(name.as_str())) {
                Some(value) => Ok(value.clone()),
                None => Err(TypeError::TupleHasNoField {
                    field: name,
                    ty: tuple.ty(),
                }
                .into()),
            },
            Value::Model(_model) => todo!(
                "match model.get_child(name)
                Some(prop) => Ok(Value::from(prop)),
                None => Err(ModelError::ChildNotFound "
            ),
            value => Err(BuiltinError::TypeError(TypeError::NoArrayType(value.ty()))),
        }
    }

    #[builtin_fn(core::format(*) -> String)]
    pub fn format(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        Ok(Value::from(
            args.positional_iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(""),
        ))
    }

    #[builtin_fn(core::format_spec(expr: Any, width: Integer, precision: Integer) -> String)]
    pub fn format_spec(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<Value, BuiltinError> {
        use std::fmt::Write;

        let expr = args.get("expr");
        let width: i64 = args.try_get("width")?;
        let precision: i64 = args.try_get("precision")?;

        let mut formatted = String::new();

        // Dynamically apply precision and width using standard format specifiers
        match (width, precision) {
            (w, p) if w >= 0 && p >= 0 => {
                write!(
                    formatted,
                    "{:width$.precision$}",
                    expr,
                    width = w as usize,
                    precision = p as usize
                )?;
            }
            (w, _) if w >= 0 => {
                write!(formatted, "{:width$}", expr, width = w as usize)?;
            }
            (_, p) if p >= 0 => {
                write!(formatted, "{:.precision$}", expr, precision = p as usize)?;
            }
            _ => {
                write!(formatted, "{}", expr)?;
            }
        }

        Ok(formatted.into())
    }

    #[builtin_fn(core::range(start: Integer, end: Integer) -> Any)] // TODO: Return [Integer]
    pub fn range(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let start: i64 = args.try_get("start")?;
        let end: i64 = args.try_get("end")?;
        let array = Array::from_iter((start..=end).map(Value::from));
        Ok(array.into())
    }

    #[builtin_fn(core::array(*) -> Any)]
    pub fn array(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        Ok(Array::from_iter(args.positional_iter().cloned()).into())
    }

    #[builtin_fn(core::tuple(*) -> Any)]
    pub fn tuple(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        Ok(args.0.into())
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
pub mod debug {
    use super::*;

    #[builtin_fn(debug::assert(cond: Bool, message: String))]
    pub fn assert(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let cond = args.get_cond()?;
        let message: String = args.try_get("message")?;

        if cond {
            // assertion ok: return None.
            Ok(Value::None)
        } else {
            // assertion failed: stop eval and return Err.
            Err(BuiltinError::AssertionFailed(message))
        }
    }

    #[builtin_fn(debug::expect(cond: Bool, message: String))]
    pub fn expect(args: Arguments, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let cond = args.get_cond()?;
        let message: String = args.try_get("message")?;
        if !cond {
            ctx.diag(BuiltinError::Expected(message));
        }
        Ok(Value::None)
    }

    #[builtin_fn(debug::panic(message: String))]
    pub fn panic(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let message: String = args.try_get("message")?;
        Err(BuiltinError::Panic(message))
    }

    #[builtin_fn(debug::error(message: String))]
    pub fn error(args: Arguments, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let message: String = args.try_get("message")?;
        ctx.diag(BuiltinError::Error(message));
        Ok(Value::None)
    }

    #[builtin_fn(debug::warning(message: String))]
    pub fn warning(args: Arguments, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let message: String = args.try_get("message")?;
        ctx.diag(BuiltinError::Warning(message));
        Ok(Value::None)
    }

    #[builtin_fn(debug::info(message: String))]
    pub fn info(args: Arguments, ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let message: String = args.try_get("message")?;
        ctx.diag(BuiltinError::Info(message));
        Ok(Value::None)
    }

    /// Print all variables
    #[builtin_fn(debug::print(*))]
    pub fn print(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        println!(
            "{}",
            args.positional_iter()
                .map(|arg| arg.to_string())
                .chain(args.named_iter().map(|(id, arg)| format!("{id} = {arg}")))
                .collect::<Vec<_>>()
                .join(", ")
        );
        Ok(Value::None)
    }
}

#[builtin_mod]
pub mod math {
    use super::*;
    use microcad_lang_types::{MathOps, tuple};

    /// Pi
    #[builtin_constant(math::PI)]
    pub static PI: Builtin = std::f64::consts::PI;

    #[builtin_constant(math::X)]
    pub static X: Builtin = tuple!(x = 1.0, y = 0.0, z = 0.0);

    #[builtin_constant(math::Y)]
    pub static Y: Builtin = tuple!(x = 0.0, y = 1.0, z = 0.0);

    #[builtin_constant(math::Z)]
    pub static Z: Builtin = tuple!(x = 0.0, y = 0.0, z = 1.0);

    #[builtin_fn(math::abs(x: Any) -> Any)]
    pub fn abs(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let x = args.get("x");
        Ok(x.abs()?)
    }

    #[builtin_fn(math::sqrt(x: Any) -> Any)]
    pub fn sqrt(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let x = args.get("x");
        Ok(x.sqrt()?)
    }

    #[builtin_fn(math::sin(x: Any) -> Any)]
    pub fn sin(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let x = args.get("x");
        Ok(x.sin()?)
    }

    #[builtin_fn(math::cos(x: Any) -> Any)]
    pub fn cos(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Value, BuiltinError> {
        let x = args.get("x");
        Ok(x.cos()?)
    }
}

/// Built-in 2D primitives
#[builtin_mod]
pub mod geo2d {
    use microcad_lang_base::BuiltinInfo;
    use microcad_lang_types::{
        Length, Model, ModelOutputType, Type, function_type,
        model::{Element, element::BuiltinWorkpiece},
    };

    use crate::BuiltinPrimitive;

    use super::*;

    pub static CIRCLE: Builtin = Builtin::Primitive(BuiltinPrimitive::new(
        BuiltinInfo::new("__mu::geo2d::Circle"),
        || function_type!((radius: Type::length()) -> Type::Model(ModelOutputType::Geometry2D)),
        Circle::call,
    ));

    /// A circle with a radius.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Circle {
        /// Radius in mm.
        pub radius: Length,
    }

    impl Circle {
        pub fn call(args: Arguments, _ctx: &mut BuiltinEvalContext) -> Result<Model, BuiltinError> {
            Ok(Model::new(BuiltinWorkpiece::Primitive2D(CIRCLE.id())).with_properties(args))
        }

        pub fn from_model(model: &Model) -> Result<Self, BuiltinError> {
            // TODO Assertion if Model is a circle primitive
            assert_eq!(
                model.element,
                Element::BuiltinWorkpiece(BuiltinWorkpiece::Primitive2D(CIRCLE.id()))
            );

            let radius = Length::try_from(
                model
                    .get_property("radius")
                    .expect("Input model must have a property 'radius'")
                    .value
                    .clone(),
            )?;

            Ok(Circle { radius })
        }
    }
}

/// Built-in Operations
#[builtin_mod]
pub mod ops {
    use std::rc::Rc;

    use microcad_lang_base::BuiltinInfo;
    use microcad_lang_types::{
        Length, Model, ModelOutputType, ModelTree, Type, function_type,
        model::{AffineTransform, BooleanOp, Element, element::BuiltinWorkpiece},
    };

    use crate::BuiltinOperation;

    use super::*;

    pub static TRANSLATE: Builtin = Builtin::Operation(BuiltinOperation::new(
        BuiltinInfo::new("__mu::ops::translate"),
        || function_type!((self: Type::Model(ModelOutputType::Any), x: Type::length(), y: Type::length(), z: Type::length()) -> Type::Model(ModelOutputType::Any)),
        translate,
    ));

    pub fn translate(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<ModelTree, BuiltinError> {
        let self_: Rc<ModelTree> = args.try_get("self")?;
        let x: Length = args.try_get("x")?;
        let y: Length = args.try_get("y")?;
        let z: Length = args.try_get("z")?;

        let mut tree = ModelTree::new(
            Model::new(BuiltinWorkpiece::AffineTransform(
                AffineTransform::Translation { x, y, z },
            ))
            .with_op_properties(args),
        );

        tree.adopt_tree(self_.root, &self_.arena);

        Ok(tree)
    }

    pub static DIFFERENCE: Builtin = Builtin::Operation(BuiltinOperation::new(
        BuiltinInfo::new("__mu::ops::difference"),
        || function_type!((self: Type::Model(ModelOutputType::Any)) -> Type::Model(ModelOutputType::Any)),
        difference,
    ));

    pub fn difference(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<ModelTree, BuiltinError> {
        let self_: Rc<ModelTree> = args.try_get("self")?;

        // Create a tree for the groups first.
        let mut group_tree = ModelTree::new(Model::new(Element::Group));
        group_tree.adopt_tree(self_.root, &self_.arena);

        // Create the actual operation node
        let mut tree = ModelTree::new(Model::new(BuiltinWorkpiece::BooleanOp(
            BooleanOp::Difference,
        )));
        tree.adopt_tree(group_tree.root, &group_tree.arena);

        Ok(tree)
    }

    pub static EXTRUDE: Builtin = Builtin::Operation(BuiltinOperation::new(
        BuiltinInfo::new("__mu::ops::extrude"),
        || function_type!((self: Type::Model(ModelOutputType::Geometry2D), height: Type::length()) -> Type::Model(ModelOutputType::Geometry3D)),
        extrude,
    ));

    pub fn extrude(
        args: Arguments,
        _ctx: &mut BuiltinEvalContext,
    ) -> Result<ModelTree, BuiltinError> {
        let self_: Rc<ModelTree> = args.try_get("self")?;
        let mut tree = ModelTree::new(
            Model::new(BuiltinWorkpiece::Operation(EXTRUDE.id())).with_op_properties(args),
        );
        tree.adopt_tree(self_.root, &self_.arena);

        Ok(tree)
    }
}
