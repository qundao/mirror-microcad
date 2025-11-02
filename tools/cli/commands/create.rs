// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI create command

use crate::*;

use std::io::Write;

#[derive(clap::Parser)]
pub struct Create {
    path: std::path::PathBuf,
}

impl RunCommand for Create {
    fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let path = cli.path_with_default_ext(&self.path);

        if path.exists() {
            eprintln!("Error: File {path:?} already exists.")
        } else {
            // create demo program
            let mut f = std::fs::File::create(path.clone())?;
            f.write_all(include_bytes!("../hello.µcad"))?;
            eprintln!("File {path:?} generated.")
        }

        Ok(())
    }
}
