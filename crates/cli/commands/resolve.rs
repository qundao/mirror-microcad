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
    #[clap(long)]
    pub resolve: bool,

    /// Do not complain about missing standard library.
    #[clap(long)]
    no_std: bool,

    /// Paths to search for files.
    ///
    /// By default, `./std/lib` (if it exists) and `~/.microcad/lib` are used.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append)]
    pub search_paths: Vec<std::path::PathBuf>,

    /// Do not use default search paths if it is not defined explicitly  with --search-paths.
    #[arg(short, long)]
    omit_default_libs: bool,
}

impl RunCommand<ResolveContext> for Resolve {
    fn run(&self, cli: &Cli) -> miette::Result<ResolveContext> {
        // run prior parse step
        let root = self.parse.run(cli)?;

        // add default paths or omit this step by option
        let mut search_paths = self.search_paths.clone();

        if !self.omit_default_libs {
            search_paths.append(&mut microcad_builtin::dirs::default_search_paths())
        };

        // search for a usable std library
        if self.no_std {
            eprintln!("Info: omitting standard library.");
        } else if !search_paths.iter().any(|dir| {
            let file_path = dir.join("std/mod.µcad");
            file_path.exists() && file_path.is_file()
        }) {
            eprintln!(
                "Warning: No std library was found in given search paths: {:?}.
Use `microcad install std` to install the std library.",
                search_paths
            );
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
