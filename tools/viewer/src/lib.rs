// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Microcad viewer library.

pub mod config;
pub mod material;
pub mod plugin;
pub mod processor;
pub mod scene;
pub mod state;
pub mod stdin;
pub mod to_bevy;

pub use crate::config::Config;
pub use crate::to_bevy::ToBevy;

pub use crate::state::State;

pub use crate::plugin::MicrocadPlugin;
