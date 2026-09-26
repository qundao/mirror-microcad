// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Common exporter test functions

use std::io::Write;

use microcad_core::Rect;
use microcad_export::{stl::AsciiStlWriter, svg::SvgWriter};



/// Renders STL content using a canvas closure, performs snapshot testing via `insta`,
/// and writes the output STL to disk under `target/svg_outputs/<snapshot_name>.stl`.
pub fn assert_stl_snapshot<F>(
    snapshot_name: &str,
    mut writer: AsciiStlWriter<Vec<u8>>,
    draw: F,
) -> std::io::Result<()>
where
    F: FnOnce(&mut AsciiStlWriter<Vec<u8>>) -> std::io::Result<()>,
{
    draw(&mut writer)?;

    let buffer = writer.finish()?;

    Ok(insta::with_settings!(
        {
            prepend_module_to_snapshot => false,
            snapshot_path => "../snapshots",
        },
        {
            insta::assert_binary_snapshot!(format!("{snapshot_name}.stl").as_str(), buffer);
        }
    ))
}
