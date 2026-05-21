//! Test the LSP processor

use tower_lsp::{LanguageServer, lsp_types as lsp};

use microcad_driver::prelude as mu;

use microcad_lsp as mu_lsp;

use googletest::prelude::*;

/// check `lsp::InitializeParams` -> `lsp::InitializeResult`
#[tokio::test]
async fn initialize() -> Result<()> {
    let config = mu::Config::default();

    log::info!("Starting LSP server");

    let processor = mu_lsp::processor::ProcessorInterface::run();

    let (service, _) = tower_lsp::LspService::build(|client| {
        mu_lsp::backend::Backend::new(client, processor, config.search_paths)
    })
    .custom_method(
        "custom/activeFileChanged",
        mu_lsp::backend::Backend::on_active_file_changed,
    )
    .finish();

    let params = lsp::InitializeParams::default();

    let backend = service.inner();

    let init_result = backend.initialize(params).await;

    assert_that!(
        init_result,
        ok(matches_pattern!(lsp::InitializeResult {
            capabilities: matches_pattern!(lsp::ServerCapabilities {
                document_formatting_provider: some(anything())
            }),
        }))
    );

    Ok(())
}
