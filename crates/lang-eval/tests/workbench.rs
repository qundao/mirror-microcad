// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for evaluating workbenches.

use microcad_builtin::__mu;
use microcad_lang_base::{DisplayWithCtx, SrcRef};
use microcad_lang_eval::{CallTrait, Eval, EvalContext};
use microcad_lang_types::{
    ArgumentValue, ArgumentValueList, Length, ModelTree, Type, Value, argument_value, list, tuple,
};
use microcad_package::{
    SymbolId,
    symbol::{
        self, ConstantValue, Parameter, Path, SourceStatement, WorkbenchExpression,
        WorkbenchStatement, constant, workbench,
    },
};

/// Expressions used for testing
pub mod helper {
    use microcad_builtin::BuiltinId;
    use microcad_lang_types::Integer;
    use microcad_package::symbol::{
        ConstantExpression, ExprSpec, SourceStatement, WorkbenchExpression, WorkbenchStatement,
        workbench::ArgumentList,
    };

    use super::*;

    /// length literal expression: `4.0mm`
    pub fn length(v: f64) -> ConstantExpression {
        ConstantValue::from_value(Length::mm(v)).into()
    }

    /// integer literal expression: `4`
    pub fn integer(n: i32) -> ConstantExpression {
        ConstantValue::from_value(n).into()
    }

    /// angle in degrees expression: `360°`
    pub fn deg(v: f64) -> ConstantExpression {
        let value = Value::deg(v);
        ConstantValue::from_value(value).into()
    }

    /// Any call to an op
    pub fn call<Expr: ExprSpec>(
        id: BuiltinId,
        args: impl Into<symbol::ArgumentList<Expr>>,
    ) -> symbol::Call<Expr> {
        symbol::Call::builtin(id).with_args(args)
    }

    /// __mu::geo2d::Circle(radius = expr)
    pub fn call_circle(radius: impl Into<WorkbenchExpression>) -> WorkbenchExpression {
        workbench::WorkbenchCall::builtin(__mu!(geo2d::Circle))
            .with_args(vec![workbench::Argument::named("radius", radius)])
            .into()
    }

    /// __mu::op::translate(self, x, y, z)
    pub fn call_translate(
        self_: impl Into<WorkbenchExpression>,
        x: impl Into<WorkbenchExpression>,
        y: impl Into<WorkbenchExpression>,
        z: impl Into<WorkbenchExpression>,
    ) -> WorkbenchExpression {
        workbench::WorkbenchCall::builtin(__mu!(ops::translate))
            .with_args(vec![
                workbench::Argument::named("self", self_),
                workbench::Argument::named("x", x),
                workbench::Argument::named("y", y),
                workbench::Argument::named("z", z),
            ])
            .into()
    }

    /// Local expression with `name`
    pub fn local(name: &str) -> ConstantExpression {
        symbol::Path::Resolved(SymbolId::Local(name.into())).into()
    }

    /// call __mu::core::member_access
    pub fn get<Expr: ExprSpec>(lhs: Expr, name: impl AsRef<str>) -> Expr
    where
        Expr: From<symbol::Call<Expr>>,
    {
        Expr::from(call(
            __mu!(core::member_access),
            vec![
                symbol::Argument::named("lhs", lhs),
                symbol::Argument::named("name", Value::from(name.as_ref().to_string())),
            ],
        ))
    }

    /// Input placeholder: @input
    /// TODO: Might be replaced with `self` in the future
    pub fn input() -> WorkbenchExpression {
        symbol::workbench::Marker::input().into()
    }

    pub fn workbench_statements(
        statements: impl IntoIterator<Item = WorkbenchStatement>,
    ) -> Box<[WorkbenchStatement]> {
        statements
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn source_statements(
        statements: impl IntoIterator<Item = SourceStatement>,
    ) -> Box<[SourceStatement]> {
        statements
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// [1..n] / n * 360°
    pub fn polar_expr() -> symbol::ConstantExpression {
        use symbol::constant::Argument as Arg;
        // [1..n]
        let range = call(
            __mu!(core::range),
            vec![
                Arg::named("start", integer(1)),
                Arg::named("end", local("n")),
            ],
        );

        // [1..n] / n
        let div = call(
            __mu!(core::div),
            vec![Arg::named("lhs", range), Arg::named("rhs", local("n"))],
        );

        // * 360°
        let mul = call(
            __mu!(core::mul),
            vec![Arg::named("lhs", div), Arg::named("rhs", Value::deg(360.0))],
        );

        mul.into()
    }
}

/// Evaluate something into a model tree and test snapshot
pub fn eval_to_model_test<T: Eval<ModelTree>>(name: &str, t: T) -> ModelTree {
    let mut context = EvalContext::new();
    let model: ModelTree = t.eval(&mut context).expect("No error");
    insta::assert_snapshot!(name, model.to_string_with_ctx(&context));
    model
}

/// Call a workbench with arguments to produce a model tree and test snapshot
pub fn call_workbench<T: CallTrait<ModelTree>>(
    name: &str,
    workbench: &T,
    arg_iter: impl IntoIterator<Item = ArgumentValue>,
) -> ModelTree {
    let mut context = EvalContext::new();
    let model: ModelTree = workbench
        .call(&ArgumentValueList::from_iter(arg_iter), &mut context)
        .expect("No error");
    insta::assert_snapshot!(name, model.to_string_with_ctx(&context));
    model
}

/// { __mu::geo2d::Circle(radius = 4.0mm); }
#[test]
fn group() {
    use helper::*;

    eval_to_model_test(
        "group",
        symbol::workbench::Group {
            src_ref: SrcRef::none(),
            attr: symbol::ModelAttributes::default(),
            statements: workbench_statements([WorkbenchStatement::expr(call_circle(length(4.0)))]),
        },
    );
}

/// {
///     prop a = 4.0mm; // Add property to current model tree
///     __mu::geo2d::Circle(a); // Add circle as new Model Tree
/// }
#[test]
fn group_with_property() {
    use helper::*;

    let model = eval_to_model_test(
        "group_with_property",
        symbol::workbench::Group {
            src_ref: SrcRef::none(),
            attr: symbol::ModelAttributes::default(),
            statements: workbench_statements([
                WorkbenchStatement::prop("a", length(4.0)),
                WorkbenchStatement::expr(call_circle(local("a"))),
            ]),
        },
    );

    let prop = model.get_property_value("a");
    assert_eq!(prop, Value::from(Length::mm(4.0)));
}

/// __mu::geo2d::Circle(radius = 4.0mm).translate(x = 1.0mm, y = 2.0mm, z = 0.0mm)
#[test]
fn translate_circle() {
    use helper::*;

    let model = eval_to_model_test(
        "translate_circle",
        call_translate(
            call_circle(length(4.0)),
            length(1.0),
            length(2.0),
            length(0.0),
        ),
    );

    let prop = model.get_property_value("radius"); // We should be able to access the property.
    assert_eq!(prop, Value::from(Length::mm(4.0)));
}

/// sketch Circle() { __mu::geo2d::Circle(radius = 4.0mm); }
#[test]
fn circle_without_parameter() {
    use helper::*;

    let workbench = symbol::Workbench {
        signature: workbench::WorkbenchSignature::new(
            symbol::WorkbenchKind::Sketch,
            vec![], // No parameters
        ),
        statements: workbench_statements([WorkbenchStatement::expr(call_circle(length(4.0)))]),
    };

    call_workbench("circle_without_parameter", &workbench, []);
}

/// sketch Circle(radius: Length) { __mu::geo2d::Circle(radius); }
///
/// Call single: Circle(4.0mm)
/// Call multi: Circle([1.0mm, 2.0mm, 3.0mm]);
#[test]
fn circle_parameter() {
    use helper::*;

    let workbench = symbol::Workbench {
        signature: workbench::WorkbenchSignature::new(
            symbol::WorkbenchKind::Sketch,
            vec![Parameter::new("radius", Type::length())],
        ),
        statements: workbench_statements([WorkbenchStatement::expr(call_circle(Path::Resolved(
            SymbolId::Local("radius".into()),
        )))]),
    };

    {
        let radius = Length::mm(4.0);
        let model = call_workbench(
            "circle_parameter_single",
            &workbench,
            [argument_value!(radius = radius)],
        );

        let prop = model.get_property_value("radius");
        assert_eq!(prop, Value::from(radius));
    }

    {
        let radius = list![Length::mm(1.0), Length::mm(2.0), Length::mm(3.0)];
        let model = call_workbench(
            "circle_parameter_multi",
            &workbench,
            [argument_value!(radius = radius.clone())],
        );

        let prop = model.get_property_value("radius");
        assert_eq!(prop, Value::from(radius));
    }
}

/// sketch Circle(radius: Length) {
///    init(diameter: Length) {
///        radius = diameter / 2;
///    }
///    __mu::geo2d::Circle(radius);
/// }
#[test]
fn circle_init() {
    use helper::*;

    let workbench = symbol::Workbench {
        signature: workbench::WorkbenchSignature::new(
            symbol::WorkbenchKind::Sketch,
            vec![Parameter::new("radius", Type::length())],
        )
        .with_inits([workbench::Init::default_init(vec![Parameter::new(
            "diameter",
            Type::length(),
        )])
        .with_statements([workbench::InitStatement::new(
            "radius",
            workbench::WorkbenchCall::builtin(__mu!(core::div)).with_args(
                workbench::ArgumentList::from_iter([
                    workbench::Argument::named("lhs", local("diameter")),
                    workbench::Argument::named("rhs", symbol::ConstantValue::from_value(2.0)),
                ]),
            ),
        )])]),
        statements: workbench_statements([WorkbenchStatement::expr(call_circle(Path::Resolved(
            SymbolId::Local("radius".into()),
        )))]),
    };

    let diameter = Length::mm(8.0);
    let model = call_workbench(
        "circle_init",
        &workbench,
        [argument_value!(diameter = diameter)],
    );

    let prop = model.get_property_value("radius");
    assert_eq!(prop, Value::from(Length::mm(4.0)));
}

/// a = 32mm;
/// __mu::geo2d::Circle(radius = a);
#[test]
fn circle_source() {
    use helper::*;

    let source = symbol::Source {
        statements: source_statements([
            SourceStatement::assignment("a", length(32.0)),
            SourceStatement::expr(call_circle(local("a"))),
        ]),
    };

    eval_to_model_test("source", source);
}

/// Evaluate [1..n]
#[test]
fn polar_expr_test() {
    use helper::*;

    let source = symbol::Source {
        statements: source_statements([
            SourceStatement::assignment("n", integer(4)),
            SourceStatement::assignment("a", polar_expr()),
            SourceStatement::expr(local("a")),
        ]),
    };

    eval_to_model_test("polar_expr_test", source);
}

/// An operation that rotates a part.
///
/// op rotate(matrix: Matrix3) {
///    init(angle: Angle, axis = __mu::math::Z) {
///        matrix = __mu::math::rotate_around_axis(angle, x = axis.x, y = axis.y, z = axis.z);
///    }
///    init(x = 0°, y = 0°, z = 0°) {
///        matrix = __mu::math::rotate_xyz(x, y, z);
///    init(roll = 0°, pitch = 0°, yaw = 0°) {
///        matrix = __mu::math::rotate_xyz(roll, pitch, yaw);
///    }
///    @input.__mu::ops::rotate(matrix)
/// }
#[test]
fn op_rotate() {
    use helper::*;
    use microcad_lang_types::Type;
    use symbol::workbench::Argument as Arg;

    let workbench = symbol::Workbench {
        signature: workbench::WorkbenchSignature::op(vec![Parameter::new("matrix", Type::mat3())])
            .with_inits([
                // init(angle: Angle, axis = __mu::math::Z)
                workbench::Init::new(vec![
                    Parameter::new("angle", Type::angle()),
                    Parameter::new("axis", Type::Any).with_default(
                        constant::ConstantExpression::from(microcad_builtin::mu::math::Z.value()),
                    ),
                ])
                .with_statements([workbench::InitStatement::new(
                    "matrix",
                    call(
                        __mu!(math::rotate_around_axis),
                        vec![
                            Arg::named("angle", local("angle")),
                            Arg::named("x", get(local("axis"), "x")),
                            Arg::named("y", get(local("axis"), "y")),
                            Arg::named("z", get(local("axis"), "z")),
                        ],
                    ),
                )]),
                // init(x = 0°, y = 0°, z = 0°)
                workbench::Init::new(vec![
                    Parameter::new("x", Type::angle()).with_default(deg(0.0)),
                    Parameter::new("y", Type::angle()).with_default(deg(0.0)),
                    Parameter::new("z", Type::angle()).with_default(deg(0.0)),
                ])
                .with_statements([workbench::InitStatement::new(
                    "matrix",
                    call(
                        __mu!(math::rotate_xyz),
                        vec![
                            Arg::named("x", local("x")),
                            Arg::named("y", local("y")),
                            Arg::named("z", local("z")),
                        ],
                    ),
                )]),
                // init(roll = 0°, pitch = 0°, yaw = 0°)
                workbench::Init::new(vec![
                    Parameter::new("roll", Type::angle()).with_default(deg(0.0)),
                    Parameter::new("pitch", Type::angle()).with_default(deg(0.0)),
                    Parameter::new("yaw", Type::angle()).with_default(deg(0.0)),
                ])
                .with_statements([workbench::InitStatement::new(
                    "matrix",
                    call(
                        __mu!(math::rotate_xyz),
                        vec![
                            Arg::named("x", local("roll")),
                            Arg::named("y", local("pitch")),
                            Arg::named("z", local("yaw")),
                        ],
                    ),
                )]),
            ]),
        statements: workbench_statements([WorkbenchStatement::expr(call(
            __mu!(ops::rotate),
            vec![
                workbench::Argument::named("self", input()),
                workbench::Argument::named("matrix", local("matrix")),
            ],
        ))]),
    };

    let mut context = EvalContext::new();
    let input_shape: Value = call_circle(length(4.0)).eval(&mut context).unwrap();

    // Test calling (angle = 90.0, axis = (x = 0, y = 0, z = 1)) via init(angle, axis)
    call_workbench(
        "rotate_angle_axis",
        &workbench,
        [
            argument_value!(angle = Value::deg(90.0)),
            argument_value!(axis = tuple!(x = 0.0, y = 0.0, z = 1.0)),
        ],
    );

    // Test calling (angle = [90.0, axis = (x = 0, y = 0, z = 0)) via init(angle, axis)
    {
        let model = call_workbench(
            "rotate_angle_axis_multi",
            &workbench,
            [
                argument_value!(
                    angle = Value::list([Value::deg(0.0), Value::deg(45.0), Value::deg(90.0)])
                ),
                argument_value!(axis = tuple!(x = 0.0, y = 0.0, z = 1.0)),
            ],
        );

        let model = model.replace_input_placeholders(input_shape);

        insta::assert_snapshot!("rotate_model_replaced", model.to_string_with_ctx(&context));
    }

    // Test calling (angle = 90.0) via init(angle, axis)
    call_workbench(
        "rotate_angle_axis_default_arg",
        &workbench,
        [argument_value!(angle = Value::deg(90.0))],
    );
}

/*
/// op polar_array(n: Integer, r: Length) {
///     @input.__mu::ops::translate(x = r, y = 0.0mm, z = 0.0mm)
///         .rotate([1..n] / n * 360°)
/// }
#[test]
fn op_polar_array() {
    use helper::*;
    use symbol::workbench::Argument as Arg;

    let polar_expr: WorkbenchExpression = polar_expr().into();
    let rotate = call(__mu!(ops::rotate), vec![
        Arg::named("m", microcad_lang_types::math::rotate_around_axis(p, x, y, z))
    ]);

    let workbench = symbol::Workbench {
        signature: workbench::WorkbenchSignature::op(vec![
            Parameter::new("n", Type::Integer),
            Parameter::new("r", Type::length()),
        ]),
        statements: workbench_statements([
            call_op(__mu!(ops::rotate), vec![
                ::named("self", call_op(__mu!(ops::translate), vec![
                    workbench::Argument::named()
                ])),
            ])

            WorkbenchStatement::expr(call_circle(Path::Resolved(
            SymbolId::Local("radius".into()),
        )))]),
    };
}
*/
