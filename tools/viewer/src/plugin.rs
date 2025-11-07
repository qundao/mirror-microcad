// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use bevy::app::{App, Plugin, Startup, Update};
use bevy_mod_outline::OutlinePlugin;

use bevy::prelude::*;

use crate::*;

#[derive(Clone)]
pub enum MicrocadPluginMode {
    Empty,
    /// Load and watch an input file.
    InputFile(std::path::PathBuf),
    /// Remove-controlled via stdin.
    Stdin,
}

pub struct MicrocadPlugin {
    pub mode: MicrocadPluginMode,
    pub config: Config,
}

impl Plugin for MicrocadPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(self.config.theme.primary.to_bevy()))
            .insert_resource(State::new(self.mode.clone(), self.config.clone()))
            .add_plugins((OutlinePlugin, MeshPickingPlugin))
            .add_plugins(processor::ProcessorPlugin)
            .add_plugins(material::MaterialPlugin)
            .add_plugins(scene::ScenePlugin)
            .add_systems(Startup, apply_window_settings)
            .add_systems(Update, stdin::handle_stdin_messages);
    }
}

fn apply_window_settings(state: Res<State>, mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut().expect("Some window");

    window.title = match &state.mode {
        MicrocadPluginMode::Empty => "µcad".to_string(),
        MicrocadPluginMode::InputFile(input) => format!("µcad - {}", input.display()),
        MicrocadPluginMode::Stdin => "µcad -- remote-controlled".to_string(), // To display current file or name here.
    };
    window.window_level = match state.config.stay_on_top {
        true => bevy::window::WindowLevel::AlwaysOnTop,
        false => bevy::window::WindowLevel::Normal,
    };
}
