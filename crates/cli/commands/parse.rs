// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI parse command.

use crate::*;

use microcad_lang::{rc::*, syntax::*, tree_display::*};

#[derive(clap::Parser)]
pub struct Parse {
    /// Input µcad file.
    input: std::path::PathBuf,

    /// Print syntax tree.
    #[clap(long)]
    pub syntax: bool,
}

impl Parse {
    pub fn input_with_ext(&self, cli: &Cli) -> std::path::PathBuf {
        cli.path_with_default_ext(&self.input)
    }

    /// Return name of the parsed file
    pub fn input_name(&self) -> String {
        eprintln!("{:?}", std::env::current_dir().expect("no current dir"));

        let input = std::path::absolute(&self.input).expect("No error");

        match input.file_stem().map(|s| s.to_string_lossy().to_string()) {
            Some(file_stem) => {
                if &file_stem == "mod" {
                    input
                        .parent()
                        .expect("No error")
                        .file_name()
                        .expect("No parent folder name")
                        .to_string_lossy()
                        .to_string()
                } else {
                    file_stem
                }
            }
            None => unimplemented!("No file stem"),
        }
    }
}

impl RunCommand<Rc<SourceFile>> for Parse {
    fn run(&self, cli: &Cli) -> miette::Result<Rc<SourceFile>> {
        let start = std::time::Instant::now();
        let source_file = SourceFile::load(self.input_with_ext(cli))?;

        if cli.time {
            eprintln!("Parsing Time   : {}", Cli::time_to_string(&start.elapsed()));
        }

        if cli.is_parse() {
            eprintln!("Parsed successfully!");
        }

        if self.syntax {
            println!("{}", FormatTree(source_file.as_ref()));
        }
        Ok(source_file)
    }
}

#[test]
fn cli_test_parse() {
    let p = Parse {
        input: "foo.µcad".into(),
        syntax: false,
    };

    assert_eq!(p.input_name(), "foo");

    let p = Parse {
        input: "../std/lib/std/mod.µcad".into(),
        syntax: false,
    };

    assert_eq!(p.input_name(), "std");
}
