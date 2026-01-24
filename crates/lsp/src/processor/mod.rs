// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language server processor.
//!
//! The processor sets up the µcad language toolchain (parse, resolve, eval).
//! It runs in a separate thread and communication is handled via
//! crossbeam channels with requests and responses.

use std::path::PathBuf;

use crossbeam::channel::{Receiver, Sender};
use miette::IntoDiagnostic;
use microcad_lang::{
    diag::{self, PushDiag},
    eval,
    src_ref::{self, SrcReferrer},
    syntax,
};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, FullDocumentDiagnosticReport, Url};

/// A processor request.
///
/// Commands that can be passed to the [`Processor`].
#[derive(Clone)]
#[allow(unused)]
pub enum ProcessorRequest {
    SetCursorPosition { url: Url, line: u32, col: u32 },
    AddDocument(Url),
    RemoveDocument(Url),
    UpdateDocument(Url),
    UpdateDocumentStr(Url, String),
    GetDocumentDiagnostics(Url),
}

/// A processor response.
pub enum ProcessorResponse {
    /// Error messages and warnings for a specific document received.
    DocumentDiagnostics(Url, FullDocumentDiagnosticReport),
}

fn src_ref_to_lsp_range(src_ref: src_ref::SrcRef) -> Option<tower_lsp::lsp_types::Range> {
    match src_ref.0 {
        Some(src_ref_inner) => {
            use tower_lsp::lsp_types::{Position, Range};

            let start = Position::new(
                src_ref_inner.at.line as u32 - 1,
                src_ref_inner.at.col as u32 - 1,
            );
            let end = Position::new(
                src_ref_inner.at.line as u32 - 1,
                (src_ref_inner.at.col + src_ref_inner.range.len()) as u32 - 1,
            );

            Some(Range::new(start, end))
        }
        None => None,
    }
}

pub struct WorkspaceSettings {
    pub search_paths: Vec<PathBuf>,
}

#[derive(Default)]
enum Context {
    #[default]
    None,
    Parse(Box<diag::DiagHandler>),
    Eval(Box<eval::EvalContext>),
}

impl Context {
    fn diag(&self) -> Option<&diag::DiagHandler> {
        match self {
            Context::None => None,
            Context::Parse(diag_handler) => Some(diag_handler),
            Context::Eval(eval_context) => Some(&eval_context.diag),
        }
    }
}

/// The processor  responsible for generating view commands.
///
/// The processor itself runs in a separate thread and can be controlled
/// via [`ProcessorInterface`] by sending requests and handling the corresponding responses.
pub struct Processor {
    workspace_settings: WorkspaceSettings,
    context: Context,
    pub request_handler: Receiver<ProcessorRequest>,
    /// Outputs
    pub response_sender: Sender<ProcessorResponse>,
}

pub type ProcessorResult = miette::Result<Vec<ProcessorResponse>>;

impl Processor {
    /// Handle processor request.
    pub fn handle_request(&mut self, request: ProcessorRequest) -> ProcessorResult {
        match request {
            ProcessorRequest::SetCursorPosition { .. } => todo!(),
            ProcessorRequest::AddDocument(url) => self.add_document(&url),
            ProcessorRequest::RemoveDocument(_) => Ok(vec![]),
            ProcessorRequest::UpdateDocument(url) => self.update_document(&url),
            ProcessorRequest::UpdateDocumentStr(url, doc) => self.update_document_str(&url, &doc),
            ProcessorRequest::GetDocumentDiagnostics(url) => self.get_document_diagnostics(&url),
        }
    }

    /// Process a µcad file (parse, resolve, eval).
    pub fn add_document(&mut self, url: &Url) -> ProcessorResult {
        self.update_document(url)
    }

    /// Update (re-evaluate) a document.
    pub fn update_document(&mut self, url: &Url) -> ProcessorResult {
        self.context = match syntax::SourceFile::load(
            url.to_file_path()
                .map_err(|_| miette::miette!("Error converting {url} to file path."))?,
        ) {
            Ok(source_file) => match eval::EvalContext::from_source(
                source_file,
                Some(microcad_builtin::builtin_module()),
                &self.workspace_settings.search_paths,
                eval::Capture::new(),
                microcad_builtin::builtin_exporters(),
                microcad_builtin::builtin_importers(),
                0,
            ) {
                Ok(eval) => Context::Eval(eval.into()),
                Err(_) => todo!(),
            },

            Err(err) => {
                let mut diag = diag::DiagHandler::default();
                let src_ref = err.src_ref();
                diag.push_diag(diag::Diagnostic::Error(src_ref::Refer::new(
                    err.into(),
                    src_ref,
                )))?;
                Context::Parse(diag.into())
            }
        };

        if let Context::Eval(context) = &mut self.context {
            context.eval()?;
        }

        Ok(vec![])
    }

    pub fn update_document_str(&mut self, url: &Url, doc: &str) -> ProcessorResult {
        let path = url
            .to_file_path()
            .map_err(|_| miette::miette!("Error converting {url} to file path."))?;
        self.context = match syntax::SourceFile::load_from_str(None, path, doc) {
            Ok(source_file) => match eval::EvalContext::from_source(
                source_file,
                Some(microcad_builtin::builtin_module()),
                &self.workspace_settings.search_paths,
                eval::Capture::new(),
                microcad_builtin::builtin_exporters(),
                microcad_builtin::builtin_importers(),
                0,
            ) {
                Ok(mut context) => {
                    context.eval()?;
                    Context::Eval(context.into())
                }
                Err(_) => todo!(),
            },

            Err(errors) => {
                let mut diag = diag::DiagHandler::default();
                for err in errors {
                    let src_ref = err.src_ref();
                    diag.push_diag(diag::Diagnostic::Error(src_ref::Refer::new(
                        err.into(),
                        src_ref,
                    )))?;
                }
                Context::Parse(diag.into())
            }
        };
        Ok(vec![])
    }

    pub fn get_document_diagnostics(&self, url: &Url) -> ProcessorResult {
        if let Some(diag) = &self.context.diag() {
            Ok(vec![ProcessorResponse::DocumentDiagnostics(
                url.clone(),
                FullDocumentDiagnosticReport {
                    result_id: None,
                    items: diag
                        .diag_list
                        .iter()
                        .filter_map(|diag| {
                            let message = diag.message();
                            match src_ref_to_lsp_range(diag.src_ref()) {
                                Some(range) => {
                                    let severity = match diag.level() {
                                        microcad_lang::diag::Level::Trace => {
                                            DiagnosticSeverity::HINT
                                        }
                                        microcad_lang::diag::Level::Info => {
                                            DiagnosticSeverity::INFORMATION
                                        }
                                        microcad_lang::diag::Level::Warning => {
                                            DiagnosticSeverity::WARNING
                                        }
                                        microcad_lang::diag::Level::Error => {
                                            DiagnosticSeverity::ERROR
                                        }
                                    };

                                    Some(Diagnostic::new(
                                        range,
                                        Some(severity),
                                        None,
                                        None,
                                        message,
                                        None,
                                        None,
                                    ))
                                }
                                None => None,
                            }
                        })
                        .collect(),
                },
            )])
        } else {
            Ok(vec![])
        }
    }
}

#[derive(Debug)]
pub struct ProcessorInterface {
    pub request_sender: Sender<ProcessorRequest>,
    pub response_receiver: Receiver<ProcessorResponse>,
}

impl ProcessorInterface {
    /// Send request.
    pub fn send_request(&self, request: ProcessorRequest) -> miette::Result<()> {
        self.request_sender.send(request).into_diagnostic()
    }

    pub fn recv_response(&self) -> miette::Result<ProcessorResponse> {
        self.response_receiver.recv().into_diagnostic()
    }

    /// Run the processing thread and create interface.
    pub fn run(workspace_settings: WorkspaceSettings) -> Self {
        let (request_sender, request_receiver) = crossbeam::channel::unbounded();
        let (response_sender, response_receiver) = crossbeam::channel::unbounded();

        std::thread::spawn(move || {
            let mut processor = Processor {
                workspace_settings,
                request_handler: request_receiver,
                response_sender,
                context: Context::None,
            };

            loop {
                if let Ok(request) = processor.request_handler.recv()
                    && let Ok(responses) = processor.handle_request(request)
                {
                    for response in responses {
                        processor.response_sender.send(response).expect("No error");
                    }
                }
            }
        });

        Self {
            request_sender,
            response_receiver,
        }
    }
}
