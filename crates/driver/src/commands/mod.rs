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
    fn load_from_file(&self) -> document::Result;
}

pub trait Pipeline {
    fn parse(&self) -> document::Result;
    fn lower(&self, config: &Config) -> document::Result;
    fn resolve(&self) -> document::Result;
    fn eval(&self) -> document::Result;

    fn run_pipeline(&self, config: &Config) -> document::Result {
        self.parse()?;
        self.lower(config)?;
        self.resolve()?;
        self.eval()
    }
}

/// Check a document for errors.
pub trait Check {
    fn check(&self, config: &Config) -> document::Result<bool>;
}

/// Write document contents to file
pub trait Sync {
    fn sync(&self) -> document::Result;
}
