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
    #[clap(long, default_value_t = false)]
    no_std: bool,

    /// Paths to search for files.
    ///
    /// By default, `./std/lib` (if it exists) and the microcad user config folder are used.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append)]
    pub search_paths: Vec<std::path::PathBuf>,

    /// Do not use default search paths if it is not defined explicitly with --search-paths.
    #[arg(short, long, default_value_t = false)]
    omit_default_paths: bool,
}

impl RunCommand<ResolveContext> for Resolve {
    fn run(&self, cli: &Cli) -> miette::Result<ResolveContext> {
        // run prior parse step
        let root = self.parse.run(cli)?;

        // add default paths or omit this step by option
        let mut search_paths = self.search_paths.clone();

        // Add default search paths path.
        if !self.omit_default_paths {
            search_paths.append(&mut microcad_builtin::dirs::default_search_paths());

            if search_paths.is_empty() {
                search_paths.push(microcad_std::get_user_stdlib_path());
            }
        }

        // search for a usable std library
        if self.no_std {
            println!("Info: omitting standard library.");
        } else if !search_paths
            .iter()
            .any(|search_path| microcad_std::is_installed(search_path))
        {
            eprintln!("Warning: No std library was found in given search paths: {search_paths:?}.");
            if let Some(first_search_path) = search_paths.first() {
                if let Err(err) = microcad_std::install(first_search_path, false) {
                    return Err(miette::miette!("Could not install standard library: {err}"));
                }
            } else {
                return Err(miette::miette!("No search paths given!"));
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
