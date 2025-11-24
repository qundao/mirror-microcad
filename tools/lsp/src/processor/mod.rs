// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language server processor.
//!
//! The processor sets up the µcad language toolchain (parse, resolve, eval).
//! It runs in a separate thread and communication is handled via
//! crossbeam channels with requests and responses.

use std::path::PathBuf;

use crossbeam::channel::{Receiver, Sender};
use microcad_lang::{
    eval::{Capture, EvalContext},
    src_ref::{SrcRef, SrcReferrer},
    syntax::SourceFile,
};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, FullDocumentDiagnosticReport, Url};

/// A processor request.
///
/// Commands that can be passed to the [`Processor`].
#[derive(Clone)]
pub enum ProcessorRequest {
    SetCursorPosition { url: Url, line: u32, col: u32 },
    AddDocument(Url),
    RemoveDocument(Url),
    UpdateDocument(Url),
    GetDocumentDiagnostics(Url),
}

/// A processor response.
pub enum ProcessorResponse {
    /// Error messages and warnings for a specific document received.
    DocumentDiagnostics(Url, FullDocumentDiagnosticReport),
}

fn src_ref_to_lsp_range(src_ref: SrcRef) -> Option<tower_lsp::lsp_types::Range> {
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

/// The processor  responsible for generating view commands.
///
/// The processor itself runs in a separate thread and can be controlled
/// via [`ProcessorInterface`] by sending requests and handling the corresponding responses.
pub struct Processor {
    workspace_settings: WorkspaceSettings,
    context: Option<EvalContext>,
    pub request_handler: Receiver<ProcessorRequest>,
    /// Outputs
    pub response_sender: Sender<ProcessorResponse>,
}

pub type ProcessorResult = anyhow::Result<Vec<ProcessorResponse>>;

impl Processor {
    /// Handle processor request.
    pub fn handle_request(&mut self, request: ProcessorRequest) -> ProcessorResult {
        match request {
            ProcessorRequest::SetCursorPosition { .. } => todo!(),
            ProcessorRequest::AddDocument(url) => self.add_document(&url),
            ProcessorRequest::RemoveDocument(_) => Ok(vec![]),
            ProcessorRequest::UpdateDocument(url) => self.update_document(&url),
            ProcessorRequest::GetDocumentDiagnostics(url) => self.get_document_diagnostics(&url),
        }
    }

    /// Process a µcad file (parse, resolve, eval).
    pub fn add_document(&mut self, url: &Url) -> ProcessorResult {
        self.update_document(url)
    }

    /// Update (re-evaluate) a document.
    pub fn update_document(&mut self, url: &Url) -> ProcessorResult {
        let source_file = SourceFile::load(
            url.to_file_path()
                .map_err(|_| anyhow::anyhow!("Error converting {url} to file path."))?,
        )?;

        self.context = EvalContext::from_source(
            source_file,
            Some(microcad_builtin::builtin_module()),
            &self.workspace_settings.search_paths,
            Capture::new(),
            microcad_builtin::builtin_exporters(),
            microcad_builtin::builtin_importers(),
            0,
        )
        .ok();

        if let Some(context) = &mut self.context {
            context.eval()?;
        }

        Ok(vec![])
    }

    pub fn get_document_diagnostics(&self, url: &Url) -> ProcessorResult {
        if let Some(context) = &self.context {
            log::info!("{:?}", context.diag.diag_list);
            Ok(vec![ProcessorResponse::DocumentDiagnostics(
                url.clone(),
                FullDocumentDiagnosticReport {
                    result_id: None,
                    items: context
                        .diag
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
    pub fn send_request(&self, request: ProcessorRequest) -> anyhow::Result<()> {
        Ok(self.request_sender.send(request)?)
    }

    pub fn recv_response(&self) -> anyhow::Result<ProcessorResponse> {
        Ok(self.response_receiver.recv()?)
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
                context: None,
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
