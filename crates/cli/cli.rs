// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI.

use clap::Parser;

use crate::commands::*;
use crate::config::Config;

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Display processing time.
    #[arg(short = 'T', long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub(crate) time: bool,

    /// Load config from file.
    #[arg(short = 'C', long = "config")]
    config_path: Option<std::path::PathBuf>,

    /// Verbosity level (use -v, -vv, or -vvv)
    #[arg(short, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,

    /// Subcommands.
    #[command(subcommand)]
    command: Commands,

    /// The loaded or default CLI config.
    #[clap(skip)]
    pub config: Config,
}

impl Cli {
    /// Create a new CLI.
    pub fn new() -> miette::Result<Self> {
        let mut cli = Self::parse();
        if let Some(config_path) = &cli.config_path {
            cli.config = Config::load(config_path)?
        }
        Ok(cli)
    }

    /// Return a path with default µcad extension given in the config.
    pub fn path_with_default_ext(&self, path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        let mut path = path.as_ref().to_path_buf();
        if path.extension().is_none() {
            path.set_extension(self.config.default_extension.clone());
        }
        path
    }

    /// Run the CLI.
    pub fn run(&self) -> miette::Result<()> {
        let start = std::time::Instant::now();

        match &self.command {
            Commands::Parse(parse) => {
                parse.run(self)?;
            }
            Commands::Resolve(resolve) => {
                resolve.run(self)?;
            }
            Commands::Eval(eval) => {
                eval.run(self)?;
            }
            Commands::Export(export) => {
                export.run(self)?;
            }
            Commands::Create(create) => {
                create.run(self)?;
            }
            Commands::Watch(watch) => {
                watch.run(self)?;
            }
            Commands::Install(install) => {
                install.run(self)?;
            }
            Commands::Completions(completions) => {
                completions.run(self)?;
            }
        }

        if self.time {
            eprintln!(
                "Overall Time   : {}",
                Self::time_to_string(&start.elapsed())
            );
        }
        Ok(())
    }

    pub(super) fn is_parse(&self) -> bool {
        matches!(self.command, Commands::Parse(..))
    }

    pub(super) fn is_resolve(&self) -> bool {
        matches!(self.command, Commands::Resolve(..))
    }

    pub(super) fn is_eval(&self) -> bool {
        matches!(self.command, Commands::Eval(..))
    }

    pub(super) fn is_export(&self) -> bool {
        matches!(self.command, Commands::Export(..))
    }

    pub(super) fn time_to_string(duration: &std::time::Duration) -> String {
        use num_format::{Locale, ToFormattedString};
        format!(
            "{:>8}µs",
            duration.as_micros().to_formatted_string(&Locale::en)
        )
    }
}
