// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::hash::HashSet;
use microcad_lang::resolve::Sources;
use microcad_lang::{eval::EvalContext, lower::ir::SourceFile, model::Model};
use microcad_lang_base::{
    Capture, Diag, DiagRenderOptions, Diagnostic, FormatTree, Refer, SrcReferrer,
};
use microcad_test_tools::test_env::*;
use miette::Report;
use std::rc::Rc;

#[allow(dead_code)]
pub fn run_test(env: TestEnv) -> std::io::Result<()> {
    use std::fs;
    env_logger::try_init().ok();

    log::info!("Running test:\n{env:?}");

    // remove generated files before updating
    let _ = fs::remove_file(env.banner_file());
    let _ = fs::remove_file(env.log_file());

    let _ = fs::hard_link("images/parse_fail.svg", env.banner_file());

    let mut log_file =
        std::fs::File::create(env.log_file()).unwrap_or_else(|_| panic!("{:?}", env.log_file()));
    let log = &mut std::io::BufWriter::new(&mut log_file);
    use std::io::Write;

    // insert UTF-8 Byte Order Mark (BOM)
    writeln!(
        log,
        "{}",
        std::str::from_utf8(&[0xEF_u8, 0xBB_u8, 0xBF_u8]).expect("test error")
    )?;

    writeln!(log, "-- Test --\n{env:?}")?;

    writeln!(
        log,
        "-- Code --\n\n{}\n",
        env.code()
            .lines()
            .enumerate()
            .map(|(n, line)| format!("{n:4}:   {line}", n = n as u32 + env.line_offset))
            .collect::<Vec<_>>()
            .join("\n")
    )?;

    // load and handle µcad source file
    let (source, errors) = SourceFile::load_from_str_with_recovery(
        Some(env.name()),
        env.source_path(),
        env.code(),
        env.line_offset,
    );
    let sources = Sources::load(source.clone(), &<Vec<&str>>::new()).expect("no externals to fail");
    let render_options = DiagRenderOptions {
        color: false,
        ..Default::default()
    };

    let result = match env.mode() {
        // test is expected to fail?
        "fail" | "todo_fail" => match errors {
            // test expected to fail failed at parsing?
            Some(errors) => {
                let mut error_lines = HashSet::default();
                for err in errors {
                    if let Some(line) = err.src_ref().line() {
                        error_lines.insert(line);
                    }
                    writeln!(log, "-- Parse Error --")?;
                    let src_ref = err.src_ref();
                    let diag = Diagnostic::Error(Refer::new(Report::from(err), src_ref));
                    writeln!(log, "{}", &diag.to_pretty_string(&sources, &render_options))?;
                }
                if env.has_error_markers() {
                    if let Some(msg) = env.report_wrong_errors(&error_lines, &HashSet::default()) {
                        writeln!(log, "{msg}")?;
                        TestResult::FailWrong
                    } else {
                        TestResult::FailOk
                    }
                } else if env.todo() {
                    TestResult::NotTodoFail
                } else {
                    TestResult::FailOk
                }
            }
            // test expected to fail succeeded at parsing?
            None => {
                // evaluate the code including µcad std library
                let mut context = create_context(&source);
                let eval = context.eval();

                writeln!(log, "{}", env.report_output(context.output()))?;
                writeln!(log, "{}", env.report_errors(context.diagnosis()))?;

                let err_warn =
                    env.report_wrong_errors(&context.error_lines(), &context.warning_lines());
                if let Some(msg) = &err_warn {
                    writeln!(log, "{msg}")?;
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
                        writeln!(log, "{err}")?;
                        if err_warn.is_some() {
                            TestResult::FailWrong
                        } else {
                            TestResult::FailOk
                        }
                    }
                    // evaluation produced errors?
                    (_, true, false) => {
                        if err_warn.is_some() {
                            TestResult::FailWrong
                        } else {
                            log::debug!(
                                "there were {error_count} errors (see {log:?})",
                                log = env.log_file(),
                                error_count = context.error_count()
                            );
                            TestResult::FailOk
                        }
                    }
                    // test fails as expected but is todo
                    (Err(_), _, true) | (_, true, true) => {
                        if err_warn.is_some() {
                            TestResult::TodoFail
                        } else {
                            TestResult::NotTodoFail
                        }
                    }
                    // test expected to fail but succeeds and is todo to fail?
                    (_, _, true) => TestResult::TodoFail,
                    // test expected to fail but succeeds?
                    (_, _, false) => TestResult::OkFail,
                }
            }
        },
        // test is expected to succeed?
        "ok" | "todo" | "warn" | "todo_warn" => match errors {
            // test awaited to succeed and parsing failed?
            Some(errors) => {
                for err in errors {
                    writeln!(log, "-- Parse Error --")?;
                    let src_ref = err.src_ref();
                    let diag = Diagnostic::Error(Refer::new(Report::from(err), src_ref));
                    writeln!(log, "{}", diag.to_pretty_string(&sources, &render_options))?;
                }

                if env.todo() {
                    TestResult::Todo
                } else if env.has_error_markers() {
                    TestResult::FailWrong
                } else {
                    TestResult::Fail
                }
            }
            // test awaited to succeed and parsing succeeds?
            None => {
                // evaluate the code including µcad std library
                let mut context = create_context(&source);
                let eval = context.eval();

                writeln!(log, "{}", env.report_output(context.output()))?;
                writeln!(log, "{}", env.report_errors(context.diagnosis()))?;
                let err_warn =
                    env.report_wrong_errors(&context.error_lines(), &context.warning_lines());
                if let Some(msg) = &err_warn {
                    writeln!(log, "{msg}")?;
                }

                let _ = fs::remove_file(env.banner_file());

                // check if test awaited to succeed but failed at evaluation
                match (eval, context.has_errors(), env.todo()) {
                    // test expected to succeed and succeeds with no errors
                    (Ok(model), false, false) => {
                        report_model(&env, log, model)?;
                        if err_warn.is_some() {
                            match env.mode() {
                                "warn" => TestResult::OkWrong,
                                "todo_warn" => TestResult::TodoWarn,
                                _ => TestResult::OkWarn,
                            }
                        } else {
                            TestResult::Ok
                        }
                    }
                    // test is todo but succeeds with no errors
                    (Ok(_), false, true) => TestResult::NotTodo,
                    // Any error but todo
                    (_, _, true) => TestResult::Todo,
                    // evaluation had been aborted?
                    (Err(err), _, _) => {
                        writeln!(log, "{err}")?;
                        TestResult::Fail
                    }
                    // evaluation produced errors?
                    (_, true, _) => TestResult::Fail,
                }
            }
        },
        "fail(no_format)" => {
            return Ok(()); // HOTFIX
        }
        _ => unreachable!(),
    };

    writeln!(log, "{}", env.result(&result))?;

    match result {
        TestResult::Fail => panic!("ERROR"),
        TestResult::OkWrong | TestResult::FailWrong => {
            panic!("ERROR: test is marked to fail but with wrong errors/warnings")
        }
        TestResult::OkFail => panic!("ERROR: test is marked to fail but succeeded"),
        _ => Ok(()),
    }
}

// evaluate the code including µcad std library
fn create_context(source: &Rc<SourceFile>) -> EvalContext {
    let mut context = EvalContext::from_source(
        source.clone(),
        Some(microcad_builtin::builtin_module()),
        &["../crates/std/lib", "../assets"],
        Capture::new(),
        microcad_builtin::builtin_exporters(),
        microcad_builtin::builtin_importers(),
    )
    .expect("resolve error");
    context.diag.render_options.color = false;
    context
}

fn report_model(
    env: &TestEnv,
    log: &mut dyn std::io::Write,
    model: Option<Model>,
) -> std::io::Result<()> {
    use microcad_core::RenderResolution;
    use microcad_export::{stl::StlExporter, svg::SvgExporter};
    use microcad_lang::model::{ExportCommand as Export, OutputType};

    // print model
    if let Some(model) = model {
        writeln!(log, "-- Model --\n{}", FormatTree(&model))?;

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
                writeln!(log, "Could not determine output type.")?;
                None
            }
            _ => panic!("Invalid geometry output"),
        };
        match export {
            Some(export) => match export.render_and_export(&model) {
                Ok(_) => writeln!(log, "Export of {:?} successful.", export.filename),
                Err(error) => writeln!(log, "Export error: {error}"),
            },
            None => writeln!(log, "Nothing will be exported."),
        }
    } else {
        writeln!(log, "-- No Model --")
    }
}
