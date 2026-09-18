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
    ModelTree,
    model::{ModelType, NodeExt},
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
pub type RenderFn = fn(&GeometryTree, &mut RenderContext) -> Result<GeometryOutput, RenderError>;

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
        self.hooks.insert(C::ITEM.id(), |tree, ctx| {
            C::from_model(ctx.model(tree).get())?.render(tree, ctx)
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
    fn render(
        &self,
        tree: &GeometryTree,
        context: &mut RenderContext,
    ) -> RenderResult<GeometryOutput> {
        context.update(|_, context| {
            Ok(self
                .render_primitive(&context.current_resolution(tree))
                .into())
        })
    }
}

impl Render for mu::ops::Difference {
    fn render(
        &self,
        tree: &GeometryTree,
        context: &mut RenderContext,
    ) -> RenderResult<GeometryOutput> {
        context.update(|_node, context| {
            let outputs = context.collect_outputs(tree);

            let geometries = Geometries2D::new(
                outputs
                    .into_iter()
                    .filter_map(|output| match &output.0.geometry {
                        Geometry::Geometry2D(geo2d) => Some(geo2d.clone()),
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
    fn render(
        &self,
        tree: &GeometryTree,
        _context: &mut RenderContext,
    ) -> RenderResult<GeometryOutput> {
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
    fn render(&self, tree: &GeometryTree, context: &mut RenderContext) -> RenderResult<T>;
}

impl Render<Vec<GeometryOutput>> for GeometryNodeData {
    fn render(
        &self,
        tree: &GeometryTree,
        context: &mut RenderContext,
    ) -> RenderResult<Vec<GeometryOutput>> {
        let model = context.model(tree);

        match model
            .builtin_id()
            .and_then(|builtin_id| context.hooks.get(builtin_id))
        {
            Some(hook) => Ok(vec![hook(tree, context)?]),
            None => Ok(context.collect_outputs(tree)),
        }
    }
}

pub fn render(model_tree: &ModelTree, context: &mut RenderContext) -> RenderResult<GeometryTree> {
    let mut tree = GeometryTree::new(
        model_tree,
        context.resolution.as_ref().cloned().unwrap_or_default(),
    );

    // Recursively process the tree starting from the root
    render_node_dfs(tree.root, &mut tree, model_tree, context)?;

    Ok(tree)
}

/// Recursive DFS helper: processes children first, then renders the current node.
fn render_node_dfs(
    node_id: GeometryNodeId,
    tree: &mut GeometryTree,
    model_tree: &ModelTree,
    context: &mut RenderContext,
) -> RenderResult<()> {
    // 1. Recurse down into all children first (Leaves are reached first)
    let children: Vec<_> = node_id.children(&tree.arena).collect();
    for child_id in children {
        render_node_dfs(child_id, tree, model_tree, context)?;
    }

    context.stack.push(node_id);
    // 2. Render current node after all children have completed
    let render_output = tree.arena[node_id].get();
    let outputs = render_output.render(tree, context)?;
    context.step();

    let render_output = tree.arena[node_id].get_mut();
    render_output.outputs = outputs;

    context.stack.pop();

    Ok(())
}
