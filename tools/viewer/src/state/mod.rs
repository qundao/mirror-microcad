// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer State module.

use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use bevy::ecs::resource::Resource;

use crate::{
    Config, plugin::MicrocadPluginMode, processor::ProcessorInterface, scene::Scene,
    stdin::StdinMessageReceiver,
};

#[derive(Resource)]
pub struct State {
    pub mode: MicrocadPluginMode,
    pub config: Config,
    pub last_modified: Arc<Mutex<Option<SystemTime>>>,
    pub scene: Scene,
    pub processor: ProcessorInterface,
    pub stdin: Option<StdinMessageReceiver>,
}

impl State {
    /// Create new state from arguments
    pub fn new(mode: MicrocadPluginMode, config: Config) -> Self {
        Self {
            mode,
            config,
            last_modified: Default::default(),
            scene: Default::default(),
            processor: ProcessorInterface::run(),
            stdin: None,
        }
    }
}
