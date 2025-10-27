// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer Load WKT.

use bevy::render::mesh::Mesh;
use wkt::TryFromWkt;

use microcad_core::Geometry2D;
use microcad_core::MultiPolygon;

pub fn load_wkt(path: &std::path::Path) -> anyhow::Result<Mesh> {
    let geometry = {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        Geometry2D::MultiPolygon(
            MultiPolygon::try_from_wkt_reader(reader)
                .map_err(|e| e.to_string())
                .expect("No error"),
        )
    };

    Ok(crate::processor::from_geometry2d(&geometry, 0.0))
}
