// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Config, document};

mod doc_gen;
mod export;
mod format;
mod print_diagnostics;
mod render;

pub use doc_gen::*;
pub use export::*;
pub use format::*;
pub use print_diagnostics::*;
pub use render::*;
/// Load something from a file
pub trait LoadFromFile {
    fn load_from_file(&mut self) -> document::Result;
}

pub trait Pipeline {
    fn parse(&mut self) -> document::Result;
    fn lower(&mut self) -> document::Result;
    fn resolve(&mut self, config: &Config) -> document::Result;
    fn eval(&mut self) -> document::Result;

    fn run_pipeline(&mut self, config: &Config) -> document::Result {
        self.parse()?;
        self.lower()?;
        self.resolve(config)?;
        self.eval()
    }
}

/// Check a document for errors.
pub trait Check {
    fn check(&mut self, config: &Config) -> document::Result<bool>;
}

/// Write document contents to file
pub trait Sync {
    fn sync(&self) -> document::Result;
}
