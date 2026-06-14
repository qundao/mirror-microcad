// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{self as base, Diagnostics, Identifier};
use microcad_lang_lower::{self as lower, ir};
use microcad_lang_parse as parse;

use test_that::prelude::*;

/// Get intermediate representation and diagnostics.
fn ir_from_test_file(name: &str) -> lower::LowerResult<(lower::ir::Source, Diagnostics)> {
    let path_string = format!("tests/test_cases/{name}.{}", base::MICROCAD_EXTENSION);
    let path = std::path::PathBuf::from(path_string);
    let abs_path = path.canonicalize().expect("No error");

    let source = base::Source {
        url: base::Url::from_file_path(abs_path).expect("No error"),
        line_offset: 0,
        code: base::Hashed::new(std::fs::read_to_string(path).expect("No error")),
    };

    use microcad_lang_parse::Parse;
    let ast = parse::ast::Source::parse(&parse::ParseContext::new(source.code()))
        .expect("No parse errors");

    use microcad_lang_lower::Lower;
    let mut context = lower::LowerContext::new(&ast.code);
    Ok((
        lower::ir::Source::lower(&ast, &mut context)?,
        context.diagnostics.clone(),
    ))
}

macro_rules! unit_test {
    ($name:ident => |$ir:ident, $diag:ident| $body:block) => {
        #[test_that::test]
        fn $name() {
            match ir_from_test_file(stringify!($name)) {
                Ok(($ir, $diag)) => $body,
                Err(err) => panic!("Error during lowering {err}"),
            }
        }
    };
}

macro_rules! snapshot_test {
    // A successful snapshot test without errors and warnings.
    ($name:ident => ok) => {
        #[test_that::test]
        fn $name() {
            let name = stringify!($name);
            match ir_from_test_file(name) {
                Ok((ir, diag)) => {
                    if diag.has_errors() || diag.has_warnings() {
                        panic!("{diag:?}");
                    }
                    insta::assert_snapshot!(name, lower::to_ron(&ir).expect("No error"));
                }
                Err(err) => panic!("Error during lowering {err}"),
            }
        }
    };

    // A successful snapshot test with errors.
    ($name:ident => error) => {
        #[test]
        fn $name() {
            let name = stringify!($name);
            match ir_from_test_file(name) {
                Ok((ir, diag)) => {
                    assert!(diag.has_errors());
                    insta::assert_snapshot!(name, lower::to_ron(&ir).expect("No error"));
                }
                Err(err) => panic!("Error during lowering {err}"),
            }
        }
    };
}

unit_test!(assignments_const_const_assignment_init => |ir, diag| {
    assert_that!(ir.items.workbenches, len(eq(1)));
    assert_that!(*ir.statements, len(eq(1)));

    assert!(diag.error_count() == 1);
});

macro_rules! pat {
    ($($any_tokens:tt)*) => {
        contains_exactly![matches_pattern!($($any_tokens)*)]
    };
}

unit_test!(assignments_const_const_assignment_mod => |ir, diag| {
    assert_that!(ir, matches_pattern!(ir::Source {
        *statements: len(eq(3)),
        items: matches_pattern!(ir::SourceItems {
            constants: len(eq(1)),
            *inline_modules: pat!(ir::InlineModule {
                visibility: eq(ir::Visibility::Private),
                items: matches_pattern!(ir::InlineModuleItems {
                    constants: len(eq(1)),
                    functions: pat!(ir::Function {
                        visibility: eq(ir::Visibility::Public),
                    }),
                    workbenches: pat!(ir::Workbench {
                        id: eq(Identifier::from("MySketch")),
                        visibility: eq(ir::Visibility::Public),
                    })
                })
            })
        })
    }));

    assert!(!diag.has_errors())
});

snapshot_test!(assignments_const_const_assignment_workbench_code => ok);
snapshot_test!(assignments_const_const_assignment_workbench => ok);
snapshot_test!(assignments_model_assignment_model_module => error);
snapshot_test!(assignments_model_assignment_model_workbench => error);

unit_test!(assignments_property_prop_assignment_fn => |ir, diag| {
    assert_eq!(ir.items.functions.len(), 1);
    assert!(diag.has_errors());
});

unit_test!(assignments_property_prop_assignment_init => |ir, diag| {
    assert_eq!(ir.items.workbenches.len(), 1);
    assert!(diag.has_errors());
});

unit_test!(assignments_property_prop_assignment_source => |ir, diag| {
    assert_eq!(diag.error_count(), 1);
});

snapshot_test!(assignments_property_prop_assignment => ok);

unit_test!(assignments_value_assignment_module => |ir, diag| {
    assert_eq!(diag.error_count(), 1);
});

/*

assignments_value_assignment_workbench
attributes_README_inner_attributes
attributes_README_outer_attributes
doc_comments_inner_doc_comment
doc_comments_outer_doc_comment
expressions_literals_boolean_literal
expressions_literals_expression_literals
expressions_literals_integer_literal
expressions_literals_quantity_literal
expressions_literals_scalar_literal
expressions_literals_string_literal
flow_conditions_if_expression
structure_functions_module_functions_mod
structure_functions_README_example
structure_functions_result_function_conditional_result
structure_functions_result_function_return
structure_functions_result_return_twice
structure_functions_workbench_functions_workbench_example
structure_functions_workbench_functions_workbench_fn_prop
structure_modules_inline_modules_inline_mod
structure_modules_README_mod_example
structure_use_use_all
structure_use_use_as
structure_use_use_module
structure_use_use_statement_pub
structure_use_use
structure_workbenches_elements_building_code_code_post_init
structure_workbenches_elements_building_code_code
structure_workbenches_elements_building_code_illegal_workbench_statement_mod
structure_workbenches_elements_building_code_illegal_workbench_statement_return
structure_workbenches_elements_building_code_illegal_workbench_statement_sketch
structure_workbenches_elements_init_code_pre_init_code
structure_workbenches_elements_initializers_code_between_initializers
structure_workbenches_elements_properties_property_no_prop_in_init_code
structure_workbenches_elements_properties_property
structure_workbenches_types_operations_op_example
structure_workbenches_types_parts_part_basic
structure_workbenches_types_sketches_sketch_basic

*/
