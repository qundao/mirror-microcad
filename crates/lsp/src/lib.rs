// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad lsp library

pub mod backend;
pub mod processor;

mod config;
pub use config::Config;

use backend::Backend;

pub use tower_lsp::{ClientSocket, LspService, Server};

/// µcad lsp service
pub fn lsp_service(config: Config) -> (LspService<Backend>, ClientSocket) {
    log::info!("Starting LSP server");

    let processor = processor::ProcessorInterface::run();

    tower_lsp::LspService::build(|client| Backend::new(client, processor, config))
        .custom_method("custom/activeFileChanged", Backend::on_active_file_changed)
        .finish()
}
