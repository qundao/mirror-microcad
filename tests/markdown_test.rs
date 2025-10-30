// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_lang::{
    eval::{Capture, EvalContext},
    model::Model,
    syntax::SourceFile,
};
use microcad_test_tools::test_env::*;

fn lines_with(code: &str, marker: &str) -> std::collections::HashSet<usize> {
    code.lines()
        .enumerate()
        .filter_map(|line| {
            if line.1.contains(marker) {
                Some(line.0 + 1)
            } else {
                None
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn init() {
    let _ = env_logger::builder().try_init();
}

#[allow(dead_code, clippy::too_many_arguments)]
pub fn run_test(env: Option<TestEnv>) {
    if let Some(mut env) = env {
        use microcad_lang::{diag::*, syntax::*};
        use std::fs;

        crate::markdown_test::init();

        log::info!("Running test:\n{env:?}");

        // remove generated files before updating
        let _ = fs::remove_file(env.banner_file());
        let _ = fs::remove_file(env.log_file());

        let _ = fs::hard_link("images/parse_fail.svg", env.banner_file());

        env.start_log();

        env.log_ln(&format!(
            "-- Test --\n  {name}\n  {reference}\n",
            name = env.name(),
            reference = env.reference()
        ));
        env.log_ln(&format!(
            "-- Code --\n\n{}\n",
            env.code()
                .lines()
                .enumerate()
                .map(|(n, line)| format!("{n:2}: {line}", n = n + 1))
                .collect::<Vec<_>>()
                .join("\n")
        ));

        // load and handle µcad source file
        let source_file_result =
            SourceFile::load_from_str(env.name(), env.source_path(), env.code());

        match env.mode() {
            // test is expected to fail?
            "fail" | "todo_fail" | "warn" | "todo_warn" => match source_file_result {
                // test expected to fail failed at parsing?
                Err(err) => {
                    env.log_ln("-- Parse Error --");
                    env.log_ln(&err.to_string());
                    env.result(TestResult::FailOk);
                }
                // test expected to fail succeeded at parsing?
                Ok(source) => {
                    // evaluate the code including µcad std library
                    let mut context = create_context(&source);
                    let eval = context.eval();

                    env.report_output(context.output());
                    env.report_errors(context.diagnosis());

                    if !env.todo()
                        && ((context.has_errors()
                            && lines_with(env.code(), "// error") != context.error_lines())
                            || (context.has_warnings()
                                && lines_with(env.code(), "// warning") != context.warning_lines()))
                    {
                        env.result(TestResult::FailWrong);
                        panic!("ERROR: test is marked to fail but fails with wrong errors");
                    }

                    let _ = fs::remove_file(env.banner_file());

                    // check if test expected to fail failed at evaluation
                    match (
                        eval,
                        context.has_errors() || context.has_warnings(),
                        env.todo(),
                    ) {
                        // evaluation had been aborted?
                        (Err(err), _, false) => {
                            env.log_ln(&err.to_string());
                            env.result(TestResult::FailOk);
                        }
                        // evaluation produced errors?
                        (_, true, false) => {
                            env.result(TestResult::FailOk);
                            log::debug!(
                                "there were {error_count} errors (see {log:?})",
                                log = env.log_file(),
                                error_count = context.error_count()
                            );
                        }
                        // test fails as expected but is todo
                        (Err(_), _, true) | (_, true, true) => env.result(TestResult::NotTodoFail),
                        // test expected to fail but succeeds and is todo to fail?
                        (_, _, true) => env.result(TestResult::TodoFail),
                        // test expected to fail but succeeds?
                        (_, _, false) => {
                            env.result(TestResult::OkFail);
                            panic!("ERROR: test is marked to fail but succeeded");
                        }
                    }
                }
            },
            // test is expected to succeed?
            "ok" | "todo" => match source_file_result {
                // test awaited to succeed and parsing failed?
                Err(err) => {
                    env.log_ln("-- Parse Error --");
                    env.log_ln(&err.to_string());

                    if env.todo() {
                        env.result(TestResult::Todo);
                    } else {
                        env.result(TestResult::Fail);
                        panic!("ERROR: {err}")
                    }
                }
                // test awaited to succeed and parsing succeeds?
                Ok(source) => {
                    // evaluate the code including µcad std library
                    let mut context = create_context(&source);
                    let eval = context.eval();

                    env.report_output(context.output());
                    env.report_errors(context.diagnosis());

                    let _ = fs::remove_file(env.banner_file());

                    // check if test awaited to succeed but failed at evaluation
                    match (
                        eval,
                        context.has_errors() || context.has_warnings(),
                        env.todo(),
                    ) {
                        // test expected to succeed and succeeds with no errors
                        (Ok(model), false, false) => {
                            report_model(&mut env, model);
                            env.result(TestResult::Ok);
                        }
                        // test is todo but succeeds with no errors
                        (Ok(_), false, true) => {
                            env.result(TestResult::NotTodo);
                        }
                        // Any error but todo
                        (_, _, true) => {
                            env.result(TestResult::Todo);
                        }
                        // evaluation had been aborted?
                        (Err(err), _, _) => {
                            env.log_ln(&err.to_string());
                            env.result(TestResult::Fail);
                            panic!("ERROR: {err}")
                        }
                        // evaluation produced errors?
                        (_, true, _) => {
                            env.result(TestResult::Fail);
                            panic!(
                                "ERROR: There were {error_count} errors (see {log:?}).",
                                log = env.log_file(),
                                error_count = context.error_count()
                            );
                        }
                    }
                }
            },
            "no-test" => (),
            _ => unreachable!(),
        }
    }
}

// evaluate the code including µcad std library
fn create_context(source: &Rc<SourceFile>) -> EvalContext {
    EvalContext::from_source(
        source.clone(),
        Some(microcad_builtin::builtin_module()),
        &["../lib", "../lang/doc/assets"],
        Capture::new(),
        microcad_builtin::builtin_exporters(),
        microcad_builtin::builtin_importers(),
    )
    .expect("resolve error")
}

fn report_model(env: &mut TestEnv, model: Option<Model>) {
    use microcad_core::RenderResolution;
    use microcad_export::{stl::StlExporter, svg::SvgExporter};
    use microcad_lang::{
        model::{ExportCommand as Export, OutputType},
        tree_display::FormatTree,
    };

    // print model
    if let Some(model) = model {
        env.log_ln(&format!("-- Model --\n{}", FormatTree(&model)));

        let export = match model.deduce_output_type() {
            OutputType::Geometry2D => Some(Export {
                filename: env.out_file("svg"),
                resolution: RenderResolution::default(),
                exporter: Rc::new(SvgExporter),
            }),
            OutputType::Geometry3D => Some(Export {
                filename: env.out_file("stl"),
                resolution: if env.hires() {
                    RenderResolution::default()
                } else {
                    RenderResolution::coarse()
                },
                exporter: Rc::new(StlExporter),
            }),
            OutputType::NotDetermined => {
                env.log_ln("Could not determine output type.");
                None
            }
            _ => panic!("Invalid geometry output"),
        };
        match export {
            Some(export) => match export.render_and_export(&model) {
                Ok(_) => env.log_ln(&format!("Export of {:?} successful.", export.filename)),
                Err(error) => env.log_ln(&format!("Export error: {error}")),
            },
            None => env.log_ln("Nothing will be exported."),
        }
    } else {
        env.log_ln("-- No model --");
    }
}
