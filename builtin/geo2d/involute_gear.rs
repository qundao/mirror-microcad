// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macros::BuiltinPrimitive2D;
use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// A parametric involute gear.
#[derive(BuiltinPrimitive2D)]
pub struct InvoluteGearProfile {
    module: Scalar,
    teeth: Integer,
    pressure_angle: Angle,
}

use geo::{Coord, LineString, coord};

fn involute(base_radius: f64, involute_angle: f64) -> Coord<f64> {
    // involute_angle in radians
    coord! {
        x: base_radius * (involute_angle.cos() + involute_angle * involute_angle.sin()),
        y: base_radius * (involute_angle.sin() - involute_angle * involute_angle.cos())
    }
}

fn involute_intersect_angle(base_radius: f64, radius: f64) -> f64 {
    ((radius / base_radius).powi(2) - 1.0).sqrt()
}

fn rotate_point(angle: f64, c: &Coord<f64>) -> Coord<f64> {
    let (x, y) = (c.x, c.y);
    coord! {
        x:  x * angle.cos() - y * angle.sin(),
        y:  x * angle.sin() + y * angle.cos()
    }
}

fn mirror_point(c: &Coord<f64>) -> Coord<f64> {
    coord! { x: c.x, y: -c.y }
}

impl InvoluteGearProfile {
    #[inline]
    pub fn pitch_radius(&self) -> f64 {
        self.module * self.teeth as f64 / 2.0
    }

    #[inline]
    pub fn base_radius(&self) -> f64 {
        self.pitch_radius() * self.pressure_angle.0.cos()
    }

    #[inline]
    pub fn outer_radius(&self) -> f64 {
        self.pitch_radius() + self.module
    }

    #[inline]
    pub fn root_radius(&self) -> f64 {
        self.pitch_radius() - 1.25 * self.module
    }

    #[inline]
    pub fn half_thick_angle(&self) -> f64 {
        // Half tooth thickness angle at pitch circle
        let z = self.teeth as f64;
        let phi = self.pressure_angle.0; // in radians
        let pitch_half = std::f64::consts::PI / (2.0 * z);

        // Correction for involute starting at base circle
        pitch_half + (phi.tan() - phi)
    }

    pub fn involute_gear_tooth(&self, involute_facets: usize) -> LineString<f64> {
        let root_radius = self.root_radius();
        let base_radius = self.base_radius();
        let outer_radius = self.outer_radius();
        let half_thick_angle = self.half_thick_angle();

        let facets = if involute_facets > 0 {
            involute_facets
        } else {
            8
        };

        // Derived quantities
        let min_radius = root_radius.max(base_radius);

        let pitch_point = involute(
            base_radius,
            involute_intersect_angle(base_radius, self.pitch_radius()),
        );
        let pitch_angle = pitch_point.y.atan2(pitch_point.x);
        let centre_angle = pitch_angle + half_thick_angle; // tooth center

        let start_angle = involute_intersect_angle(base_radius, min_radius);
        let stop_angle = involute_intersect_angle(base_radius, outer_radius);

        let mut points: Vec<Coord<f64>> = Vec::with_capacity(facets * 2 + 1);
        let res = facets;

        let mut left_points = Vec::new();
        let mut right_points = Vec::new();

        // One side (right flank)
        for i in 1..=res {
            let t1 = start_angle + (stop_angle - start_angle) * ((i - 1) as f64 / res as f64);
            let t2 = start_angle + (stop_angle - start_angle) * (i as f64 / res as f64);

            let p1 = involute(base_radius, t1);
            let p2 = involute(base_radius, t2);

            let side1_p1 = rotate_point(centre_angle, &p1);
            let side1_p2 = rotate_point(centre_angle, &p2);
            let side2_p1 = mirror_point(&rotate_point(centre_angle, &p1));
            let side2_p2 = mirror_point(&rotate_point(centre_angle, &p2));

            left_points.push(side1_p1);
            left_points.push(side1_p2);

            right_points.push(side2_p1);
            right_points.push(side2_p2);
        }

        left_points.reverse();

        points.extend(left_points);
        points.extend(right_points);

        points.reverse();

        LineString::new(points)
    }
}

impl Render<Geometry2D> for InvoluteGearProfile {
    fn render(&self, resolution: &RenderResolution) -> Geometry2D {
        let tooth = self.involute_gear_tooth(
            (resolution.circular_segments(self.outer_radius()) / self.teeth.max(5) as u32) as usize,
        );
        //tooth.into()

        let z = self.teeth as usize;
        let mut points: Vec<Coord<f64>> = Vec::new();

        for i in 0..z {
            let angle = 2.0 * consts::PI * i as f64 / z as f64;
            for p in tooth.coords() {
                points.push(rotate_point(angle, p));
            }
        }

        // Close the polygon
        points.push(points[0]);

        Polygon::new(LineString::from(points), vec![]).into()
    }
}

impl RenderWithContext<Geometry2DOutput> for InvoluteGearProfile {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}
