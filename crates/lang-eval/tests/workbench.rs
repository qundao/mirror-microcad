// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for evaluating workbenches.

use microcad_builtin::__mu;
use microcad_lang_base::SrcRef;
use microcad_lang_eval::{CallTrait, Eval, EvalContext};
use microcad_lang_types::{ArgumentValueList, Length, ModelTree, Value};
use microcad_package::{
    SymbolId,
    symbol::{
        self, Attributes, ConstantValue, Path, WorkbenchExpression, WorkbenchStatement, workbench,
    },
};

/// Expressions used for testing
pub mod helper {

    use microcad_package::symbol::WorkbenchStatement;

    use super::*;

    /// `4.0mm`
    pub fn expr_length(v: f64) -> WorkbenchExpression {
        ConstantValue::from_value(Length::mm(v)).into()
    }

    /// __mu::geo2d::Circle(radius = expr)
    pub fn call_circle(expr: impl Into<WorkbenchExpression>) -> symbol::WorkbenchExpression {
        workbench::Call::builtin(__mu!(geo2d::Circle))
            .with_args(vec![workbench::Argument::named("radius", expr)])
            .into()
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
        statements: statements([WorkbenchStatement::expr(call_circle(expr_length(4.0)))]),
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
            WorkbenchStatement::prop("a", expr_length(4.0)),
            WorkbenchStatement::expr(call_circle(Path::Resolved(SymbolId::Local("a".into())))),
        ]),
    };

    let mut context = EvalContext::new();
    let model: ModelTree = group.eval(&mut context).expect("No error");

    let prop = model.get_property("a").expect("A property");
    assert_eq!(prop.value, Value::from(Length::mm(4.0)));

    insta::assert_snapshot!("group_with_property", model)
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
        statements: statements([WorkbenchStatement::expr(call_circle(expr_length(4.0)))]),
    };

    let mut context = EvalContext::new();
    let model = workbench
        .call(&ArgumentValueList::default(), &mut context)
        .expect("No eval error");

    insta::assert_snapshot!("circle_without_parameter", model)
}

/// sketch Circle(radius: Length) { __mu::geo2d::Circle(radius); }
#[test]
fn circle_parameter() {
    todo!()
}

/// sketch Circle(radius: Length) {
///    init(diameter: Length) {
///        radius = diameter / 2;
///    }
///    __mu::geo2d::Circle(radius);
/// }
#[test]
fn circle_init() {
    todo!()
}

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
}
