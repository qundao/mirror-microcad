// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer settings.

pub mod theme;

use std::time::Duration;
pub use theme::Theme;

use bevy::ecs::resource::Resource;
use microcad_core::{RenderResolution, Scalar};

/// Viewer configuration.
#[derive(Resource, serde::Deserialize, Clone)]
pub struct Config {
    /// Additional search paths for microcad interpreter.
    pub search_paths: Vec<std::path::PathBuf>,

    /// Delay when the input file is reloaded.
    pub reload_delay: Duration,

    /// Window stays on top.
    pub stay_on_top: bool,

    /// Mesh smoothness threshold angle (default = 20°)
    pub mesh_smoothness_angle: Scalar,

    /// Render resolution in mm (default = 0.25mm)
    pub render_resolution: Scalar,

    /// Export resolution in mm (default = 0.1mm)
    pub export_resolution: Scalar,

    /// The viewer theme.
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            search_paths: microcad_builtin::dirs::default_search_paths(),
            reload_delay: Duration::from_millis(100),
            stay_on_top: false,
            mesh_smoothness_angle: 20.0,
            render_resolution: RenderResolution::medium().linear,
            export_resolution: RenderResolution::high().linear,
            theme: Theme::default(),
        }
    }
}
