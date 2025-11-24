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

/// Extract the standard library into the standard library path.
pub fn extract(overwrite: bool) -> std::io::Result<()> {
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

    // Extract all embedded files.
    Lib::iter().try_for_each(|file| {
        let file_path = dst.join(file.as_ref());
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
