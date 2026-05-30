// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language server processor.
//!
//! The processor sets up the µcad language toolchain (parse, resolve, eval).
//! It runs in a separate thread and communication is handled via
//! crossbeam channels with requests and responses.

use microcad_driver::prelude as mu;
use mu::traits::*;

use crossbeam::channel::{Receiver, Sender};

use miette::IntoDiagnostic;

use mu::Url;
use tower_lsp::lsp_types as lsp;

use crate::to_lsp::ToLsp;

/// A processor request.
///
/// Commands that can be passed to the [`Processor`].
#[derive(Clone)]
#[allow(unused, missing_docs)]
pub enum ProcessorRequest {
    SetCursorPosition { url: Url, line: u32, col: u32 },
    AddDocument(Url),
    RemoveDocument(Url),
    UpdateDocument(Url),
    UpdateDocumentCode(Url, String),
    GetDocumentDiagnostics(Url),
    GetFullSemanticTokens(Url),
    FormatDocument(Url),
}

/// A processor response.
pub enum ProcessorResponse {
    /// Error messages and warnings for a specific document received.
    DocumentDiagnostics(Url, lsp::FullDocumentDiagnosticReport),
    /// Output semantic tokens.
    SemanticTokens(Url, lsp::SemanticTokensResult),
    /// Update the code of the document (e.g. after linting or formatting)
    UpdatedDocumentCode {
        /// Document URL
        url: Url,
        /// The new code of the document
        code: String,
    },
}

impl ProcessorResponse {
    fn diagnostics(url: Url, diag: &mu::Diagnostics) -> Self {
        Self::DocumentDiagnostics(url.clone(), diag.to_lsp())
    }
}

/// The processor  responsible for generating view commands.
///
/// The processor itself runs in a separate thread and can be controlled
/// via [`ProcessorInterface`] by sending requests and handling the corresponding responses.
pub struct Processor {
    /// Request handler.
    pub request_handler: Receiver<ProcessorRequest>,

    /// Response handler.
    pub response_sender: Sender<ProcessorResponse>,

    /// Processor documents.
    pub documents: mu::HashMap<Url, mu::document::Source>,
}

/// Type alias for a Result from a processor command.
pub type ProcessorResult = miette::Result<Vec<ProcessorResponse>>;

impl Processor {
    /// Handle processor request.
    pub fn handle_request(&mut self, request: ProcessorRequest) -> ProcessorResult {
        match request {
            ProcessorRequest::SetCursorPosition { .. } => todo!(),
            ProcessorRequest::AddDocument(url) => self.add_document(url),
            ProcessorRequest::RemoveDocument(url) => self.remove_document(&url),
            ProcessorRequest::UpdateDocument(url) => self.update_document(&url),
            ProcessorRequest::UpdateDocumentCode(url, doc) => self.update_document_code(&url, doc),
            ProcessorRequest::GetDocumentDiagnostics(url) => self.get_document_diagnostics(&url),
            ProcessorRequest::GetFullSemanticTokens(url) => self.get_full_semantic_tokens(&url),
            ProcessorRequest::FormatDocument(url) => self.format_document(&url),
        }
    }

    /// Process a µcad file (parse, resolve, eval).
    pub fn add_document(&mut self, url: Url) -> ProcessorResult {
        match mu::document::Source::load(url.clone()) {
            Ok(mut document) => {
                document.load_from_file()?;
                Self::compile_document(&mut document)?;
                self.documents.insert(url, document);
            }
            Err(_) => {
                log::error!("Could not load document: {url}")
            }
        }
        Ok(vec![])
    }

    /// Remove µcad file.
    pub fn remove_document(&mut self, url: &Url) -> ProcessorResult {
        self.documents.remove(url);
        Ok(vec![])
    }

    fn compile_document(document: &mut mu::document::Source) -> ProcessorResult {
        match document
            .parse()
            .and(document.lower())
            .and(document.resolve(mu::ResolveParameters::default()))
            .and(document.eval())
        {
            Ok(_) => Ok(vec![]),
            Err(_) => {
                log::error!("Error compiling document");
                Ok(vec![])
            }
        }
    }

    /// Update (re-evaluate) a document.
    pub fn update_document(&mut self, url: &Url) -> ProcessorResult {
        match self.documents.get_mut(url) {
            Some(document) => {
                document.load_from_file()?;
                Self::compile_document(document)
            }
            None => {
                log::error!("Document does not exist!");
                Ok(vec![])
            }
        }
    }

    pub fn update_document_code(&mut self, url: &Url, code: String) -> ProcessorResult {
        let document =
            self.documents
                .entry(url.clone())
                .or_insert(mu::document::Source::from_source(mu::base::Source {
                    url: url.clone(),
                    line_offset: 0,
                    code: mu::Hashed::new(code),
                }));

        Self::compile_document(document)
    }

    pub fn format_document(&mut self, url: &Url) -> ProcessorResult {
        match self.documents.get_mut(url) {
            Some(document) => match document
                .load_from_file()
                .and(Self::compile_document(document))
                .and(document.format(&mu::FormatParameters::default()))
            {
                Ok(formatted) => {
                    if formatted {
                        Self::compile_document(document)?;
                    }
                    Ok(vec![ProcessorResponse::UpdatedDocumentCode {
                        url: url.clone(),
                        code: document
                            .get_code()
                            .map(|s| s.to_string())
                            .unwrap_or_default(),
                    }])
                }
                Err(err) => {
                    log::error!("Error formatting document `{url}`: {err}");
                    Ok(vec![])
                }
            },
            None => {
                log::error!("Document does not exist!");
                Ok(vec![])
            }
        }
    }

    fn get_full_semantic_tokens(&self, url: &Url) -> ProcessorResult {
        match self
            .documents
            .get(url)
            .and_then(|doc| doc.ast_source.as_ref())
        {
            Some(ast) => {
                use crate::semantic_tokens::SemanticTokens;
                let mut ctx = crate::semantic_tokens::TokenContext::new(&ast);
                ast.ast.semantic_tokens(&mut ctx);

                Ok(vec![ProcessorResponse::SemanticTokens(
                    url.clone(),
                    lsp::SemanticTokensResult::Tokens(lsp::SemanticTokens {
                        result_id: None, // TODO: support delta updates
                        data: ctx.tokens().clone(),
                    }),
                )])
            }
            None => Ok(vec![]),
        }
    }

    fn get_document_diagnostics(&self, url: &Url) -> ProcessorResult {
        Ok(match self.documents.get(url) {
            Some(document) => vec![ProcessorResponse::diagnostics(
                url.clone(),
                document.diags(),
            )],
            None => vec![],
        })
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
    pub fn run() -> Self {
        let (request_sender, request_receiver) = crossbeam::channel::unbounded();
        let (response_sender, response_receiver) = crossbeam::channel::unbounded();

        std::thread::spawn(move || {
            let mut processor = Processor {
                request_handler: request_receiver,
                response_sender,
                documents: mu::HashMap::default(),
            };

            loop {
                if let Ok(request) = processor.request_handler.recv()
                    && let Ok(responses) = processor.handle_request(request)
                {
                    for response in responses {
                        processor.response_sender.send(response).ok();
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
