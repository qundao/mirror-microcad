// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinPrimitive3D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// The builtin sphere primitive, defined by its radius.
#[derive(BuiltinPrimitive3D)]
pub struct Sphere {
    /// Radius of the sphere in millimeters.
    pub radius: Length,
}

impl Render<Geometry3D> for Sphere {
    fn render(&self, resolution: &RenderResolution) -> Geometry3D {
        Manifold::sphere(*self.radius, resolution.circular_segments(*self.radius)).into()
    }
}

impl RenderWithContext<Geometry3DOutput> for Sphere {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
