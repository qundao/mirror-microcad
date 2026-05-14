// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::RenderResolution;
use microcad_driver::commands::compile::{RenderParameters, ResolveParameters};
use microcad_driver::commands::{CompileParameters, PrintDiagnostics};
use microcad_driver::*;

use microcad_lang_base::{DiagRenderOptions, FormatTree};
use microcad_test_tools::test_env::*;
use std::rc::Rc;

#[allow(dead_code)]
pub fn run_test(env: TestEnv) -> std::io::Result<()> {
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

    let diag_render_options = DiagRenderOptions {
        color: false,
        ..Default::default()
    };

    let mut source = document::Source::from_source(env.source.clone());

    use microcad_driver::commands::Compile;

    let resolution = if env.hires() {
        RenderResolution::high()
    } else {
        RenderResolution::medium()
    };

    let model = source.compile(CompileParameters {
        resolve: ResolveParameters {
            search_paths: vec!["../crates/std/lib".into(), "../assets".into()],
        },
        render: RenderParameters::from(resolution).with_empty_cache(),
    });
    let diag = source.diagnostics.borrow();
    let error_lines = diag.error_lines();
    let warning_lines = diag.warning_lines();

    let result = match env.mode() {
        // test is expected to fail?
        TestMode::Fail => {
            if diag.has_errors() {
                writeln!(log, "-- Errors --")?;
                writeln!(log, "{}", source.diagnostics_string(&diag_render_options))?;

                if env.has_error_markers()
                    && let Some(msg) = env.report_wrong_errors(&error_lines, &HashSet::default())
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
            } else {
                TestResult::OkFail
            }
        }
        // test is expected to succeed?
        TestMode::Ok => TestResult::Ok,
        TestMode::Ignore => {
            return Ok(());
        }
    };

    if let Ok(model) = model {
        report_model(&env, log, model)?;
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

fn report_model(env: &TestEnv, log: &mut dyn std::io::Write, model: Model) -> std::io::Result<()> {
    if model.has_no_output() {
        return writeln!(log, "-- No Model --");
    }

    use microcad_export::{stl::StlExporter, svg::SvgExporter};
    use microcad_lang::model::{ExportCommand as Export, OutputType};

    writeln!(log, "-- Model --\n{}", FormatTree(&model))?;

    let export = match model.deduce_output_type() {
        OutputType::Geometry2D => Some(Export {
            filename: env.out_file("svg"),
            exporter: Rc::new(SvgExporter),
        }),
        OutputType::Geometry3D => Some(Export {
            filename: env.out_file("stl"),
            exporter: Rc::new(StlExporter),
        }),
        OutputType::NotDetermined => {
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
