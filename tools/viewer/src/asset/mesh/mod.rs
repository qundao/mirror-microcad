use bevy::render::mesh::Mesh;

use crate::processor::triangle_mesh_to_bevy_with_smoothness;

mod stl_loader;

pub fn load_stl(path: &std::path::Path) -> anyhow::Result<Mesh> {
    // Suppose you already have your TriangleMesh loaded or created:
    // For example, create a simple triangle mesh for demo here

    let tri_mesh = {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        stl_loader::load_ascii_stl(reader).expect("No error")
    };
    println!("Mesh with {} vertices", tri_mesh.positions.len());

    Ok(triangle_mesh_to_bevy_with_smoothness(&tri_mesh, 20.0))
}
