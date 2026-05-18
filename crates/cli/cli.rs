// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI.

use std::str::FromStr;

use clap::Parser;

use crate::commands::*;
use microcad_driver::prelude as mu;

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
    pub config: std::rc::Rc<mu::Config>,
}

impl Cli {
    /// Create a new CLI.
    pub fn new() -> miette::Result<Self> {
        let mut cli = Self::parse();

        #[cfg(not(debug_assertions))]
        mu::install_std()?;

        if let Some(config_path) = &cli.config_path {
            cli.config = std::rc::Rc::new(mu::Config::load(config_path)?);
        }
        Ok(cli)
    }

    /// Generate compile parameters
    pub fn compile_parameters(
        &self,
        resolution: &str,
    ) -> miette::Result<microcad_driver::commands::CompileParameters> {
        use microcad_driver::commands::compile::*;
        Ok(CompileParameters {
            resolve: ResolveParameters {
                search_paths: self.config.search_paths.clone(),
            },
            render: RenderParameters::from_str(resolution)?.with_empty_cache(),
        })
    }

    /// Print diagnostics with colors and unicode.
    pub fn print_diagnostics(&self, diag: &impl mu::traits::PrintDiagnostics) {
        eprintln!(
            "{}",
            diag.diagnostics_string(&mu::PrintDiagnosticsParameters {
                color: true,
                unicode: true
            })
        );
    }

    /// Run the CLI.
    pub fn run(&self) -> miette::Result<()> {
        let start = std::time::Instant::now();

        match &self.command {
            Commands::Check(check) => {
                check.run(self)?;
            }
            Commands::Export(export) => {
                export.run(self)?;
            }
            Commands::Format(format) => {
                format.run(self)?;
            }
            Commands::Create(create) => {
                create.run(self)?;
            }
            Commands::Watch(watch) => {
                watch.run(self)?;
            }
            Commands::Completions(completions) => {
                completions.run(self)?;
            }
            Commands::Doc(doc) => {
                doc.run(self)?;
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

    pub(super) fn time_to_string(duration: &std::time::Duration) -> String {
        use num_format::{Locale, ToFormattedString};
        format!(
            "{:>8}µs",
            duration.as_micros().to_formatted_string(&Locale::en)
        )
    }
}
