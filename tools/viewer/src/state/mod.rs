// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer State module.

use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use bevy::ecs::resource::Resource;

use crate::{Settings, processor::ProcessorInterface, scene::Scene};

#[derive(Resource)]
pub struct State {
    pub input: std::path::PathBuf,
    pub settings: Settings,
    pub last_modified: Arc<Mutex<Option<SystemTime>>>,
    pub scene: Scene,
    pub processor: ProcessorInterface,
}

impl State {
    /// Create new state from arguments
    pub fn new(input: std::path::PathBuf, settings: Settings) -> Self {
        Self {
            input,
            settings,
            last_modified: Default::default(),
            scene: Default::default(),
            processor: ProcessorInterface::run(),
        }
    }
}
