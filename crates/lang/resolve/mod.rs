// Copyright © 2025 The µcad authors <info@ucad.xyz>
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
mod symbol_table;
mod symbolize;

use crate::{diag::*, rc::*, syntax::*};
pub use externals::*;
pub use lookup::*;
pub use resolve_context::*;
pub use resolve_error::*;
pub use sources::*;
pub use symbol::*;
pub use symbol_table::*;

use grant::*;
use names::*;

/// Trait for items which can be fully qualified.
pub trait FullyQualify {
    /// Get a fully (up to root of symbol map) qualified name.
    fn full_name(&self) -> QualifiedName;
}

impl GetSourceByHash for Rc<SourceFile> {
    fn get_by_hash(&self, hash: u64) -> ResolveResult<std::rc::Rc<SourceFile>> {
        if hash == self.hash {
            Ok(self.clone())
        } else if hash == 0 {
            Err(ResolveError::NulHash)
        } else {
            Err(ResolveError::UnknownHash(hash))
        }
    }
}
