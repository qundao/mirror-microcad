// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer settings.

use std::time::Duration;

use bevy::ecs::resource::Resource;
use microcad_core::{RenderResolution, Scalar};

#[derive(Resource, serde::Deserialize, Clone)]
pub struct Config {
    /// Additional search paths for microcad interpreter.
    pub search_paths: Vec<std::path::PathBuf>,

    /// Delay when the input file is reloaded.
    pub reload_delay: Duration,

    /// Window stays on top.
    pub stay_on_top: bool,

    /// Render resolution in mm (default = 0.25mm)
    pub render_resolution: Scalar,

    /// Export resolution in mm (default = 0.1mm)
    pub export_resolution: Scalar,
}

impl Config {
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

impl Default for Config {
    fn default() -> Self {
        Self {
            search_paths: Self::default_search_paths(),
            reload_delay: Duration::from_millis(200),
            stay_on_top: false,
            render_resolution: RenderResolution::medium().linear,
            export_resolution: RenderResolution::high().linear,
        }
    }
}
