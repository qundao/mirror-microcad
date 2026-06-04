// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_driver::{
    commands::Render,
    prelude::{self as mu, RenderParameters},
};

use microcad_test_tools::test_env::*;
use std::rc::Rc;

#[allow(dead_code)]
pub fn run_test(env: TestEnv) -> std::io::Result<()> {
    use mu::traits::*;

    use std::fs;
    env_logger::try_init().ok();

    log::info!("Running test:\n{env:?}");

    // remove generated files before updating
    let _ = fs::remove_file(env.banner_file());
    let _ = fs::remove_file(env.log_file());

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
            .map(|(n, line)| format!("{n:4}:   {line}", n = n as u32 + env.source.line_offset + 1))
            .collect::<Vec<_>>()
            .join("\n")
    )?;

    let diag_render_options = mu::base::DiagRenderOptions {
        color: false,
        ..Default::default()
    };

    let mut source = mu::document::Source::from_source(env.source.clone());

    use microcad_driver::commands::Compile;

    let model = source.compile(mu::CompileParameters {
        resolve: mu::ResolveParameters {
            search_paths: vec!["../crates/std/lib".into(), "../assets".into()],
        },
    });
    let diag = source.diags();
    let error_lines = diag.error_lines();
    let warning_lines = diag.warning_lines();

    let result = match env.mode() {
        // test is expected to fail?
        TestMode::Fail => {
            if diag.has_errors() {
                writeln!(log, "-- Errors --")?;
                writeln!(log, "{}", source.diagnostics_string(&diag_render_options))?;

                if env.has_error_markers()
                    && let Some(msg) =
                        env.report_wrong_errors(&error_lines, &mu::HashSet::default())
                {
                    writeln!(log, "{msg}")?;
                    TestResult::FailWrong
                } else {
                    TestResult::FailOk
                }
            } else {
                TestResult::OkFail
            }
        }

        TestMode::Todo => TestResult::Todo,
        TestMode::Warn => {
            if diag.has_warnings() {
                writeln!(log, "-- Warnings --")?;
                writeln!(log, "{}", source.diagnostics_string(&diag_render_options))?;

                if env.has_error_markers()
                    && let Some(msg) = env.report_wrong_errors(&error_lines, &warning_lines)
                {
                    writeln!(log, "{msg}")?;
                    TestResult::OkWarn
                } else {
                    TestResult::OkWarn
                }
            } else if diag.has_errors() {
                writeln!(log, "-- Errors --")?;
                writeln!(log, "{}", source.diagnostics_string(&diag_render_options))?;

                TestResult::Fail
            } else {
                TestResult::OkFail
            }
        }
        // test is expected to succeed?
        TestMode::Ok => {
            if diag.has_errors() {
                writeln!(log, "-- Errors --")?;
                writeln!(log, "{}", source.diagnostics_string(&diag_render_options))?;

                TestResult::Fail
            } else {
                TestResult::Ok
            }
        }
        TestMode::Ignore => {
            return Ok(());
        }
    };

    if let Ok(model) = model {
        report_model(&env, log, source, model)?;
    }

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

fn report_model(
    env: &TestEnv,
    log: &mut dyn std::io::Write,
    mut source: mu::document::Source,
    model: mu::Model,
) -> std::io::Result<()> {
    if model.has_no_output() {
        return writeln!(log, "-- No Model --");
    }

    use mu::OutputType::*;
    use mu::export::{stl::StlExporter, svg::SvgExporter};

    let model = match source.render(RenderParameters {
        resolution: if env.hires() {
            mu::RenderResolution::high()
        } else {
            mu::RenderResolution::medium()
        },
        cache: None,
        progress_tx: None,
    }) {
        Ok(model) => model,
        Err(err) => {
            return writeln!(log, "Render error: {err}");
        }
    };
    writeln!(log, "-- Model --\n{}", mu::base::FormatTree(&model))?;

    let export = match model.deduce_output_type() {
        Geometry2D => Some(mu::ExportCommand {
            filename: env.out_file("svg"),
            exporter: Rc::new(SvgExporter),
        }),
        Geometry3D => Some(mu::ExportCommand {
            filename: env.out_file("stl"),
            exporter: Rc::new(StlExporter),
        }),
        NotDetermined => {
            writeln!(log, "Could not determine output type.")?;
            None
        }
        _ => panic!("Invalid geometry output"),
    };

    match export {
        Some(export) => match export.export(&model) {
            Ok(_) => writeln!(log, "Export of {:?} successful.", export.filename),
            Err(err) => writeln!(log, "Export error: {err}"),
        },
        None => writeln!(log, "Nothing will be exported."),
    }
}
