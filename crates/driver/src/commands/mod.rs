// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Result;

mod doc_gen;
mod export;
mod format;
mod print_diagnostics;

pub mod compile;
pub mod render;

pub use compile::{Compile, CompileParameters};
pub use doc_gen::*;
pub use export::*;
pub use format::*;
pub use print_diagnostics::*;
pub use render::*;

/// Load something from a file
pub trait LoadFromFile {
    fn load_from_file(&mut self) -> Result;
}

/// Write document contents to file
pub trait Sync {
    fn sync(&self) -> Result;
}
