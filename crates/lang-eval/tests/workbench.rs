// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for evaluating workbenches.

use microcad_builtin::__mu;
use microcad_lang_base::SrcRef;
use microcad_lang_eval::{CallTrait, Eval, EvalContext};
use microcad_lang_types::{
    ArgumentValueList, Length, ModelTree, Type, Value, argument_value, list,
};
use microcad_package::{
    SymbolId,
    symbol::{self, Attributes, ConstantValue, Parameter, Path, WorkbenchStatement, workbench},
};

/// Expressions used for testing
pub mod helper {
    use microcad_package::symbol::{WorkbenchExpression as Expr, WorkbenchStatement};

    use super::*;

    /// `4.0mm`
    pub fn length(v: f64) -> Expr {
        ConstantValue::from_value(Length::mm(v)).into()
    }

    /// __mu::geo2d::Circle(radius = expr)
    pub fn call_circle(radius: impl Into<Expr>) -> Expr {
        workbench::Call::builtin(__mu!(geo2d::Circle))
            .with_args(vec![workbench::Argument::named("radius", radius)])
            .into()
    }

    /// __mu::op::translate(self, x, y, z)
    pub fn call_translate(
        self_: impl Into<Expr>,
        x: impl Into<Expr>,
        y: impl Into<Expr>,
        z: impl Into<Expr>,
    ) -> Expr {
        workbench::Call::builtin(__mu!(ops::translate))
            .with_args(vec![
                workbench::Argument::named("self", self_),
                workbench::Argument::named("x", x),
                workbench::Argument::named("y", y),
                workbench::Argument::named("z", z),
            ])
            .into()
    }

    /// Local expression with `name`
    pub fn local(name: &str) -> Expr {
        symbol::Path::Resolved(SymbolId::Local(name.into())).into()
    }

    pub fn statements(
        statements: impl IntoIterator<Item = WorkbenchStatement>,
    ) -> Box<[WorkbenchStatement]> {
        statements
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

/// { __mu::geo2d::Circle(radius = 4.0mm); }
#[test]
fn group() {
    use helper::*;

    let group = symbol::workbench::Group {
        src_ref: SrcRef::none(),
        attr: Attributes::default(),
        statements: statements([WorkbenchStatement::expr(call_circle(length(4.0)))]),
    };

    let mut context = EvalContext::new();
    let model: ModelTree = group.eval(&mut context).expect("No error");
    insta::assert_snapshot!("group", model)
}

/// {
///     prop a = 4.0mm; // Add property to current model tree
///     __mu::geo2d::Circle(a); // Add circle as new Model Tree
/// }
#[test]
fn group_with_property() {
    use helper::*;

    let group = symbol::workbench::Group {
        src_ref: SrcRef::none(),
        attr: Attributes::default(),
        statements: statements([
            WorkbenchStatement::prop("a", length(4.0)),
            WorkbenchStatement::expr(call_circle(local("a"))),
        ]),
    };

    let mut context = EvalContext::new();
    let model: ModelTree = group.eval(&mut context).expect("No error");

    let prop = model.get_property_value("a");
    assert_eq!(prop, Value::from(Length::mm(4.0)));

    insta::assert_snapshot!("group_with_property", model)
}

/// __mu::geo2d::Circle(radius = 4.0mm).translate(x = 1.0mm, y = 2.0mm, z = 0.0mm)
#[test]
fn translate_circle() {
    use helper::*;

    let expr = call_translate(
        call_circle(length(4.0)),
        length(1.0),
        length(2.0),
        length(0.0),
    );

    let mut context = EvalContext::new();
    let model: ModelTree = expr.eval(&mut context).expect("No error");
    let prop = model.get_property_value("radius"); // We should be able to access the property.
    assert_eq!(prop, Value::from(Length::mm(4.0)));

    insta::assert_snapshot!("translate_circle", model)
}

/// sketch Circle() { __mu::geo2d::Circle(radius = 4.0mm); }
#[test]
fn circle_without_parameter() {
    use helper::*;

    let workbench = symbol::Workbench {
        attr: Attributes::default(),
        signature: workbench::WorkbenchSignature::new(
            symbol::WorkbenchKind::Sketch,
            vec![], // No parameters
        ),
        statements: statements([WorkbenchStatement::expr(call_circle(length(4.0)))]),
    };

    let mut context = EvalContext::new();
    let model = workbench
        .call(&ArgumentValueList::default(), &mut context)
        .expect("No eval error");

    insta::assert_snapshot!("circle_without_parameter", model)
}

/// sketch Circle(radius: Length) { __mu::geo2d::Circle(radius); }
///
/// Call single: Circle(4.0mm)
/// Call multi: Circle([1.0mm, 2.0mm, 3.0mm]);
#[test]
fn circle_parameter() {
    use helper::*;

    let workbench = symbol::Workbench {
        attr: Attributes::default(),
        signature: workbench::WorkbenchSignature::new(
            symbol::WorkbenchKind::Sketch,
            vec![Parameter::new("radius", Type::length())],
        ),
        statements: statements([WorkbenchStatement::expr(call_circle(Path::Resolved(
            SymbolId::Local("radius".into()),
        )))]),
    };

    {
        let radius = Length::mm(4.0);
        let mut context = EvalContext::new();
        let model = workbench
            .call(
                &ArgumentValueList::from_iter([argument_value!(radius = radius)]),
                &mut context,
            )
            .expect("No eval error");

        let prop = model.get_property_value("radius");
        assert_eq!(prop, Value::from(radius));

        insta::assert_snapshot!("circle_parameter_single", model);
    }

    {
        let radius = list![Length::mm(1.0), Length::mm(2.0), Length::mm(3.0)];
        let mut context = EvalContext::new();
        let model = workbench
            .call(
                &ArgumentValueList::from_iter([argument_value!(radius = radius.clone())]),
                &mut context,
            )
            .expect("No eval error");

        let prop = model.get_property_value("radius");
        assert_eq!(prop, Value::from(radius));

        insta::assert_snapshot!("circle_parameter_multi", model);
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
        attr: Attributes::default(),
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
            workbench::Call::builtin(__mu!(core::div)).with_args(
                workbench::ArgumentList::from_iter([
                    workbench::Argument::named("lhs", local("diameter")),
                    workbench::Argument::named("rhs", symbol::ConstantValue::from_value(2.0)),
                ]),
            ),
        )])]),
        statements: statements([WorkbenchStatement::expr(call_circle(Path::Resolved(
            SymbolId::Local("radius".into()),
        )))]),
    };

    let diameter = Length::mm(8.0);
    let mut context = EvalContext::new();
    let model = workbench
        .call(
            &ArgumentValueList::from_iter([argument_value!(diameter = diameter)]),
            &mut context,
        )
        .expect("No eval error");

    let prop = model.get_property_value("radius");
    assert_eq!(prop, Value::from(Length::mm(4.0)));

    insta::assert_snapshot!("circle_init", model);
}

/*
/// sketch Circle(radius: Length) {
///     init(diameter: Length) {
///         radius = diameter / 2;
///     }
///     __mu::geo2d::Circle(radius)
///         .__mu::geo2d::translate(x = 10.0mm, y = 10.0mm, z = 0.0mm);
/// }
#[test]
fn circle_init_op() {
    todo!()
}*/
