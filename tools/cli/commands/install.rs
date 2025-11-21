// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI install command.

use crate::*;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "lib/std"]
struct StdLib;

#[derive(clap::Parser)]
pub struct Install {
    /// Name of µcad library to install (currently only `std` is supported).
    pub library: String,

    /// Directory to install libraries into.
    /// If this command line option is not set, the library will be installed in ~/.microcad/lib/$LIBNAME
    pub root: Option<std::path::PathBuf>,

    /// Force overwrite.
    #[arg(short = 'f', long, default_value = "false", action = clap::ArgAction::SetTrue)]
    force: bool,
}

fn get_user_stdlib_path() -> std::path::PathBuf {
    let mut path = dirs::config_dir().expect("config directory");
    path.push("microcad");
    path.push("std");
    path
}

fn extract_stdlib(overwrite: bool) -> std::io::Result<()> {
    let dst = get_user_stdlib_path();
    if dst.exists() {
        if overwrite {
            println!("Overwriting existing µcad standard library in {:?}", dst);
        } else {
            println!(
                "Found µcad standard library already in {:?} (use -f to force overwrite)",
                dst
            );
            return Ok(());
        }
    }

    println!("Installing µcad standard library into {:?}...", dst);

    std::fs::create_dir_all(&dst)?;

    // Extrahiere alle eingebetteten Dateien
    StdLib::iter().try_for_each(|file| {
        let file_path = dst.join(file.as_ref());
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(
            file_path,
            StdLib::get(file.as_ref())
                .expect("embedded std not found")
                .data,
        )
    })?;

    println!("Successfully installed µcad standard library.");

    Ok(())
}

impl RunCommand for Install {
    fn run(&self, _cli: &Cli) -> anyhow::Result<()> {
        if self.library == "std" {
            Ok(extract_stdlib(self.force)?)
        } else {
            Err(anyhow::anyhow!(
                "Only `std` is supported as installable library at the moment.",
            ))
        }
    }
}
