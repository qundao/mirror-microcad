// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The context of geometry processor.

use microcad_lang::{model::Model, rc::RcMut, render::RenderCache, syntax::SourceFile};

use crate::{config, processor::registry::InstanceRegistry};

/// The current state of the processor.
#[derive(Debug, Clone, Default)]
pub enum ProcessingState {
    #[default]
    /// The processor does currently nothing.
    Idle,
    /// The processor is busy (with progress between 0..100.0)
    Busy(f32),
    /// The processor is in an error state.
    Error,
}

/// The context of the processor.
///
/// It caches the relevant data to assure that only necessary models will be rerendered.
pub struct ProcessorContext {
    pub(super) state: ProcessingState,

    /// Flag to tell whether to initialize.
    pub(super) initialized: bool,

    /// Search paths are set during initialization.
    pub(super) search_paths: Vec<std::path::PathBuf>,

    /// The current render resolutions.
    pub(super) resolution: microcad_core::RenderResolution,
    pub(super) theme: config::Theme,

    pub(super) line_number: Option<u32>,

    /// The current source file being processed (if any).
    pub(super) source_file: Option<std::rc::Rc<SourceFile>>,

    /// Model resulted from an evaluation.
    pub(super) model: Option<Model>,

    /// Keeps a UUID register of all model instances that have been rendered.
    pub(super) instance_registry: InstanceRegistry,

    /// µcad Render cache.
    pub(super) render_cache: RcMut<RenderCache>,
}

impl Default for ProcessorContext {
    fn default() -> Self {
        Self {
            state: Default::default(),
            initialized: false,
            search_paths: Default::default(),
            resolution: Default::default(),
            theme: Default::default(),
            source_file: None,
            model: None,
            line_number: None,
            instance_registry: Default::default(),
            render_cache: RcMut::new(RenderCache::new()),
        }
    }
}
