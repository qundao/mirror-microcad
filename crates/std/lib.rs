// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI install command.

use rust_embed::RustEmbed;

/// The µcad standard library asset.
#[derive(RustEmbed)]
#[folder = "lib"]
pub struct Lib;

pub fn get_user_stdlib_path() -> std::path::PathBuf {
    let mut path = dirs::config_dir().expect("config directory");
    path.push("microcad");
    path.push("lib");
    path
}

/// Check if there is a std library installed.
pub fn is_installed(search_path: impl AsRef<std::path::Path>) -> bool {
    let std = search_path.as_ref().join("std/mod.µcad");
    std.exists() && std.is_file()
}

/// Install the standard library into the standard library path.
pub fn install(search_path: impl AsRef<std::path::Path>, overwrite: bool) -> std::io::Result<()> {
    let path = search_path.as_ref();
    if path.exists() {
        if overwrite {
            println!("Overwriting existing µcad standard library in {path:?}");
        } else {
            println!("Found µcad standard library already in {path:?} (use -f to force overwrite)");
            return Ok(());
        }
    }

    println!("Installing µcad standard library into {:?}...", path);

    std::fs::create_dir_all(&path)?;

    // Extract all embedded files.
    Lib::iter().try_for_each(|file| {
        let file_path = path.join(file.as_ref());
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(
            file_path,
            Lib::get(file.as_ref())
                .expect("embedded folder 'lib' not found")
                .data,
        )
    })?;

    println!("Successfully installed µcad standard library.");

    Ok(())
}
