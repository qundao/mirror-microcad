// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer settings.

use std::time::Duration;

use bevy::ecs::resource::Resource;

#[derive(Resource, Clone)]
pub struct Settings {
    /// Search paths for microcad interpreter.
    pub search_paths: Vec<std::path::PathBuf>,

    /// Delay when the input file is reloaded.
    pub reload_delay: Duration,
}

impl Settings {
    /// `./lib` (if exists) and `~/.config/microcad/lib` (if exists).
    pub fn default_search_paths() -> Vec<std::path::PathBuf> {
        let local_dir = std::path::PathBuf::from("./lib");
        let mut search_paths = Vec::new();

        if let Some(global_root_dir) = Self::global_root_dir()
            && global_root_dir.exists()
        {
            search_paths.push(global_root_dir);
        }
        if local_dir.exists() {
            search_paths.push(local_dir);
        }

        search_paths
    }

    /// Returns microcad's config dir, even if it does not exist.
    ///
    /// On Linux, the config dir is located in `~/.config/microcad`.
    pub fn config_dir() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|dir| dir.join("microcad"))
    }

    /// Returns global root dir, even if it does not exist.
    ///
    /// On Linux, the root dir is located in `~/.config/microcad/lib`.
    pub fn global_root_dir() -> Option<std::path::PathBuf> {
        Self::config_dir().map(|dir| dir.join("lib"))
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            search_paths: Self::default_search_paths(),
            reload_delay: Duration::from_millis(500),
        }
    }
}
