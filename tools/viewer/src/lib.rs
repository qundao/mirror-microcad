// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Microcad viewer library.

pub mod asset;
pub mod plugin;
pub mod processor;
pub mod scene;
pub mod settings;
pub mod state;
pub mod stdin;
pub mod watcher;

pub use crate::settings::Settings;

pub use crate::state::State;

pub use crate::plugin::MicrocadPlugin;
