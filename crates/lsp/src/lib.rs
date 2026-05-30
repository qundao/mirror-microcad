// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad lsp library

pub mod backend;
pub mod processor;

pub use config::Config;

mod config;
mod semantic_tokens;
mod to_lsp;

use backend::Backend;

pub use tower_lsp as lsp;

pub use lsp::Server;

/// µcad lsp service
pub fn build_lsp_service(config: Config) -> (lsp::LspService<Backend>, lsp::ClientSocket) {
    log::info!("Starting LSP server");

    let processor = processor::ProcessorController::run();

    lsp::LspService::build(|client| Backend::new(client, processor, config))
        .custom_method("custom/activeFileChanged", Backend::on_active_file_changed)
        .finish()
}
