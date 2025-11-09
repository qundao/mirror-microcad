// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer State module.

mod cursor;
mod model;

use bevy::ecs::resource::Resource;

use crate::{Config, plugin::MicrocadPluginInput, processor::ProcessorInterface, scene::Scene};

pub use cursor::Cursor;
pub use model::ModelViewState;

#[derive(Resource)]
pub struct State {
    pub input: Option<MicrocadPluginInput>,
    pub config: Config,
    pub scene: Scene,
    pub cursor: Cursor,
    pub processor: ProcessorInterface,
}

impl State {
    /// Create new state from arguments
    pub fn new(input: Option<MicrocadPluginInput>, config: Config) -> Self {
        Self {
            input,
            config,
            cursor: Default::default(),
            scene: Default::default(),
            processor: ProcessorInterface::run(),
        }
    }
}
