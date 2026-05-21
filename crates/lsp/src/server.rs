// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language server.

use std::{path::PathBuf, sync::OnceLock};

use microcad_driver::prelude as mu;

use microcad_viewer_ipc::{ViewerProcessInterface, ViewerRequest};
use tower_lsp::{
    Client, LanguageServer, LspService, Server, async_trait,
    jsonrpc::Result,
    lsp_types::{
        DiagnosticOptions, DiagnosticServerCapabilities, DidChangeTextDocumentParams,
        DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
        DocumentDiagnosticParams, DocumentDiagnosticReport, DocumentDiagnosticReportPartialResult,
        DocumentDiagnosticReportResult, DocumentFormattingParams, ExecuteCommandParams,
        InitializeParams, InitializeResult, InitializedParams, MessageType, OneOf, Position, Range,
        RelatedFullDocumentDiagnosticReport, SemanticTokensFullOptions, SemanticTokensLegend,
        SemanticTokensOptions, SemanticTokensParams, SemanticTokensPartialResult,
        SemanticTokensResult, SemanticTokensServerCapabilities, ServerCapabilities,
        TextDocumentSyncCapability, TextDocumentSyncKind, TextEdit, Url,
    },
};

use clap::Parser;

use crate::processor::{
    ProcessorRequest, ProcessorResponse,
    semantic_tokens::{LEGEND_MODIFIERS, LEGEND_TYPES},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// log into given file
    #[arg(short, long, value_name = "FILE")]
    log_file: Option<std::path::PathBuf>,

    #[arg(long)]
    stdio: bool,
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
        env_logger::try_init_from_env("MICROCAD_LSP_LOG").ok();
    }

    /*
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("init log failed");
    */

    let config = mu::Config::default();

    log::info!("Starting LSP server");

    let processor = processor::ProcessorInterface::run();

    let (service, socket) =
        LspService::build(|client| backend::Backend::new(client, processor, config.search_paths))
            .custom_method(
                "custom/activeFileChanged",
                backend::Backend::on_active_file_changed,
            )
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
