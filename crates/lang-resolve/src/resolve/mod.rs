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
mod resolver;
pub mod stack;
mod type_check;
pub mod visitor;

use microcad_lang_base::{
    CompilationResult, GetSourceByHash, HashId, LibraryId, PushDiag, SymbolId,
};

use microcad_lang_lower::{Ir, ir::UnresolvedPath};
pub use resolver::Resolver;

use crate::{
    Library, ResolveResult,
    error::ResolveError,
    library::SymbolNodeId,
    resolve::cache::{LibraryCache, SourceCache},
};

/// A Resolve Context to resolve exactly one library at a time.
pub struct ResolveContext<'lib> {
    // pub resolver: Box<dyn Resolver>,
    pub diag: Vec<ResolveError>,
    pub lib_search_paths: Vec<std::path::PathBuf>,
    pub lib: &'lib mut Library,
    pub lib_cache: LibraryCache,
    pub src_cache: SourceCache,
}

impl<'lib> PushDiag<ResolveError> for ResolveContext<'lib> {
    fn push_diag(&mut self, err: impl Into<ResolveError>) {
        self.diag.push(err.into());
    }
}

impl<'lib> GetSourceByHash for ResolveContext<'lib> {
    fn get_source_by_hash(
        &'_ self,
        hash: microcad_lang_base::HashId,
    ) -> Option<&microcad_lang_base::Source> {
        self.src_cache.get_source_by_hash(hash)
    }
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
impl<'lib> ResolveContext<'lib> {
    pub fn new(lib: &'lib mut Library) -> Self {
        Self {
            diag: Default::default(),
            lib_search_paths: vec![microcad_std::global_library_search_path()],
            lib,
            lib_cache: LibraryCache::default(),
            src_cache: SourceCache::default(),
        }
    }

    /// Add a new search paths.
    pub fn with_search_path(mut self, path: impl AsRef<std::path::Path>) -> Self {
        self.lib_search_paths.push(path.as_ref().to_path_buf());
        self
    }

    /// Get intermediate representation by hash id.
    pub fn get_ir(&self, id: HashId) -> Option<&Ir> {
        self.src_cache
            .get(id)
            .and_then(|source_unit| source_unit.ir())
    }

    pub fn lib(&'lib self) -> &'lib Library {
        self.lib
    }
}

pub fn resolve(
    path: impl AsRef<std::path::Path>,
    ctx: &mut ResolveContext,
) -> CompilationResult<Library, ResolveError> {
    /*
    let mut context = ResolveContext::new(resolver);

    // Step 1: Scaffold symbol hierarchy
    let scaffolded = context.scaffold()?;

    // Step 2: Name resolution (SymbolPath -> SymbolId/LocalId)
        let bound = self.bind(scaffolded)?;

        // Step 3: Naming style & identifier checks
        self.case_check(&bound)?;

        // Step 4: Type checking & inference
        let typed = self.type_check(bound)?;

        // Step 5: Canonicalize/normalize expressions (if enabled)
        let normalized = self.normalize(typed)?;

        // Step 6: Lower to RST and resolve remaining attributes
        let rst = self.reduce(normalized)?;
    Ok((rst, context.diagnostics))
    */
    todo!()
}
