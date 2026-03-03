// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Single symbol resolving
//!
//! After parsing a source file (see [`mod@crate::parse`]) it must be resolved to get a symbol out of it:
//!
//! ```no_run
//! use microcad_lang::{syntax::*, parse::*, resolve::*, diag::*};
//! let source_file = SourceFile::load("my.µcad").expect("parsing success");
//! let mut context = ResolveContext::create(
//!     source_file,
//!     &["./std/lib"],
//!     None,
//!     DiagHandler::default(),
//! ).unwrap();
//! ```
//!
//! To "run" the source file (and get the expected output) it must now be evaluated (see [`crate::eval`])  .

mod externals;
mod grant;
mod lookup;
mod names;
mod resolve_context;
mod resolve_error;
mod sources;
mod symbol;
mod symbolize;

use crate::{diag::*, syntax::*};
pub use externals::*;
pub use lookup::*;
pub use resolve_context::*;
pub use resolve_error::*;
pub use sources::*;
pub use symbol::*;

use grant::*;
use names::*;

/// Trait for items which can be fully qualified.
pub trait FullyQualify {
    /// Get a fully (up to root of symbol map) qualified name.
    fn full_name(&self) -> QualifiedName;
}
