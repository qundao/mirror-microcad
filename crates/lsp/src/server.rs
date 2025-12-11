// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language server.

mod processor;

use microcad_viewer_ipc::{ViewerProcessInterface, ViewerRequest};
use tower_lsp::{
    Client, LanguageServer, LspService, Server, async_trait,
    jsonrpc::Result,
    lsp_types::{
        DiagnosticOptions, DiagnosticServerCapabilities, DidChangeTextDocumentParams,
        DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
        DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportPartialResult,
        DocumentDiagnosticReportResult, ExecuteCommandParams, InitializeParams, InitializeResult,
        InitializedParams, MessageType, RelatedFullDocumentDiagnosticReport, ServerCapabilities,
        TextDocumentSyncCapability, TextDocumentSyncKind, Url,
    },
};

#[derive(Debug)]
struct Backend {
    client: Client,
    processor: processor::ProcessorInterface,
    viewer: ViewerProcessInterface,
}

impl Backend {
    fn send_lsp(&self, req: ProcessorRequest) {
        if let Err(err) = self.processor.send_request(req) {
            log::error!("Cannot send request to lsp processor: {err}")
        }
    }
    fn send_viewer(&self, req: ViewerRequest) {
        if let Err(err) = self.viewer.send_request(req) {
            log::error!("Cannot send request to viewer: {err}")
        }
    }

    async fn on_active_file_changed(&self, params: serde_json::Value) {
        log::trace!("on_active_file_changed: {params:?}");
        if let Ok(Some(uri)) = read_uri("uri", &params) {
            self.send_lsp(ProcessorRequest::UpdateDocument(uri.clone()));
            match uri.to_file_path() {
                Ok(path) => {
                    log::info!("New active document: {:?}", path);
                    self.send_viewer(ViewerRequest::ShowSourceCodeFromFile { path });
                }
                Err(()) => log::error!("Cannot parse URI: {uri}"),
            }
        } else {
            log::error!("No 'uri' field in notification parameters");
        }
    }
}

#[async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        log::info!("initialize");
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions::default(),
                )),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        log::info!("initialized");
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        log::info!("shutdown");
        self.send_viewer(ViewerRequest::Exit);
        Ok(())
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        match uri.to_file_path() {
            Ok(path) => {
                log::info!("Did change {path:?}");
                if let Some(last) = params.content_changes.last() {
                    self.send_lsp(ProcessorRequest::UpdateDocumentStr(uri, last.text.clone()));
                }
            }
            Err(()) => log::error!("Cannot parse URI: {uri}"),
        }
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        match uri.to_file_path() {
            Ok(path) => {
                log::info!("Did open: {path:?}");
                self.send_lsp(ProcessorRequest::AddDocument(uri))
            }
            Err(_) => log::error!("Cannot parse URI: {uri}"),
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        match uri.to_file_path() {
            Ok(path) => {
                log::info!("Did save: {path:?}");
                self.send_lsp(ProcessorRequest::UpdateDocument(uri.clone()));
                self.send_viewer(ViewerRequest::ShowSourceCodeFromFile { path });
            }
            Err(_) => log::error!("Cannot parse URI: {uri}"),
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        match uri.to_file_path() {
            Ok(path) => {
                log::info!("Did close: {path:?}");
                self.send_lsp(ProcessorRequest::RemoveDocument(uri))
            }
            Err(_) => log::error!("Cannot parse URI: {uri}"),
        }
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        self.send_lsp(ProcessorRequest::GetDocumentDiagnostics(
            params.text_document.uri,
        ));

        // Wait for response
        if let Ok(ProcessorResponse::DocumentDiagnostics(_url, diag)) =
            self.processor.recv_response()
        {
            log::info!("Diagnostics received!");
            Ok(DocumentDiagnosticReportResult::Report(
                DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                    related_documents: None, // TODO: Get related documents.
                    full_document_diagnostic_report: diag,
                }),
            ))
        } else {
            Ok(DocumentDiagnosticReportResult::Partial(
                DocumentDiagnosticReportPartialResult {
                    related_documents: None,
                },
            ))
        }
    }

    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> Result<Option<serde_json::Value>> {
        log::trace!("execute_command: {params:?}");
        match params.command.as_str() {
            "microcad.showPreview" => {
                if let Some(arg) = params.arguments.first() {
                    let uri = match uri_obj_to_lsp_url(match arg.get("uri") {
                        Some(v) => v,
                        None => {
                            log::error!("Missing 'uri' field");
                            return Ok(Some(serde_json::json!({
                                "error": "Missing 'uri' field"
                            })));
                        }
                    }) {
                        Ok(uri) => uri,
                        Err(err) => {
                            return Ok(Some(serde_json::json! ({"error": format!("{err}")})));
                        }
                    };

                    log::info!("ShowPreview received for {uri}");
                    if let Err(err) =
                        self.viewer
                            .send_request(ViewerRequest::ShowSourceCodeFromFile {
                                path: uri.path().into(),
                            })
                    {
                        log::error!("{err}");
                        return Ok(Some(serde_json::json! ({"error": format!("{err}")})));
                    }

                    self.client
                        .log_message(MessageType::INFO, format!("Preview generated for {uri}"))
                        .await;

                    return Ok(Some(serde_json::json!({ "ok": true })));
                }
            }
            "microcad.minimizePreview" => {
                log::info!("MinimizePreview received");
                if let Err(err) = self.viewer.send_request(ViewerRequest::Minimize) {
                    log::error!("Could not send request ViewerRequest::Minimize: {err}");
                    return Ok(Some(serde_json::json!({
                        "error": "Cannot minimize viewer: {err}"
                    })));
                }
                self.client
                    .log_message(MessageType::INFO, "Preview hidden")
                    .await;
                return Ok(Some(serde_json::json!({ "ok": true })));
            }
            _ => log::info!("Unknown command '{}'", params.command),
        }

        Ok(None)
    }
}

use clap::Parser;

use crate::processor::{ProcessorRequest, ProcessorResponse, WorkspaceSettings};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// log into given file
    #[arg(short, long, value_name = "FILE")]
    log_file: Option<std::path::PathBuf>,

    #[arg(long)]
    stdio: bool,

    /// Paths to search for files.
    ///
    /// By default, `./lib` (if it exists) and `~/.microcad/lib` are used.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append)]
    pub search_paths: Vec<std::path::PathBuf>,
}

impl Args {
    /// Returns microcad's config dir, even if it does not exist.
    ///
    /// On Linux, the config dir is located in `~/.config/microcad`.
    pub fn config_dir() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|dir| dir.join("microcad"))
    }

    /// Returns global root dir, even if it does not exist.
    ///
    /// On Linux, the root dir is located in `~/.config/microcad/lib`.
    pub fn global_root_dir() -> Option<std::path::PathBuf> {
        Self::config_dir().map(|dir| dir.join("lib"))
    }

    /// `./lib` (if exists) and `~/.config/microcad/lib` (if exists).
    pub fn default_search_paths() -> Vec<std::path::PathBuf> {
        let local_dir = std::path::PathBuf::from("./lib");
        let mut search_paths = Vec::new();

        if let Some(global_root_dir) = Self::global_root_dir() {
            if global_root_dir.exists() {
                search_paths.push(global_root_dir);
            }
        }
        if local_dir.exists() {
            search_paths.push(local_dir);
        }

        search_paths
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Some(log_file) = args.log_file {
        let file = std::fs::File::create(log_file).expect("could not create log file");
        let target = Box::new(file);
        env_logger::Builder::new()
            .target(env_logger::Target::Pipe(target))
            .filter(None, log::LevelFilter::Info)
            .init();
    } else {
        env_logger::init()
    }
    /*    // construct a subscriber that prints formatted traces to stdout
        let subscriber = tracing_subscriber::FmtSubscriber::new();
        // use that subscriber to process traces emitted after this point
        tracing::subscriber::set_global_default(subscriber).expect("init log failed");
    */

    // add default paths if no search paths are given.
    let mut search_paths = args.search_paths.clone();

    if search_paths.is_empty() {
        search_paths.append(&mut Args::default_search_paths())
    };

    log::info!("Starting LSP server");

    let processor = processor::ProcessorInterface::run(WorkspaceSettings {
        search_paths: search_paths.clone(),
    });
    let viewer = ViewerProcessInterface::run(&search_paths, true);

    let (service, socket) = LspService::build(|client| Backend {
        client,
        processor,
        viewer,
    })
    .custom_method("custom/activeFileChanged", Backend::on_active_file_changed)
    .finish();
    log::info!("LSP service has been created");

    if args.stdio {
        use tokio::io::{stdin, stdout};
        Server::new(stdin(), stdout(), socket).serve(service).await;
    } else {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:5007")
            .await
            .expect("bind listener to 127.0.0.1:5007");
        log::info!("LSP server listening...");
        let (stream, _) = listener.accept().await.expect("accept socket");
        log::info!("Client has connected to LSP service");
        let (read, write) = tokio::io::split(stream);
        Server::new(read, write, socket).serve(service).await;
    };
}

fn uri_obj_to_lsp_url(uri_obj: &serde_json::Value) -> std::result::Result<Url, String> {
    if let Some(uri) = read_uri("external", uri_obj)? {
        return Ok(uri);
    }
    if let Some(fs_path) = read_uri("fsPath", uri_obj)? {
        return Ok(fs_path);
    }
    Err("Neither external nor fsPath found in uri object".to_string())
}

fn read_uri(value: &str, uri_obj: &serde_json::Value) -> std::result::Result<Option<Url>, String> {
    if let Some(external) = uri_obj.get(value).and_then(serde_json::Value::as_str) {
        return Url::parse(external)
            .map_err(|e| {
                log::info!("Failed to parse external URL: {e}");
                format!("Failed to parse external URL: {e}")
            })
            .map(Some);
    }

    Ok(None)
}
