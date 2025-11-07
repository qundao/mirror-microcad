// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// A parametric **involute spur gear** profile.
///
/// This type defines the geometric parameters for generating a 2D involute gear shape.
///
/// # See also
/// - [`involute_gear_tooth`](#method.involute_gear_tooth): Generates a single gear tooth.
///
/// # References
/// - [Involute Gear Geometry (Wikipedia)](https://en.wikipedia.org/wiki/Involute_gear)
#[derive(BuiltinPrimitive2D)]
pub struct InvoluteGearProfile {
    /// Gear module (mm per tooth), controlling the overall gear size.
    /// Defines the ratio between the pitch diameter and the number of teeth:
    /// `pitch_diameter = module * teeth`.
    pub module: Scalar,
    /// Total number of teeth on the gear. Must be a positive integer.
    pub teeth: Integer,
    /// Pressure angle (radians or degrees), defining the shape of the involute flank.
    /// The standard pressure angle of the gear, typically
    /// `20°` or `25°`, which determines the shape of the involute profile and
    /// the base circle radius.
    pub pressure_angle: Angle,
}

use geo::{Coord, coord};

impl InvoluteGearProfile {
    fn involute(base_radius: Scalar, involute_angle: Scalar) -> Coord<Scalar> {
        // involute_angle in radians
        coord! {
            x: base_radius * (involute_angle.cos() + involute_angle * involute_angle.sin()),
            y: base_radius * (involute_angle.sin() - involute_angle * involute_angle.cos())
        }
    }

    fn involute_intersect_angle(base_radius: Scalar, radius: Scalar) -> Scalar {
        ((radius / base_radius).powi(2) - 1.0).sqrt()
    }

    fn rotate_point(angle: Scalar, c: &Coord<Scalar>) -> Coord<Scalar> {
        coord! {
            x:  c.x * angle.cos() - c.y * angle.sin(),
            y:  c.x * angle.sin() + c.y * angle.cos()
        }
    }

    fn mirror_point(c: &Coord<Scalar>) -> Coord<Scalar> {
        coord! { x: c.x, y: -c.y }
    }

    #[inline]
    fn pitch_radius(&self) -> Scalar {
        self.module * self.teeth as Scalar / 2.0
    }

    #[inline]
    fn base_radius(&self) -> Scalar {
        self.pitch_radius() * self.pressure_angle.0.cos()
    }

    #[inline]
    fn outer_radius(&self) -> Scalar {
        self.pitch_radius() + self.module
    }

    #[inline]
    fn root_radius(&self) -> Scalar {
        self.pitch_radius() - 1.25 * self.module
    }

    #[inline]
    fn half_thick_angle(&self) -> Scalar {
        // Half tooth thickness angle at pitch circle
        let z = self.teeth as Scalar;
        let phi = self.pressure_angle.0; // in radians
        let pitch_half = consts::PI / (2.0 * z);

        // Correction for involute starting at base circle
        pitch_half + phi.tan() - phi
    }

    /// Create the line string for a single gear tooth.
    pub fn involute_gear_tooth(&self, facets: usize) -> LineString {
        let r = self.base_radius();
        let involute = |angle| Self::involute(r, angle);
        let intersect_angle = |angle| Self::involute_intersect_angle(r, angle);
        let facets = facets.max(8);

        let pitch_point = involute(intersect_angle(self.pitch_radius()));
        let pitch_angle = pitch_point.y.atan2(pitch_point.x);
        let theta = pitch_angle + self.half_thick_angle(); // tooth center

        let start_angle = intersect_angle(self.root_radius().max(r));
        let stop_angle = intersect_angle(self.outer_radius());
        let d = (stop_angle - start_angle) / facets as Scalar;

        let mut left_points = Vec::with_capacity(facets * 2);
        let mut right_points = Vec::with_capacity(facets * 2);

        for i in 0..facets {
            let p = (
                Self::rotate_point(theta, &involute(start_angle + d * i as Scalar)),
                Self::rotate_point(theta, &involute(start_angle + d * (i + 1) as Scalar)),
            );

            right_points.extend([Self::mirror_point(&p.0), Self::mirror_point(&p.1)]);
            left_points.extend([p.0, p.1]);
        }

        LineString::new(right_points.into_iter().rev().chain(left_points).collect())
    }
}

impl Render<Geometry2D> for InvoluteGearProfile {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        let tooth = self.involute_gear_tooth(
            (resolution.circular_segments(self.outer_radius() / self.teeth as f64)) as usize,
        );

        let inv = 2.0 * consts::PI / self.teeth as Scalar;

        Polygon::new(
            LineString::new(
                (0..self.teeth)
                    .flat_map(|i| {
                        let angle = i as Scalar * inv;
                        tooth.coords().map(move |p| Self::rotate_point(angle, p))
                    })
                    .collect(),
            ),
            vec![],
        )
        .into()
    }
}

impl RenderWithContext<Geometry2DOutput> for InvoluteGearProfile {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
