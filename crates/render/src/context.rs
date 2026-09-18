// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render context

use std::sync::mpsc;

use microcad_hash::ToHash;
use microcad_lang_base::Shared;
use microcad_lang_types::{ModelNodeRef, ModelTree};

use crate::{
    GeometryNodeId, GeometryNodeRef, GeometryOutput, GeometryTree, RenderCache, RenderHooks,
    RenderResolution, RenderResult,
};

/// Our progress sender.
pub type ProgressTx = mpsc::Sender<f32>;

/// The render context.
///
/// Keeps a stack of model nodes and the render cache.
pub struct RenderContext<'tree> {
    /// Model stack.
    pub stack: Vec<GeometryNodeId>,

    pub hooks: RenderHooks,

    pub model_tree: &'tree ModelTree,

    /// The number of models to be rendered.
    models_to_render: usize,

    /// The number of model that been been rendered.
    models_rendered: usize,

    pub tree: GeometryTree,

    /// Progress is given as a percentage between 0.0 and 100.0.
    pub progress_tx: Option<ProgressTx>,

    /// Optional render cache.
    pub cache: Option<Shared<RenderCache>>,

    /// Optional render resolution. If none, we will use the `RenderResolution::default()`.
    pub resolution: Option<RenderResolution>,
}

impl<'tree> RenderContext<'tree> {
    /// Initialize context with current model and prerender model.
    pub fn new(model_tree: &'tree ModelTree) -> Self {
        let tree = GeometryTree::new(model_tree, RenderResolution::default());
        Self {
            stack: vec![],
            hooks: RenderHooks::new(),
            model_tree,
            models_rendered: 0,
            models_to_render: 0,
            tree,
            progress_tx: None,
            cache: None,
            resolution: None,
        }
    }

    /// Make a single progress step. A progress signal is sent with each new percentage.
    fn step(&mut self) {
        let old_percent = self.progress_in_percent();
        self.models_rendered += 1;
        let new_percent = self.progress_in_percent();

        // Check if integer percentage increased
        if (old_percent.floor() as u32) < (new_percent.floor() as u32)
            && let Some(progress_tx) = &mut self.progress_tx
        {
            progress_tx.send(new_percent).expect("No error");
        }
    }

    /// Return render progress in percent.
    pub fn progress_in_percent(&self) -> f32 {
        (self.models_rendered as f32 / self.models_to_render as f32) * 100.0
    }

    /// Update a geometry if it is not in cache.
    pub fn update(
        &mut self,
        f: impl FnOnce(GeometryNodeId, &mut RenderContext) -> RenderResult<GeometryOutput>,
    ) -> RenderResult<GeometryOutput> {
        let geo = self.geo_node();
        let hash = geo.to_hash();

        match self.cache.clone() {
            Some(cache) => {
                // 1. Concurrent Read Check (Shared Lock)
                {
                    let mut cache_read = cache.write().unwrap();
                    if let Some(geo) = cache_read.get(&hash) {
                        return Ok(geo.clone());
                    }
                } // Read lock is automatically dropped here so other threads aren't blocked during expensive geometry computation

                // 2. Compute Geometry Outside the Lock
                let (geo, cost) = self.call_with_cost(geo, f)?;

                // 3. Insert Result into Cache (Exclusive Write Lock)
                {
                    let mut cache_write = cache.write().unwrap();
                    // Optional double-check: in case another thread evaluated the exact same hash while we were computing
                    if let Some(cached_geo) = cache_write.get(&hash) {
                        return Ok(cached_geo.clone());
                    }

                    cache_write.insert_with_cost(hash, geo.clone(), cost);
                }

                Ok(geo)
            }
            None => Ok(f(geo, self)?),
        }
    }

    /// Return current render resolution.
    pub fn current_resolution(&self) -> RenderResolution {
        self.resolution.as_ref().cloned().unwrap_or_default()
    }

    // Return the generated item and the number of milliseconds.
    fn call_with_cost(
        &mut self,
        geo_node: GeometryNodeId,
        f: impl FnOnce(GeometryNodeId, &mut RenderContext) -> RenderResult<GeometryOutput>,
    ) -> RenderResult<(GeometryOutput, f64)> {
        use std::time::Instant;
        let start = Instant::now();

        let r = f(geo_node, self)?;

        let duration = start.elapsed();
        Ok((r, (duration.as_nanos() as f64) / 1_000_000.0))
    }

    fn geo_node(&self) -> GeometryNodeId {
        self.stack.last().copied().unwrap()
    }

    pub fn model(&self) -> ModelNodeRef<'tree> {
        let geo_node = self.geo_node();
        let node = self.tree.arena.get(geo_node).unwrap();
        node.get().model(self.model_tree)
    }
}
