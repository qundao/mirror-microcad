// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Common exporter test functions

use std::io::Write;

use microcad_core::Rect;
use microcad_export::svg::SvgWriter;

/*
pub fn get_build_target_dir() -> std::io::Result<std::path::PathBuf> {
    // CARGO_MANIFEST_DIR points to the directory containing the active Cargo.toml
    let manifest_dir = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()),
    );

    // Walk up until we find the root `target` folder or workspace `Cargo.toml`
    let mut root = manifest_dir.clone();
    while let Some(parent) = root.parent() {
        if parent.join("Cargo.toml").exists() && parent.join("target").exists() {
            root = parent.to_path_buf();
            break;
        }
        root = parent.to_path_buf();
    }

    let output_dir = root.join("target").join("svg_outputs");
    std::fs::create_dir_all(&output_dir)?;
    Ok(output_dir)
}
*/

/// Renders SVG content using a canvas closure, performs snapshot testing via `insta`,
/// and writes the output SVG to disk under `target/svg_outputs/<snapshot_name>.svg`.
pub fn assert_svg_snapshot<F>(
    snapshot_name: &str,
    mut writer: SvgWriter<Vec<u8>>,
    draw: F,
) -> std::io::Result<()>
where
    F: FnOnce(&mut SvgWriter<Vec<u8>>) -> std::io::Result<()>,
{
    draw(&mut writer)?;

    let buffer = writer.finish()?;

    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);

    Ok(insta::with_settings!(
        {
            prepend_module_to_snapshot => false,
            snapshot_path => "../snapshots",
        },
        {
            insta::assert_binary_snapshot!(format!("{snapshot_name}.svg").as_str(), buffer);
        }
    ))
}
