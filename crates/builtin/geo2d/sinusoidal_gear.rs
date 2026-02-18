// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// A parametric **sinusoidal spur gear** profile.
#[derive(BuiltinPrimitive2D)]
pub struct SinusoidalGearProfile {
    /// Module (amplitude), teeth size.
    pub module: Length,
    /// Number of teeth.
    pub teeth: Integer,
    /// Tooth roundness parameter (0.0..1.0).
    pub roundness: Scalar, // b = 0.0..1.0
    /// Stretch factor (0.0..1.0)
    pub stretch: Scalar,
}

impl Render<Geometry2D> for SinusoidalGearProfile {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        let a = *self.module * 2.0;
        let b = self.roundness;
        let radius = a * self.teeth as Scalar * 0.5;
        let n = resolution.circular_segments(radius) * 2;

        Polygon::new(
            LineString::new(
                (0..n)
                    .map(|i| {
                        let theta = (i as Scalar) / (n as Scalar) * consts::PI * 2.0;
                        let phi_sin = (theta * (self.teeth as Scalar)).sin();
                        let l =
                            a * (phi_sin.abs().powf(b) * phi_sin.signum()) * self.stretch + radius;
                        geo::coord! {
                            x:  l * theta.cos(),
                            y:  l * theta.sin(),
                        }
                    })
                    .collect(),
            ),
            vec![],
        )
        .into()
    }
}

impl RenderWithContext<Geometry2DOutput> for SinusoidalGearProfile {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
