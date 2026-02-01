// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI resolve command.

use microcad_lang::{diag::*, resolve::*};

use crate::*;

#[derive(clap::Parser)]
pub struct Resolve {
    #[clap(flatten)]
    pub parse: Parse,

    /// Print resolve context.
    #[arg(long)]
    pub resolve: bool,

    /// Do not load default standard library.
    #[arg(long)]
    no_std: bool,

    /// Add path to search for additional libraries (may be used multiple times).
    #[arg(short = 'L', action = clap::ArgAction::Append)]
    pub lib_path: Vec<std::path::PathBuf>,
}

impl RunCommand<ResolveContext> for Resolve {
    fn run(&self, cli: &Cli) -> miette::Result<ResolveContext> {
        // run prior parse step
        let root = self.parse.run(cli)?;

        // add default paths or omit this step by option
        let mut search_paths = self.lib_path.clone();

        // Add default search paths path.
        if !self.no_std {
            search_paths.push(microcad_std::global_std_path());
        }

        // search for a usable std library
        if self.no_std {
            eprintln!("Info: omitting standard library (--no-std).");
        } else if !search_paths.iter().any(microcad_std::is_installed) {
            eprintln!("Warning: No std library was found in given search paths: {search_paths:?}.");
            if let Err(err) = microcad_std::install(microcad_std::global_std_path(), false) {
                return Err(miette::miette!("Could not install standard library: {err}"));
            }
        }

        let start = std::time::Instant::now();

        // resolve the file
        let context = ResolveContext::create(
            root,
            &search_paths,
            Some(microcad_builtin::builtin_module()),
            DiagHandler::default(),
        )?;

        if cli.time {
            eprintln!("Resolving Time : {}", Cli::time_to_string(&start.elapsed()));
        }

        if self.resolve {
            print!("{context}");
        }

        if cli.is_resolve() {
            if context.has_errors() {
                eprint!("{}", context.diagnosis());
            }
            eprintln!("Resolved successfully!");
        }

        Ok(context)
    }
}
