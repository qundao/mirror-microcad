// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Test µcad code blocks inside mark down

use microcad_lang_base::Capture;

use crate::{CodeBlock, code_block::TestResult};

pub struct Test {
    /// Path of md file
    md_file: std::path::PathBuf,
    /// Code block
    code_block: CodeBlock,
}

impl Test {
    /// Generate a test from an md file and a code block, if the code block has a test result.
    pub fn new(md_file: impl AsRef<std::path::Path>, code_block: CodeBlock) -> Option<Self> {
        match &code_block.test_result() {
            Some(_) => Some(Self {
                md_file: md_file.as_ref().to_path_buf(),
                code_block,
            }),
            None => None,
        }
    }

    fn name(&self) -> &str {
        self.code_block.name()
    }

    fn result(&self) -> &TestResult {
        self.code_block
            .test_result()
            .as_ref()
            .expect("A test result")
    }

    pub fn md_file_path(&self) -> std::path::PathBuf {
        self.md_file.clone()
    }

    /// Return path where to store any test output.
    pub fn test_path(&self) -> std::path::PathBuf {
        self.md_file.parent().unwrap().join(".test")
    }

    /// Return test banner filename as string.
    pub fn banner(&self) -> String {
        self.banner_file()
            .to_string_lossy()
            .escape_default()
            .to_string()
    }

    /// Return test banner filename as path.
    pub fn banner_file(&self) -> std::path::PathBuf {
        self.test_path().join(format!("{}.svg", self.name()))
    }

    /// Return log file path.
    pub fn log_file(&self) -> std::path::PathBuf {
        self.test_path().join(format!("{}.log", self.name()))
    }

    /// Return output file path (without any file extension).
    pub fn out_file_path_stem(&self) -> std::path::PathBuf {
        self.test_path().join(format!("{}-out", self.name()))
    }

    /// Return output file path with given extension.
    pub fn out_file(&self, ext: &str) -> std::path::PathBuf {
        self.out_file_path_stem().with_extension(ext)
    }

    pub fn create_context(
        &self,
        source: &std::rc::Rc<microcad_lang::syntax::SourceFile>,
    ) -> microcad_lang::eval::EvalContext {
        let mut context = microcad_lang::eval::EvalContext::from_source(
            source.clone(),
            Some(microcad_builtin::builtin_module()),
            &["../crates/std/lib", "../assets"],
            Capture::new(),
            microcad_builtin::builtin_exporters(),
            microcad_builtin::builtin_importers(),
            self.code_block.line_offset(),
        )
        .expect("resolve error");
        context.diag.render_options.color = false;
        context
    }

    /// Generate rust code for test
    pub fn generate_test_rs(&self) -> String {
        format!(
            r##"
        #[test]
        #[allow(non_snake_case)]
        fn r#{name}() {{
            use microcad_lang_markdown::*;
            use microcad_lang_markdown::TestResult::*;
            Test::new({path:?}, {code_block:?}).run()
        }}"##,
            name = self.name(),
            path = self.md_file,
            code_block = self.code_block
        )
    }
    /*
    /// Run
    pub fn run(
        &self,
        path: impl AsRef<std::path::Path>,
        log: &mut String,
    ) -> Result<(), MarkdownError> {
        // load and handle µcad source file
        let source = SourceFile::load_from_str(Some(self.name()), path, &self.code)?;
        let mut context = self.create_context(&source);

        let eval = context.eval()?;

        fn report_model(env: &mut CodeBlock, model: Option<Model>) {
            use microcad_core::RenderResolution;
            use microcad_export::{stl::StlExporter, svg::SvgExporter};
            use microcad_lang::model::{ExportCommand as Export, OutputType};

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
                        Ok(_) => {
                            env.log_ln(&format!("Export of {:?} successful.", export.filename))
                        }
                        Err(error) => env.log_ln(&format!("Export error: {error}")),
                    },
                    None => env.log_ln("Nothing will be exported."),
                }
            } else {
                env.log_ln("-- No Model --");
            }
        }
    }*/
}

impl std::fmt::Debug for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "        Test name: {}", self.name())?;
        writeln!(f, "  Expected result: {}", self.result())?;
        writeln!(
            f,
            "      Source file: {}:{offset}",
            self.md_file_path().display(),
            offset = self.code_block.line_offset()
        )?;
        writeln!(f, "        Test path: {}", self.test_path().display())
    }
}
