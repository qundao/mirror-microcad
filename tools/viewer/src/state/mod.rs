// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer State module.

use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use bevy::ecs::resource::Resource;

use crate::{Config, plugin::MicrocadPluginInput, processor::ProcessorInterface, scene::Scene};

#[derive(Resource)]
pub struct State {
    pub input: Option<MicrocadPluginInput>,
    pub config: Config,
    pub last_modified: Arc<Mutex<Option<SystemTime>>>,
    pub scene: Scene,
    pub processor: ProcessorInterface,
}

impl State {
    /// Create new state from arguments
    pub fn new(input: Option<MicrocadPluginInput>, config: Config) -> Self {
        Self {
            input,
            config,
            last_modified: Default::default(),
            scene: Default::default(),
            processor: ProcessorInterface::run(),
        }
    }
}
