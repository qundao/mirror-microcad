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
        let input = &self.input;
        let input = if input.is_dir() {
            &input.join("mod.µcad")
        } else {
            input
        };

        cli.path_with_default_ext(&input)
    }

    /// Return name of the parsed file
    pub fn input_name(&self) -> miette::Result<QualifiedName> {
        eprintln!("{:?}", std::env::current_dir().expect("no current dir"));

        let input = std::path::absolute(&self.input).expect("No error");

        let name = match input.file_stem().map(|s| s.to_string_lossy().to_string()) {
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
        };

        match name.as_str().try_into() {
            Ok(name) => Ok(name),
            Err(err) => Err(miette::miette!("Not a valid file stem")),
        }
    }
}

impl RunCommand<Rc<SourceFile>> for Parse {
    fn run(&self, cli: &Cli) -> miette::Result<Rc<SourceFile>> {
        let start = std::time::Instant::now();
        let name = self.input_name()?;

        let (source_file, error) = SourceFile::load_with_name(self.input_with_ext(cli), name);
        if let Some(error) = error {
            return Err(error.into());
        }

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
fn test_cli_parse() {
    let p = Parse {
        input: "foo.µcad".into(),
        syntax: false,
    };

    assert_eq!(
        p.input_name().expect("test error"),
        "foo".try_into().expect("test error")
    );

    let p = Parse {
        input: "../std/lib/std/mod.µcad".into(),
        syntax: false,
    };

    assert_eq!(
        p.input_name().expect("no input name"),
        QualifiedName::from_id("std".into())
    );
}
