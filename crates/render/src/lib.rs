// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model methods and trait implementations for rendering.

mod attribute;
mod cache;
mod context;
mod display;
mod output;
mod render;
mod tree;

use microcad_hash::HashMap;

pub use attribute::*;
pub use cache::*;
pub use context::*;
pub use tree::*;

use microcad_core::{Geometries2D, Geometry, Geometry2D, Scalar};
use microcad_lang_types::{
    ModelNodeRef, ModelTree,
    model::{self, ModelType, NodeExt},
};
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

/// Builder methods.
impl RenderHooks {
    pub fn new() -> Self {
        let mut hooks = RenderHooks::default();
        hooks.insert::<mu::geo2d::Circle>();
        hooks.insert::<mu::ops::Difference>();
        hooks
    }
}

impl RenderHooks {
    pub fn insert<C: BuiltinConstruct + Render>(&mut self) {
        self.hooks.insert(C::ITEM.id(), |ctx| {
            C::from_model(ctx.model().get())?.render(ctx)
        });
    }

    pub fn get(&self, id: BuiltinId) -> Option<RenderFn> {
        self.hooks.get(&id).cloned()
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
        context.update(|_, context| Ok(self.render_primitive(&context.current_resolution()).into()))
    }
}

impl Render for mu::ops::Difference {
    fn render(&self, context: &mut RenderContext) -> RenderResult<GeometryOutput> {
        context.update(|node, context| {
            let mut nodes = Vec::new();
            for child in node
                .first_child(&context.tree.arena)
                .unwrap()
                .children(&context.tree.arena)
            {
                nodes.push(child);
            }

            let mut outputs = Vec::new();

            for node in nodes {
                let output = context.tree.arena.get(node).unwrap();
                let model = context
                    .model_tree
                    .arena
                    .get(output.get().model_node_id)
                    .unwrap()
                    .get();

                context.stack.push(node);
                match model
                    .builtin_id()
                    .and_then(|builtin_id| context.hooks.get(builtin_id))
                {
                    Some(hook) => outputs.push(hook(context)?),
                    None => {
                        todo!()
                    }
                }
                context.stack.pop();
            }

            let geometries = Geometries2D::new(
                outputs
                    .into_iter()
                    .filter_map(|output| match output.geometry {
                        Geometry::Geometry2D(geo2d) => Some(geo2d),
                        Geometry::Geometry3D(_geo3d) => todo!(),
                    })
                    .collect(),
            );

            Ok(GeometryOutput::from(Geometry::from(Geometry2D::from(
                geometries.boolean_op(microcad_core::BooleanOp::Subtract),
            ))))
        })
    }
}

impl Render for mu::ops::Extrude {
    fn render(&self, _context: &mut RenderContext) -> RenderResult<GeometryOutput> {
        todo!()
        /*context.update(|context, node| {
            let outputs = Vec::new();
            node.children().try_for_each(|node| {
                outputs.push(node.render(context)?);
                Ok(())
            });

            Ok(node.union().extrude(self.height.to_num()))
        })*/
    }
}

/// A result from rendering a model.
pub type RenderResult<T = GeometryOutput> = Result<T, RenderError>;

/// The render trait.
pub trait Render<T = GeometryOutput> {
    /// Render method.
    fn render(&self, context: &mut RenderContext) -> RenderResult<T>;
}

impl Render<Option<GeometryOutput>> for RenderOutput {
    fn render(&self, context: &mut RenderContext) -> RenderResult<Option<GeometryOutput>> {
        let model = context.model();

        match model
            .builtin_id()
            .and_then(|builtin_id| context.hooks.get(builtin_id))
        {
            Some(hook) => Ok(Some(hook(context)?)),
            None => Ok(None),
        }
    }
}

impl Render<GeometryTree> for ModelTree {
    fn render(&self, context: &mut RenderContext) -> RenderResult<GeometryTree> {
        let mut tree = GeometryTree::new(&self, context.current_resolution());

        context.stack.push(tree.root);

        while let Some(node_id) = context.stack.last().cloned() {
            // 1. Compute geometry for the current node
            let render_output = tree.arena[node_id].get_mut();

            render_output.geometry = render_output.render(context)?;

            context.stack.pop();

            // 2. Push children onto the stack in reverse order for left-to-right DFS traversal
            let children: Vec<_> = node_id.children(&tree.arena).collect();
            for child_id in children.into_iter().rev() {
                context.stack.push(child_id);
            }
        }

        Ok(tree)
    }
}
