// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Built-in 3D geometries.

mod sphere;

pub use sphere::*;

use microcad_lang::builtin::*;

/// geo3d Builtin module
pub fn geo3d() -> Symbol {
    crate::ModuleBuilder::new("geo3d")
        .builtin::<Sphere>()
        .build()
}
