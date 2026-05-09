// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad driver is a high-level API to be integrated in LSP, CLI or Viewer.

pub mod commands;
mod config;
mod document;
mod session;
mod watcher;

pub use microcad_lang::lower::ir::SourceFile;
pub use microcad_lang::model::Model;
pub use microcad_lang::render::{RenderCache, RenderContext};
pub use microcad_lang_base::{RcMut, Url};

pub use config::Config;
pub use document::Document;
pub use session::Session;
pub use watcher::Watcher;
