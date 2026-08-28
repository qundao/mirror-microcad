// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model methods and trait implementations for rendering.

mod attribute;
mod cache;
mod context;
mod output;
mod render;
mod tree;

use microcad_hash::HashMap;

pub use attribute::*;
pub use cache::*;
pub use context::*;
pub use tree::*;

use microcad_core::{Geometry, Geometry2D, Scalar};
use microcad_lang_types::model::ModelType;
pub use output::*;
pub use render::{RenderPrimitive, RenderResolution};

use miette::Diagnostic;
use thiserror::Error;

/// An error that occurred during rendering.
#[derive(Debug, Error, Diagnostic)]
pub enum RenderError {
    /// Invalid output type.
    #[error("Invalid output type: {0}")]
    InvalidOutputType(ModelType),

    #[error("Builtin error")]
    BuiltinError(#[from] BuiltinError),

    /// Nothing to render.
    #[error("Nothing to render")]
    NothingToRender,
}

use microcad_builtin::{BuiltinConstruct, BuiltinError, BuiltinId, mu};

/// Built-in execution function signature
pub type RenderFn = fn(&mut RenderContext) -> Result<GeometryOutput, RenderError>;

#[derive(Debug, Default)]
pub struct RenderHooks {
    hooks: HashMap<BuiltinId, RenderFn>,
}

impl RenderHooks {
    pub fn new() -> Self {
        let mut hooks = RenderHooks::default();
        hooks.insert::<mu::geo2d::Circle>();
        hooks
    }

    pub fn insert<C: BuiltinConstruct + Render>(&mut self) {
        self.hooks.insert(C::ITEM.id(), |ctx| {
            C::from_model(ctx.model().get())?.render(ctx)
        });
    }
}

impl RenderPrimitive for mu::geo2d::Circle {
    fn render_primitive(&self, resolution: &RenderResolution) -> Geometry {
        let radius: Scalar = self.radius.to_num();
        let n = resolution.circular_segments(radius);
        Geometry2D::Polygon(microcad_core::Circle::circle_polygon(radius, n)).into()
    }
}

impl<T: RenderPrimitive> Render for T {
    fn render(&self, context: &mut RenderContext) -> RenderResult<GeometryOutput> {
        context.update(|context, _| Ok(self.render_primitive(&context.current_resolution()).into()))
    }
}

impl Render for mu::ops::Difference {
    fn render(&self, context: &mut RenderContext) -> RenderResult<GeometryOutput> {
        todo!()
        /*context.update(|context, node| {
            let outputs = Vec::new();
            node.into_group().children().try_for_each(|node| {
                outputs.push(node.render_with_context(context)?);
                Ok(())
            });

            Ok(outputs.difference())
        })*/
    }
}

impl Render for mu::ops::Extrude {
    fn render(&self, context: &mut RenderContext) -> RenderResult<GeometryOutput> {
        todo!()
        /*context.update(|context, node| {
            let outputs = Vec::new();
            node.children().try_for_each(|node| {
                outputs.push(node.render_with_context(context)?);
                Ok(())
            });

            Ok(node.union().extrude(self.height.to_num()))
        })*/
    }
}

/// A result from rendering a model.
pub type RenderResult<T> = Result<T, RenderError>;

/// The render trait.
pub trait Render<T = GeometryOutput> {
    /// Render method.
    fn render(&self, context: &mut RenderContext) -> RenderResult<T>;
}

/*
impl Element {
    /// Fetch the local matrix
    pub fn get_affine_transform(&self) -> RenderResult<Option<AffineTransform>> {
        match &self {
            Element::BuiltinWorkpiece(builtin_workpiece) => match builtin_workpiece.kind {
                BuiltinWorkbenchKind::Transform => match builtin_workpiece.call()? {
                    BuiltinWorkpieceOutput::Transform(affine_transform) => {
                        Ok(Some(affine_transform))
                    }
                    _ => unreachable!(),
                },
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}

impl Model {
    /// Pre-render the model.
    ///
    /// Pre-rendering create as render output and calculates the matrices, resolutions and hashes of a model.
    pub fn prerender(&self, resolution: RenderResolution) -> RenderResult<usize> {
        pub fn create_render_output(model: &Model) -> RenderResult<()> {
            let output = RenderOutput::new(model)?;
            {
                let mut model_ = model.borrow_mut();
                model_.output = Some(output);
            };

            model
                .borrow()
                .children
                .iter()
                .try_for_each(create_render_output)
        }

        pub fn set_world_matrix(model: &Model, matrix: Mat4) -> RenderResult<()> {
            let world_matrix = {
                let mut model_ = model.borrow_mut();
                let output = model_.output.as_mut().expect("Output");
                let world_matrix = matrix * output.local_matrix().unwrap_or(Mat4::identity());
                output.set_world_matrix(world_matrix);
                world_matrix
            };

            model
                .borrow()
                .children
                .iter()
                .try_for_each(|model| set_world_matrix(model, world_matrix))
        }

        /// Set the resolution for this model.
        pub fn set_resolution(model: &Model, resolution: RenderResolution) {
            let resolution = match model.borrow().attributes().get_resolution() {
                Some(resolution_attribute) => RenderResolution {
                    linear: match resolution_attribute {
                        ResolutionAttribute::Absolute(linear) => linear,
                        ResolutionAttribute::Relative(factor) =>
                        // Example: A relative resolution of 200% scales an absolution resolution from 0.1mm to 0.5mm.
                        {
                            resolution.linear / factor
                        }
                    },
                },
                None => resolution,
            };

            let new_resolution = {
                let mut model_ = model.borrow_mut();
                let output = model_.output.as_mut().expect("Output");
                let resolution = resolution * output.local_matrix().unwrap_or(Mat4::identity());
                output.set_resolution(resolution.clone());
                resolution
            };

            model.borrow().children.iter().for_each(|model| {
                set_resolution(model, new_resolution.clone());
            });
        }

        // Create specific render output with local matrix.
        create_render_output(self)?;

        // Calculate the world matrix.
        set_world_matrix(self, Mat4::identity())?;

        // Calculate the resolution for the model.
        set_resolution(self, resolution);

        log::trace!("Finished prerender:\n{}", FormatTree(self));

        Ok(self
            .descendants()
            .filter(|model| !model.has_no_output())
            .count())
    }
}

/// This implementation renders a [`Geometry2D`] out of a [`Model`].
///
/// Notes:
/// * The impl attaches the output geometry to the model's render output.
/// * It is assumed the model has been pre-rendered.
impl RenderWithContext<GeometryOutput> for Model {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.with_model(self.clone(), |context| {
            let model = context.model();
            let geometry: GeometryOutput = {
                let model_ = model.borrow();
                        match model_.element() {
                            // A group geometry will render the child geometry
                            Element::BuiltinWorkpiece(builtin_workpiece) => {
                                Ok(builtin_workpiece.render_with_context(context)?)
                            }
                            _ => Ok(model_.children.render_with_context(context)?),
                        }
                }
            }?;

            self.borrow_mut()
                .output_mut()
                .set_geometry(GeometryOutput::Geometry2D(geometry.clone()));
            Ok(geometry)
        })
    }
}


impl RenderWithContext<Geometries2D> for Models {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometries2D> {
        let mut geometries = Vec::new();
        for model in self.iter() {
            let geo: Geometry2DOutput = model.render_with_context(context)?;
            geometries.push(Rc::new(geo.inner.clone()));
        }
        Ok(geometries.into_iter().collect())
    }
}

impl RenderWithContext<Geometry2DOutput> for Models {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        match self.len() {
            0 => Err(RenderError::NothingToRender),
            1 => self.first().expect("One item").render_with_context(context),
            _ => Ok(Rc::new(
                Geometry2D::Collection(self.render_with_context(context)?).into(),
            )),
        }
    }
}

impl RenderWithContext<Geometries3D> for Models {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometries3D> {
        let mut geometries = Vec::new();
        for model in self.iter() {
            let geo: Geometry3DOutput = model.render_with_context(context)?;
            geometries.push(Rc::new(geo.inner.clone()));
        }
        Ok(geometries.into_iter().collect())
    }
}

impl RenderWithContext<Geometry3DOutput> for Models {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        match self.len() {
            0 => Err(RenderError::NothingToRender),
            1 => self.first().expect("One item").render_with_context(context),
            _ => Ok(Rc::new(
                Geometry3D::Collection(self.render_with_context(context)?).into(),
            )),
        }
    }
}
*/
