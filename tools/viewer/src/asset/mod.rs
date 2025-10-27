use bevy::render::mesh::Mesh;

use crate::asset::{mesh::load_stl, polygon::load_wkt};

pub use microcad_core::Scalar;
pub use microcad_core::Vec3;
pub mod mesh;
pub mod polygon;

/// Render an asset.
pub fn render_to_mesh(path: &std::path::Path) -> anyhow::Result<Mesh> {
    let ext = path.extension().unwrap().to_str().unwrap().to_lowercase();
    match ext.as_str() {
        "stl" => load_stl(path),
        "wkt" => load_wkt(path),
        _ => unimplemented!("Unknown format"),
    }
}
