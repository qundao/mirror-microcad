// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::resolve::Sources;
use microcad_lang::src_ref::{Refer, SrcReferrer};
use microcad_lang::{
    eval::{Capture, EvalContext},
    model::Model,
    syntax::SourceFile,
};
use microcad_test_tools::test_env::*;
use miette::Report;
use std::rc::Rc;

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
        // insert UTF-8 Byte Order Mark (BOM)
        env.log_ln(std::str::from_utf8(&[0xEF_u8, 0xBB_u8, 0xBF_u8]).expect("test error"));

        env.log_ln(&format!("-- Test --\n{env:?}"));
        env.log_ln(&format!(
            "-- Code --\n\n{}\n",
            env.code()
                .lines()
                .enumerate()
                .map(|(n, line)| format!("{n:4}:   {line}", n = env.offset_line(n)))
                .collect::<Vec<_>>()
                .join("\n")
        ));

        // load and handle µcad source file
        let (source, errors) = SourceFile::load_from_str_with_recovery(
            Some(env.name()),
            env.source_path(),
            env.code(),
        );
        let sources =
            Sources::load(source.clone(), &<Vec<&str>>::new()).expect("no externals to fail");
        let render_options = DiagRenderOptions {
            color: false,
            ..Default::default()
        };

        match env.mode() {
            // test is expected to fail?
            "fail" | "todo_fail" => match errors {
                // test expected to fail failed at parsing?
                Some(errors) => {
                    let mut error_lines = std::collections::HashSet::new();
                    for err in errors {
                        if let Some(line) = err.src_ref().line() {
                            error_lines.insert(line + env.offset() - 1);
                        }
                        env.log_ln("-- Parse Error --");
                        let src_ref = err.src_ref();
                        let diag = Diagnostic::Error(Refer::new(Report::from(err), src_ref));
                        env.log_ln(&diag.to_pretty_string(&sources, env.offset(), &render_options));
                    }
                    if env.has_error_markers() {
                        if env.report_wrong_errors(&error_lines, &std::collections::HashSet::new())
                        {
                            env.result(TestResult::FailWrong);
                            panic!("ERROR: test is marked to fail but with wrong errors/warnings");
                        }
                    } else if env.todo() {
                        env.result(TestResult::NotTodoFail);
                    } else {
                        env.result(TestResult::FailOk);
                    }
                }
                // test expected to fail succeeded at parsing?
                None => {
                    // evaluate the code including µcad std library
                    let mut context = create_context(&source, env.offset());
                    let eval = context.eval();

                    env.report_output(context.output());
                    env.report_errors(context.diagnosis());

                    let err_warn =
                        env.report_wrong_errors(&context.error_lines(), &context.warning_lines());
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
                            if err_warn {
                                env.result(TestResult::FailWrong);
                                panic!(
                                    "ERROR: test is marked to fail but with wrong errors/warnings"
                                );
                            }
                            env.result(TestResult::FailOk);
                        }
                        // evaluation produced errors?
                        (_, true, false) => {
                            if err_warn {
                                env.result(TestResult::FailWrong);
                                panic!(
                                    "ERROR: test is marked to fail but with wrong errors/warnings"
                                );
                            }
                            env.result(TestResult::FailOk);
                            log::debug!(
                                "there were {error_count} errors (see {log:?})",
                                log = env.log_file(),
                                error_count = context.error_count()
                            );
                        }
                        // test fails as expected but is todo
                        (Err(_), _, true) | (_, true, true) => {
                            if err_warn {
                                env.result(TestResult::TodoFail)
                            } else {
                                env.result(TestResult::NotTodoFail)
                            }
                        }
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
            "ok" | "todo" | "warn" | "todo_warn" | "no_output" => match errors {
                // test awaited to succeed and parsing failed?
                Some(errors) => {
                    let first_err = errors[0].to_string();
                    for err in errors {
                        env.log_ln("-- Parse Error --");
                        let src_ref = err.src_ref();
                        let diag = Diagnostic::Error(Refer::new(Report::from(err), src_ref));
                        env.log_ln(&diag.to_pretty_string(&sources, env.offset(), &render_options));
                    }

                    if env.todo() {
                        env.result(TestResult::Todo);
                    } else if env.has_error_markers() {
                        env.result(TestResult::FailWrong);
                        panic!("ERROR: test is marked to fail but with wrong errors/warnings");
                    } else {
                        env.result(TestResult::Fail);
                        panic!("ERROR: {first_err}")
                    }
                }
                // test awaited to succeed and parsing succeeds?
                None => {
                    // evaluate the code including µcad std library
                    let mut context = create_context(&source, env.offset());
                    let eval = context.eval();

                    env.report_output(context.output());
                    env.report_errors(context.diagnosis());
                    let err_warn =
                        env.report_wrong_errors(&context.error_lines(), &context.warning_lines());

                    let _ = fs::remove_file(env.banner_file());

                    // check if test awaited to succeed but failed at evaluation
                    match (eval, context.has_errors(), env.todo()) {
                        // test expected to succeed and succeeds with no errors
                        (Ok(model), false, false) => match report_model(&mut env, model) {
                            Ok(_) => {
                                if err_warn {
                                    match env.mode() {
                                        "warn" => {
                                            env.result(TestResult::OkWrong);
                                            panic!(
                                                "ERROR: test is marked to fail but with wrong errors/warnings"
                                            );
                                        }
                                        "todo_warn" => {
                                            env.result(TestResult::TodoWarn);
                                        }
                                        _ => env.result(TestResult::OkWarn),
                                    }
                                } else {
                                    env.result(TestResult::Ok)
                                }
                            }
                            Err(err) => {
                                if env.mode() != "no_output" {
                                    env.log_ln(&format!("Export failed: {err}."));
                                    env.result(TestResult::Fail);
                                    panic!(
                                        "ERROR: Export error: {err} (see {log:?}).",
                                        log = env.log_file(),
                                    );
                                }
                                env.result(TestResult::Ok)
                            }
                        },
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
            _ => unreachable!("Unknown mode: {}", env.mode()),
        }
    }
}

// evaluate the code including µcad std library
fn create_context(source: &Rc<SourceFile>, line_offset: usize) -> EvalContext {
    let mut context = EvalContext::from_source(
        source.clone(),
        Some(microcad_builtin::builtin_module()),
        &["../crates/std/lib", "../assets"],
        Capture::new(),
        microcad_builtin::builtin_exporters(),
        microcad_builtin::builtin_importers(),
        line_offset - 1,
    )
    .expect("resolve error");
    context.diag.render_options.color = false;
    context
}

fn report_model(
    env: &mut TestEnv,
    model: Option<Model>,
) -> Result<(), microcad_lang::builtin::ExportError> {
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
            Some(export) => {
                export.render_and_export(&model)?;
                env.log_ln(&format!("Export of {:?} successful.", export.filename));
            }
            None => env.log_ln("Nothing will be exported."),
        }
    } else {
        env.log_ln("-- No Model --");
    }

    Ok(())
}
