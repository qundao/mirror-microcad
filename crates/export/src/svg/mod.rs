// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

mod attributes;
mod canvas;
pub mod exporter;
mod primitives;
pub mod writer;

#[cfg(test)]
mod tests;

use std::io::Write;

pub use attributes::SvgTagAttributes;
pub use canvas::*;
pub use exporter::*;
pub use primitives::*;
pub use writer::*;

/// Trait to write something into an SVG.
pub trait WriteSvg {
    /// Write SVG tags directly to the provided writer.
    fn write_svg<W: Write>(
        &self,
        writer: &mut SvgWriter<W>,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()>;

    /// Convenience helper to write with default attributes.
    fn write_svg_default<W: std::io::Write>(
        &self,
        writer: &mut SvgWriter<W>,
    ) -> std::io::Result<()> {
        self.write_svg(writer, &SvgTagAttributes::default())
    }
}

/// Trait to write something into an SVG while mapping it to canvas coordinates.
pub trait WriteSvgMapped: WriteSvg + MapToCanvas {
    /// Map coordinates using the writer's canvas and emit SVG tags.
    fn write_svg_mapped<W: Write>(
        &self,
        writer: &mut SvgWriter<W>,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()>;

    /// Convenience helper to map and write using default attributes.
    fn write_svg_mapped_default<W: Write>(&self, writer: &mut SvgWriter<W>) -> std::io::Result<()> {
        self.write_svg_mapped(writer, &SvgTagAttributes::default())
    }
}

// Blanket implementation for any type that can be mapped and rendered as SVG
impl<T> WriteSvgMapped for T
where
    T: WriteSvg + MapToCanvas,
{
    fn write_svg_mapped<W: std::io::Write>(
        &self,
        writer: &mut SvgWriter<W>,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        let mapped = self.map_to_canvas(writer.canvas());
        mapped.write_svg(writer, attr)
    }
}
