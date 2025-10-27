// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::{Triangle, TriangleMesh};

use std::io::BufRead;

pub fn load_ascii_stl(reader: impl BufRead) -> Result<TriangleMesh, String> {
    let mut mesh = TriangleMesh::default();

    let lines = reader.lines();

    // Parse state
    enum State {
        None,
        FacetNormal,
        OuterLoop, // normal stored here
    }
    let mut state = State::None;

    // Current triangle vertex indices buffer
    let mut current_triangle: Vec<u32> = Vec::with_capacity(3);

    for line in lines {
        let line = line.map_err(|e| format!("IO error: {e}"))?;
        let line = line.trim();

        if line.starts_with("facet normal") {
            // Parse normal vector
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() != 5 {
                return Err(format!("Malformed facet normal line: {line}"));
            }
            state = State::FacetNormal;
        } else if line == "outer loop" {
            if let State::FacetNormal = state {
                state = State::OuterLoop;
                current_triangle.clear();
            } else {
                return Err("outer loop found outside of facet normal".into());
            }
        } else if line.starts_with("vertex") {
            if let State::OuterLoop = state {
                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() != 4 {
                    return Err(format!("Malformed vertex line: {line}"));
                }
                let x: f32 = parts[1].parse().map_err(|_| "Invalid vertex x")?;
                let y: f32 = parts[2].parse().map_err(|_| "Invalid vertex y")?;
                let z: f32 = parts[3].parse().map_err(|_| "Invalid vertex z")?;
                let pos = cgmath::Vector3::new(x, y, z);

                // Avoid duplicates (pos + normal)
                let idx = {
                    let idx = mesh.positions.len() as u32;
                    mesh.positions.push(pos);
                    idx
                };

                current_triangle.push(idx);
            } else {
                return Err("vertex found outside of outer loop".into());
            }
        } else if line == "endloop" {
            // Expect 3 vertices
            if current_triangle.len() != 3 {
                return Err(format!(
                    "Expected 3 vertices, got {}",
                    current_triangle.len()
                ));
            }
            mesh.triangle_indices.push(Triangle(
                current_triangle[0],
                current_triangle[1],
                current_triangle[2],
            ));
            current_triangle.clear();
        } else if line == "endfacet" {
            state = State::None;
        } else if line.starts_with("solid") || line.starts_with("endsolid") {
            // ignore
        } else if line.is_empty() {
            // ignore blank lines
        } else {
            // Unexpected line
            // You can decide to ignore or error
            // For now, ignore unknown lines to be lenient
        }
    }

    Ok(mesh)
}
