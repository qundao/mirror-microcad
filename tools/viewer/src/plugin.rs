// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use bevy::app::{App, Plugin, Startup, Update};
use bevy_mod_outline::OutlinePlugin;

use bevy::prelude::*;

use crate::Settings;

pub struct MicrocadPlugin {
    pub input: std::path::PathBuf,
    pub settings: Settings,
}

impl Plugin for MicrocadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OutlinePlugin, MeshPickingPlugin))
            .add_plugins(crate::processor::ProcessorPlugin)
            .add_plugins(crate::scene::ScenePlugin)
            .insert_resource(crate::stdin::MessageReceiver::run())
            .insert_resource(crate::state::State::new(
                self.input.clone(),
                self.settings.clone(),
            ))
            .add_systems(Startup, crate::watcher::start_file_watcher)
            .add_systems(Startup, set_window_title)
            .add_systems(Update, crate::stdin::handle_messages)
            .add_systems(Update, crate::watcher::handle_external_reload);
    }
}

fn set_window_title(state: Res<crate::State>, mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut().expect("Some window");
    window.title = format!("µcad - {}", state.input.display());
}
