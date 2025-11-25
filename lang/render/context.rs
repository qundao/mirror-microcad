// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render context

use std::sync::mpsc;

use microcad_core::RenderResolution;

use crate::{model::Model, rc::RcMut, render::*};

/// Our progress sender.
pub type ProgressTx = mpsc::Sender<f32>;

/// The render context.
///
/// Keeps a stack of model nodes and the render cache.
#[derive(Default)]
pub struct RenderContext {
    /// Model stack.
    pub model_stack: Vec<Model>,

    /// Optional render cache.
    pub cache: Option<RcMut<RenderCache>>,

    /// The number of models to be rendered.
    models_to_render: usize,

    /// The number of model that been been rendered.
    models_rendered: usize,

    /// Progress is given as a percentage between 0.0 and 100.0.
    pub progress_tx: Option<ProgressTx>,
}

impl RenderContext {
    /// Initialize context with current model and prerender model.
    pub fn new(
        model: &Model,
        resolution: RenderResolution,
        cache: Option<RcMut<RenderCache>>,
        progress_tx: Option<ProgressTx>,
    ) -> RenderResult<Self> {
        Ok(Self {
            model_stack: vec![model.clone()],
            cache,
            models_to_render: model.prerender(resolution)?,
            models_rendered: 0,
            progress_tx,
        })
    }

    /// The current model (panics if it is none).
    pub fn model(&self) -> Model {
        self.model_stack.last().expect("A model").clone()
    }

    /// Run the closure `f` within the given `model`.
    pub fn with_model<T>(&mut self, model: Model, f: impl FnOnce(&mut RenderContext) -> T) -> T {
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

    /// Update a 2D geometry if it is not in cache.
    pub fn update_2d<T: Into<WithBounds2D<Geometry2D>>>(
        &mut self,
        f: impl FnOnce(&mut RenderContext, Model) -> RenderResult<T>,
    ) -> RenderResult<Geometry2DOutput> {
        let model = self.model();
        let hash = model.computed_hash();

        match self.cache.clone() {
            Some(cache) => {
                {
                    let mut cache = cache.borrow_mut();
                    if let Some(GeometryOutput::Geometry2D(geo)) = cache.get(&hash) {
                        return Ok(geo.clone());
                    }
                }
                {
                    let (geo, cost) = self.call_with_cost(model, f)?;
                    let geo: Geometry2DOutput = Rc::new(geo.into());
                    let mut cache = cache.borrow_mut();
                    cache.insert_with_cost(hash, geo.clone(), cost);
                    Ok(geo)
                }
            }
            None => Ok(Rc::new(f(self, model)?.into())),
        }
    }

    /// Update a 3D geometry if it is not in cache.
    pub fn update_3d<T: Into<WithBounds3D<Geometry3D>>>(
        &mut self,
        f: impl FnOnce(&mut RenderContext, Model) -> RenderResult<T>,
    ) -> RenderResult<Geometry3DOutput> {
        let model = self.model();
        let hash = model.computed_hash();
        match self.cache.clone() {
            Some(cache) => {
                {
                    let mut cache = cache.borrow_mut();
                    if let Some(GeometryOutput::Geometry3D(geo)) = cache.get(&hash) {
                        return Ok(geo.clone());
                    }
                }
                {
                    let (geo, cost) = self.call_with_cost(model, f)?;
                    let geo: Geometry3DOutput = Rc::new(geo.into());
                    let mut cache = cache.borrow_mut();
                    cache.insert_with_cost(hash, geo.clone(), cost);
                    Ok(geo)
                }
            }
            None => Ok(Rc::new(f(self, model)?.into())),
        }
    }

    /// Return current render resolution.
    pub fn current_resolution(&self) -> RenderResolution {
        self.model().borrow().resolution()
    }

    // Return the generated item and the number of milliseconds.
    fn call_with_cost<T>(
        &mut self,
        model: Model,
        f: impl FnOnce(&mut RenderContext, Model) -> RenderResult<T>,
    ) -> RenderResult<(T, f64)> {
        use std::time::Instant;
        let start = Instant::now();

        let r = f(self, model)?;

        let duration = start.elapsed();
        Ok((r, (duration.as_nanos() as f64) / 1_000_000.0))
    }
}
