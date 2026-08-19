// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for evaluating workbenches.

use microcad_builtin::__mu;
use microcad_lang_base::{Identifier, SrcRef, SymbolId, ToCompactString};
use microcad_lang_eval::{CallTrait, EvalContext};
use microcad_lang_types::{ArgumentValueList, Length, Type, Value, argument_value};
use microcad_package::symbol::{
    self, Attributes, ConstantValue, Parameter, Path, Workbench, WorkbenchExpression,
    WorkbenchStatement,
    workbench::{self, Argument, ArgumentList, Call, Group, If},
};

/// sketch Circle() { __mu::geo2d::Circle(radius = 4.0mm); }
#[test]
fn circle_no_parameters() {
    let workbench = symbol::Workbench {
        attr: Attributes::default(),
        signature: symbol::workbench::WorkbenchSignature::new(
            symbol::WorkbenchKind::Sketch,
            vec![], // No parameters
        ),
        statements: vec![symbol::WorkbenchStatement::new(
            symbol::workbench::Call::builtin(__mu!(geo2d::Circle)).with_args(vec![
                Argument::named("radius", ConstantValue::from_value(Length::mm(4.0))),
            ]),
        )]
        .into_boxed_slice(),
    };

    let mut context = EvalContext::new();
    let result = workbench
        .call(&ArgumentValueList::default(), &mut context)
        .expect("No eval error");
}
