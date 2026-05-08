// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, RcMut};

pub type CommandResult<T> = Result<T, RcMut<Diagnostics>>;

mod doc_gen;
mod export;

pub use doc_gen::*;

/// Check a document for errors.
pub trait Check {
    fn check(&self) -> CommandResult<()>;
}

/// Format a document.
pub trait Format {
    fn format(&self) -> CommandResult<bool>;
}

/// Write document contents to file
pub trait Sync {
    fn sync(&self) -> CommandResult<()>;
}
