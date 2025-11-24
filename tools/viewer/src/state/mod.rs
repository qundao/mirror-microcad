// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer State module.

mod cursor;
mod event;
mod model;

use bevy::ecs::resource::Resource;

use crate::{
    Config,
    plugin::MicrocadPluginInput,
    processor::{ProcessingState, ProcessorInterface},
    scene::Scene,
};

pub use cursor::Cursor;
pub use event::{ViewerEvent, handle_viewer_event};
pub use model::ModelViewState;

/// The application state (the bevy view model).
#[derive(Resource)]
pub struct State {
    /// Input interface (e.g. file or stdin).
    pub input: Option<MicrocadPluginInput>,
    /// The configuration settings of loaded at startup.
    pub config: Config,
    /// The scene entities to be spawned and rendered.
    pub scene: Scene,
    /// Information at cursor positions (view cursor and editor cursor).
    pub cursor: Cursor,
    /// The µcad geometry processor.
    pub processor: ProcessorInterface,
    /// The current processing state.
    pub processing_state: ProcessingState,
}

impl State {
    /// Create new state from arguments.
    pub fn new(input: Option<MicrocadPluginInput>, config: Config) -> Self {
        Self {
            input,
            config,
            cursor: Default::default(),
            scene: Default::default(),
            processor: ProcessorInterface::run(),
            processing_state: Default::default(),
        }
    }
}
