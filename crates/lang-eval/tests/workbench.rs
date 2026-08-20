// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for evaluating workbenches.

use microcad_builtin::__mu;
use microcad_lang_base::SrcRef;
use microcad_lang_eval::{CallTrait, Eval, EvalContext};
use microcad_lang_types::{ArgumentValueList, Length, ModelTree};
use microcad_package::symbol::{self, Attributes, ConstantValue, WorkbenchStatement, workbench};

/// Expressions used for testing
pub mod helper {

    use microcad_package::symbol::WorkbenchStatement;

    use super::*;

    /// __mu::geo2d::Circle(radius)
    pub fn call_circle(radius: f64) -> symbol::WorkbenchExpression {
        workbench::Call::builtin(__mu!(geo2d::Circle))
            .with_args(vec![workbench::Argument::named(
                "radius",
                ConstantValue::from_value(Length::mm(radius)),
            )])
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

#[test]
fn group() {
    use helper::*;

    let group = symbol::workbench::Group {
        src_ref: SrcRef::none(),
        attr: Attributes::default(),
        statements: statements([WorkbenchStatement::expr(call_circle(4.0))]),
    };

    let mut context = EvalContext::new();
    use microcad_lang_eval::Eval;
    let model: ModelTree = group.eval(&mut context).expect("No error");

    println!("{model}");
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
        statements: statements([WorkbenchStatement::expr(call_circle(4.0))]),
    };

    let mut context = EvalContext::new();
    let result = workbench
        .call(&ArgumentValueList::default(), &mut context)
        .expect("No eval error");
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
