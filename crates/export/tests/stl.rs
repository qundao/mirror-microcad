// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL test

use microcad_builtin::{BuiltinEvalContext, mu};
use microcad_core::Rect;
use microcad_export::{
    Writer,
    stl::{AsciiStlWriter, WriteAsciiStl},
};
use microcad_lang_types::{Value, arguments};
use microcad_render::RenderContext;

use crate::common::{circle, rect};
mod common;

/// Renders STL content using a canvas closure, performs snapshot testing via `insta`,
/// and writes the output STL to disk under `target/svg_outputs/<snapshot_name>.stl`.
pub fn assert_stl_snapshot<F>(snapshot_name: &str, draw: F) -> std::io::Result<()>
where
    F: FnOnce(&mut AsciiStlWriter<Vec<u8>>) -> std::io::Result<()>,
{
    let mut writer = AsciiStlWriter::new(vec![]);
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

/// Rect(0mm,0mm,10mm,10mm).extrude(10mm);
#[test]
fn test_cube() -> std::io::Result<()> {
    let mut ctx = BuiltinEvalContext::default();

    assert_stl_snapshot("test_cube", |writer| {
        let rect = rect(Rect::new((0., 0.), (10., 10.0)));
        let extrude = mu::ops::extrude(arguments!(self = rect, height = Value::mm(10.0)), &mut ctx)
            .expect("No error");
        let geometry = RenderContext::new(&extrude)
            .render()
            .expect("No render errors");

        geometry.write_ascii_stl(writer)?;
        Ok(())
    })
}
