// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The resolve compiler stage.
//!
//! It consists of the following steps:
//! 1) Load the (optional) manifest file
//! 2) Prepare built-in library.
//! 3) Load all external dependencies, including `std`.
//! 4) Load source files inside the workspace.
//! 5) `bind`: Resolve [`mir::Path`] to [`SymbolId`]s/[`LocalId`]s.
//! 6) `case_check`: Validate identifier casing rules.
//! 7) `type_check`: Verify expression types.

//! Each sub-step is implemented in a separate module.

mod bind;
mod cache;
mod case_check;
mod loader;
pub mod stack;
mod type_check;
pub mod visitor;

use microcad_lang_base::{CompilationResult, HashId, IssueList, PushIssue, Shared};

use microcad_lang_lower::Ir;

use crate::{Library, ResolveError, ResolveInfo, ResolveIssue, ResolveWarning};

pub use crate::resolve::cache::{LibraryCache, SourceCache};

#[derive(Debug, Default)]
pub struct ResolveContext {
    pub lib_search_paths: Vec<std::path::PathBuf>,
    pub lib_cache: Shared<LibraryCache>,
    pub src_cache: Shared<SourceCache>,
}

impl ResolveContext {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ResolveContext {
    /// Add a new search paths.
    pub fn with_search_path(mut self, path: impl AsRef<std::path::Path>) -> Self {
        self.lib_search_paths.push(path.as_ref().to_path_buf());
        self
    }

    /// Get intermediate representation by hash id.
    pub fn get_ir(&self, id: HashId) -> Option<Ir> {
        self.src_cache
            .read_unwrap()
            .get(id)
            .and_then(|source_unit| source_unit.ir().cloned())
    }
}

/// A Resolve Context to resolve exactly one library at a time.
pub struct ResolveLibraryContext<'ctx, 'lib> {
    pub issues: IssueList<ResolveIssue>,
    pub lib: &'lib mut Library,
    pub ctx: &'ctx mut ResolveContext,
}

/// The scaffolding step builds the unresolved tree from a workspace directory.
///
/// The workspace can be resolved in two modes:
/// - `Lib`: We resolve the workspace as a library by loading the `lib.mu` file, if it exists.
///   If it does not exist, we load every file in the workspace root directory.
/// - `Source`: We resolve a single source file within the workspace and only load its dependencies.
///
/// Optionally, the workspace can contain a `mu.toml` manifest file.
/// The `mu.toml` contain metadata for the µcad library and defines external dependencies.
///
/// A workspace directory can look like this:
///
/// /my_project
/// ├── mu.toml # Optional, but mandatory for publishing.
/// ├── lib.mu # Optional. If it is not present and if we are in Lib mode, we load all files in the directory.
/// ├── foo.mu # A file in the workspace
/// ├── /foo # A subdirectory
/// |   ├── bar.mu
/// |   ... # More files
/// ├── baz.mu
impl<'ctx, 'lib> ResolveLibraryContext<'ctx, 'lib> {
    pub fn new(lib: &'lib mut Library, ctx: &'ctx mut ResolveContext) -> Self {
        Self {
            lib,
            ctx,
            issues: Default::default(),
        }
    }

    pub fn lib(&'lib self) -> &'lib Library {
        self.lib
    }

    pub fn ctx(&'ctx self) -> &'ctx ResolveContext {
        self.ctx
    }

    pub fn issues(self) -> IssueList<ResolveIssue> {
        self.issues
    }
}

impl<'ctx, 'lib> PushIssue<ResolveIssue> for ResolveLibraryContext<'ctx, 'lib> {
    fn push_issue(&mut self, issue: impl Into<ResolveIssue>) {
        self.issues.push_issue(issue);
    }

    fn push_err(&mut self, err: impl Into<ResolveError>) {
        self.issues.push_err(err);
    }

    fn push_warn(&mut self, warn: impl Into<ResolveWarning>) {
        self.issues.push_warn(warn);
    }

    fn push_info(&mut self, info: impl Into<ResolveInfo>) {
        self.issues.push_info(info);
    }
}

pub fn resolve(
    path: impl AsRef<std::path::Path>,
    ctx: &mut ResolveContext,
) -> CompilationResult<Library, ResolveIssue> {
    let mut lib = Library::new();
    let mut lib_ctx = ResolveLibraryContext::new(&mut lib, ctx);

    match lib_ctx.load(path) {
        Ok(()) => {
            let issues = lib_ctx.issues();
            Ok((lib, issues))
        }
        Err(err) => {
            let mut issues = lib_ctx.issues();
            issues.push_err(err);
            Err(issues)
        }
    }
}
