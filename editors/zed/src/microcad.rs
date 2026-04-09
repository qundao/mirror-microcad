use zed_extension_api::{self as zed, LanguageServerId, Result};

const UCAD_BINARY_NAME: &str = "microcad-lsp";

struct MicrocadExtension;

impl zed::Extension for MicrocadExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let shell_env = worktree.shell_env();

        let command = worktree
            .which(UCAD_BINARY_NAME)
            .ok_or_else(|| format!("Could not find {} binary", UCAD_BINARY_NAME))?;

        Ok(zed::Command {
            command,
            args: vec![],
            env: shell_env,
        })
    }
}

zed::register_extension!(MicrocadExtension);
