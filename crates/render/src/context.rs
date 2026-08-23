// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render context

use std::sync::{Arc, RwLock, mpsc};

use microcad_hash::ToHash;
use microcad_lang_types::ModelNodeRef;

use crate::{GeometryOutput, RenderCache, RenderResolution, RenderResult};

/// Our progress sender.
pub type ProgressTx = mpsc::Sender<f32>;

/// The render context.
///
/// Keeps a stack of model nodes and the render cache.
#[derive(Default)]
pub struct RenderContext<'tree> {
    /// Model stack.
    pub model_stack: Vec<ModelNodeRef<'tree>>,

    /// Optional render cache.
    pub cache: Option<Arc<RwLock<RenderCache>>>,

    /// The number of models to be rendered.
    models_to_render: usize,

    /// The number of model that been been rendered.
    models_rendered: usize,

    /// Progress is given as a percentage between 0.0 and 100.0.
    pub progress_tx: Option<ProgressTx>,
}

impl<'tree> RenderContext<'tree> {
    /// Initialize context with current model and prerender model.
    pub fn new(
        _model: &ModelNodeRef<'tree>,
        _resolution: RenderResolution,
        _progress_tx: Option<ProgressTx>,
    ) -> RenderResult<Self> {
        todo!() /*  Ok(Self {
        model_stack: vec![model.clone()],
        cache,
        models_rendered: 0,
        progress_tx,
        models_to_render: // model.prerender(resolution)?,
        }) */
    }

    /// The current model (panics if it is none).
    pub fn model(&self) -> ModelNodeRef<'tree> {
        *self.model_stack.last().expect("A model")
    }

    /// Run the closure `f` within the given `model`.
    pub fn with_model<T>(
        &mut self,
        model: ModelNodeRef<'tree>,
        f: impl FnOnce(&mut RenderContext) -> T,
    ) -> T {
        self.model_stack.push(model);
        let result = f(self);
        self.model_stack.pop();

        self.step();

        result
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
        f: impl FnOnce(&mut RenderContext, ModelNodeRef<'tree>) -> RenderResult<GeometryOutput>,
    ) -> RenderResult<GeometryOutput> {
        let model = self.model();
        let hash = model.to_hash();

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
                let (geo, cost) = self.call_with_cost(model, f)?;

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
            None => Ok(f(self, model)?),
        }
    }

    /// Return current render resolution.
    pub fn current_resolution(&self) -> RenderResolution {
        todo!()
        //        self.model().resolution()
    }

    // Return the generated item and the number of milliseconds.
    fn call_with_cost(
        &mut self,
        model: ModelNodeRef<'tree>,
        f: impl FnOnce(&mut RenderContext, ModelNodeRef<'tree>) -> RenderResult<GeometryOutput>,
    ) -> RenderResult<(GeometryOutput, f64)> {
        use std::time::Instant;
        let start = Instant::now();

        let r = f(self, model)?;

        let duration = start.elapsed();
        Ok((r, (duration.as_nanos() as f64) / 1_000_000.0))
    }
}
