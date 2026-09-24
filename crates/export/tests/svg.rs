// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! SVG tests
use cgmath::Vector2;
use geo::coord;
use microcad_core::{Circle, Rect};
use microcad_export::svg::{SvgTagAttributes, SvgWriter, WriteSvg};
use microcad_render::RenderContext;

pub fn get_svg_output_dir() -> std::io::Result<std::path::PathBuf> {
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

/// Renders SVG content using a canvas closure, performs snapshot testing via `insta`,
/// and writes the output SVG to disk under `target/svg_outputs/<snapshot_name>.svg`.
pub fn assert_svg_snapshot<F>(
    snapshot_name: &str,
    content_rect: Rect,
    draw: F,
) -> std::io::Result<()>
where
    F: FnOnce(&mut SvgWriter<Vec<u8>>) -> std::io::Result<()>,
{
    let buffer = Vec::new();
    let mut writer = SvgWriter::new_canvas(buffer, None, content_rect, None)?;

    draw(&mut writer)?;

    let buffer = writer.finish()?;
    let svg_string = String::from_utf8(buffer)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let file_path = get_svg_output_dir()?.join(format!("{}.svg", snapshot_name));
    std::fs::write(file_path, &svg_string)?;

    insta::assert_snapshot!(snapshot_name, svg_string);

    Ok(())
}

/// Render a polygon with holes
#[test]
fn svg_polygon() -> std::io::Result<()> {
    let bounds = Rect::new(coord! { x: 0., y: 0. }, coord! { x: 100., y: 100. });

    assert_svg_snapshot("svg_polygon", bounds, |writer| {
        let circle = Circle {
            radius: 10.0,
            offset: Vector2::new(25.0, 25.0),
        }
        .to_polygon(32);

        circle.write_svg_default(writer)?;
        Ok(())
    })
}

// Circle(42mm) - Circle(23mm);
/*#[test]
fn svg_difference() {
    let mut ctx = BuiltinEvalContext::default();

    // {
    //     Circle(42mm);
    //     Circle(23mm);
    // }
    let mut group = ModelTree::new(Element::Group);
    group.append(circle(42.0));
    group.append(circle(23.0));
    let diff = mu::ops::difference(arguments!(self = group), &mut ctx).expect("No error");

    let mut ctx = RenderContext::new(&diff);
    let geometry = render(&diff, &mut ctx).expect("No render errors");

    insta::assert_snapshot!("render_difference", geometry);
}
*/
