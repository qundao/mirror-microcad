// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use cgmath::{InnerSpace, SquareMatrix};
use microcad_core::{Integer, Mat3, Scalar, Vec3};
use microcad_lang::{diag::*, eval::*, parameter, resolve::*, ty::*, value::*};

/// Absolute value abs(x)
fn abs() -> Symbol {
    Symbol::new_builtin_fn(
        "abs",
        [parameter!(x)].into_iter(),
        &|_params, args, ctx| {
            let (_, arg) = args.get_single()?;
            Ok(match &arg.value {
                Value::Integer(i) => Value::Integer(i.abs()),
                Value::Quantity(q) => {
                    Value::Quantity(Quantity::new(q.value.abs(), q.quantity_type.clone()))
                }
                value => {
                    ctx.error(
                        arg,
                        EvalError::BuiltinError(format!("Cannot calculate abs({value})")),
                    )?;
                    Value::None
                }
            })
        },
        None,
    )
}

/// Square root sqrt(x).
fn sqrt() -> Symbol {
    Symbol::new_builtin_fn(
        "sqrt",
        [parameter!(x)].into_iter(),
        &|_params, args, ctx| {
            let (_, arg) = args.get_single()?;
            Ok(match &arg.value {
                Value::Integer(i) => (*i as Scalar).sqrt().into(),
                Value::Quantity(q) => {
                    Value::Quantity(Quantity::new(q.value.sqrt(), q.quantity_type.clone()))
                }
                value => {
                    ctx.error(
                        arg,
                        EvalError::BuiltinError(format!("Cannot calculate sqrt({value})")),
                    )?;
                    Value::None
                }
            })
        },
        None,
    )
}

/// Cast some Quantity into an Integer.
fn int() -> Symbol {
    Symbol::new_builtin_fn(
        "int",
        [parameter!(x)].into_iter(),
        &|_params, args, ctx| {
            let (_, arg) = args.get_single()?;
            Ok(match &arg.value {
                Value::Integer(i) => Value::Integer(*i),
                Value::Quantity(q) => Value::Integer(q.value.floor() as Integer),
                value => {
                    ctx.error(
                        arg,
                        EvalError::BuiltinError(format!("Cannot calculate int({value})")),
                    )?;
                    Value::None
                }
            })
        },
        None,
    )
}

/// Implementation for a builtin trigonometric function.
fn trigonometric(
    name: &str,
    args: &ArgumentValueList,
    ctx: &mut EvalContext,
    f: impl FnOnce(f64) -> f64,
) -> EvalResult<Value> {
    let (_, arg) = args.get_single()?;
    Ok(match &arg.value {
        Value::Integer(i) => Value::Quantity(Quantity::new(f(*i as f64), QuantityType::Scalar)),
        Value::Quantity(Quantity {
            value,
            quantity_type: QuantityType::Angle,
        })
        | Value::Quantity(Quantity {
            value,
            quantity_type: QuantityType::Scalar,
        }) => Value::Quantity(Quantity::new(f(*value), QuantityType::Scalar)),
        value => {
            ctx.error(
                arg,
                EvalError::BuiltinError(format!("Cannot calculate {name}({value})")),
            )?;
            Value::None
        }
    })
}

/// Calculate cos(x).
fn cos() -> Symbol {
    Symbol::new_builtin_fn(
        "cos",
        [parameter!(x)].into_iter(),
        &|_params, args, ctx| trigonometric("cos", args, ctx, |v| v.cos()),
        None,
    )
}

/// Calculate sin(x).
fn sin() -> Symbol {
    Symbol::new_builtin_fn(
        "sin",
        [parameter!(x)].into_iter(),
        &|_params, args, ctx| trigonometric("sin", args, ctx, |v| v.sin()),
        None,
    )
}

/// Calculate tan(x).
fn tan() -> Symbol {
    Symbol::new_builtin_fn(
        "tan",
        [parameter!(x)].into_iter(),
        &|_params, args, ctx| trigonometric("tan", args, ctx, |v| v.tan()),
        None,
    )
}

/// Calculate acos(x).
fn acos() -> Symbol {
    Symbol::new_builtin_fn(
        "acos",
        [parameter!(x)].into_iter(),
        &|_params, args, ctx| trigonometric("acos", args, ctx, |v| v.acos()),
        None,
    )
}

/// Calculate asin(x).
fn asin() -> Symbol {
    Symbol::new_builtin_fn(
        "asin",
        [parameter!(x)].into_iter(),
        &|_params, args, ctx| trigonometric("asin", args, ctx, |v| v.asin()),
        None,
    )
}

/// Calculate atan(x).
fn atan() -> Symbol {
    Symbol::new_builtin_fn(
        "atan",
        [parameter!(x)].into_iter(),
        &|_params, args, ctx| trigonometric("atan", args, ctx, |v| v.atan()),
        None,
    )
}

/// Helper function to get an angle from a field in an argument list.
///
/// Returns `None` if the argument is not an angle.
fn get_angle(args: &Tuple, axis: &str) -> Option<cgmath::Rad<f64>> {
    match args.get_value(axis).expect("Argument expected") {
        Value::Quantity(Quantity {
            value,
            quantity_type: QuantityType::Angle,
        }) => Some(cgmath::Rad::<f64>(*value)),
        _ => None,
    }
}

/// Helper function to return rotation X,Y,Z rotation matrices from an [`Tuple`].
fn rotation_matrices_xyz(args: &Tuple) -> (Mat3, Mat3, Mat3) {
    match (
        get_angle(args, "x"),
        get_angle(args, "y"),
        get_angle(args, "z"),
    ) {
        (Some(angle_x), Some(angle_y), Some(angle_z)) => (
            Mat3::from_angle_x(angle_x),
            Mat3::from_angle_y(angle_y),
            Mat3::from_angle_z(angle_z),
        ),
        _ => (Mat3::identity(), Mat3::identity(), Mat3::identity()),
    }
}

pub fn orient_z_to(target: Vec3) -> Mat3 {
    let z_axis = Vec3::unit_z();
    let target = target.normalize();

    // Handle edge case where target is already Z
    if (target - z_axis).magnitude2() < 1e-6 {
        return Mat3::identity();
    }

    // Handle 180-degree rotation (target is -Z)
    if (target + z_axis).magnitude2() < 1e-6 {
        // Rotate 180° around any axis perpendicular to Z.
        // For stability, pick X if possible, otherwise Y.
        let perp_axis = if z_axis.cross(Vec3::unit_x()).magnitude2() > 1e-6 {
            Vec3::unit_x()
        } else {
            Vec3::unit_y()
        };
        return Mat3::from_axis_angle(perp_axis, cgmath::Rad(std::f64::consts::PI));
    }

    // Normal case
    let rotation_axis = z_axis.cross(target).normalize();
    let dot = z_axis.dot(target).clamp(-1.0, 1.0); // avoid NaNs
    let angle = cgmath::Rad(dot.acos());

    Mat3::from_axis_angle(rotation_axis, angle)
}

/// Rotate a vector around an axis.
fn rotate_around_axis() -> Symbol {
    Symbol::new_builtin_fn(
        "rotate_around_axis",
        [
            parameter!(angle: Angle),
            parameter!(x: Scalar),
            parameter!(y: Scalar),
            parameter!(z: Scalar),
        ]
        .into_iter(),
        &|params, args, ctx| match ArgumentMatch::find_match(args, params) {
            Ok(ref args) => Ok(match get_angle(args, "angle") {
                Some(angle) => {
                    let axis = Vec3::new(args.get("x"), args.get("y"), args.get("z"));
                    let matrix = Mat3::from_axis_angle(axis, angle);
                    Value::Matrix(Box::new(Matrix::Matrix3(matrix)))
                }
                None => Value::None,
            }),
            Err(err) => {
                ctx.error(args, err)?;
                Ok(Value::None)
            }
        },
        None,
    )
}

/// Rotate around X, Y, Z (in that order).
fn rotate_xyz() -> Symbol {
    Symbol::new_builtin_fn(
        "rotate_xyz",
        [
            parameter!(x: Angle),
            parameter!(y: Angle),
            parameter!(z: Angle),
        ]
        .into_iter(),
        &|params, args, ctx| match ArgumentMatch::find_match(args, params) {
            Ok(args) => {
                let (x_matrix, y_matrix, z_matrix) = rotation_matrices_xyz(&args);
                Ok(Value::Matrix(Box::new(Matrix::Matrix3(
                    x_matrix * y_matrix * z_matrix,
                ))))
            }
            Err(err) => {
                ctx.error(args, err)?;
                Ok(Value::None)
            }
        },
        None,
    )
}

/// Rotate around Z, Y, X (in that order).
fn rotate_zyx() -> Symbol {
    Symbol::new_builtin_fn(
        "rotate_zyx",
        [
            parameter!(x: Angle),
            parameter!(y: Angle),
            parameter!(z: Angle),
        ]
        .into_iter(),
        &|params, args, ctx| match ArgumentMatch::find_match(args, params) {
            Ok(args) => {
                let (x_matrix, y_matrix, z_matrix) = rotation_matrices_xyz(&args);
                Ok(Value::Matrix(Box::new(Matrix::Matrix3(
                    z_matrix * y_matrix * x_matrix,
                ))))
            }
            Err(err) => {
                ctx.error(args, err)?;
                Ok(Value::None)
            }
        },
        None,
    )
}

pub fn math() -> Symbol {
    crate::ModuleBuilder::new("math")
        .pub_const("PI", std::f64::consts::PI)
        .pub_const("X", Value::Tuple(Box::new(Vec3::unit_x().into())))
        .pub_const("Y", Value::Tuple(Box::new(Vec3::unit_y().into())))
        .pub_const("Z", Value::Tuple(Box::new(Vec3::unit_z().into())))
        .symbol(abs())
        .symbol(sqrt())
        .symbol(int())
        .symbol(cos())
        .symbol(sin())
        .symbol(tan())
        .symbol(acos())
        .symbol(asin())
        .symbol(atan())
        .symbol(rotate_around_axis())
        .symbol(rotate_xyz())
        .symbol(rotate_zyx())
        .build()
}
