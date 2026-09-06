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

//!
//! Each sub-step is implemented in a separate module.

mod bind;
mod case_check;
mod resolver;
mod type_check;

use microcad_lang_base::{CompilationResult, GetSourceByHash, PushDiag};

pub use resolver::Resolver;

use crate::{Library, ResolveResult, error::ResolveError};

/// Resolve Context
pub struct ResolveContext {
    // pub resolver: Box<dyn Resolver>,
    pub diag: Vec<ResolveError>,
    pub lib_search_paths: Vec<std::path::PathBuf>,
    pub std_lib: Option<Library>,
}

impl PushDiag<ResolveError> for ResolveContext {
    fn push_diag(&mut self, err: impl Into<ResolveError>) {
        self.diag.push(err.into());
    }
}

impl GetSourceByHash for ResolveContext {
    fn get_source_by_hash(
        &'_ self,
        hash: microcad_lang_base::HashId,
    ) -> Option<&microcad_lang_base::Source> {
        // TODO implement source cache
        None
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
impl ResolveContext {
    pub fn new() -> Self {
        Self {
            diag: Default::default(),
            std_lib: None,
            lib_search_paths: vec![microcad_std::global_library_search_path()],
        }
    }

    /// Add a new search paths.
    pub fn with_search_path(mut self, path: impl AsRef<std::path::Path>) -> Self {
        self.lib_search_paths.push(path.as_ref().to_path_buf());
        self
    }

    /// Load a custom version of the standard library from a path.
    pub fn with_std(mut self, path: impl AsRef<std::path::Path>) -> Self {
        let lib = Library::load(path, &mut self);
        self.std_lib = lib.ok();
        self
    }

    /// Load standard library from path.
    ///
    /// Installs the standard library, if it is not installed.
    fn _load_std(&mut self, path: impl AsRef<std::path::Path>) -> ResolveResult<Library> {
        let path = path.as_ref();

        let std_lib = match Library::load(&path, self) {
            Ok(std_lib) => {
                let loaded_version = std_lib.version().expect("A version");
                let expected_version = microcad_std::version();
                if loaded_version != &expected_version {
                    eprintln!(
                        "µcad standard library version mismatch: {loaded_version} != {expected_version}",
                    );

                    // Handle version mismatch, force re-install
                    microcad_std::StdLib::reinstall(true).map_err(ResolveError::new)?;
                    Library::load(&path, self)?
                } else {
                    std_lib
                }
            }
            Err(err) => {
                self.push_diag(err);
                // Install the library and try to load it.
                match microcad_std::StdLib::install(&path) {
                    Ok(_) => Library::load(path, self)?,
                    Err(err) => return Err(ResolveError::new(err)),
                }
            }
        };

        Ok(std_lib)
    }

    pub fn try_load_std(&mut self) -> ResolveResult<()> {
        // A standard library is already loaded, we can stop here.
        if self.std_lib.is_some() {
            return Ok(());
        }

        let paths = self.lib_search_paths.clone();
        self.std_lib = paths.iter().find_map(|search_path| {
            let std_lib_path =
                search_path.join(format!("std-{ver}", ver = microcad_std::version()));

            match self._load_std(std_lib_path) {
                Ok(std_lib) => Some(std_lib),
                Err(err) => {
                    self.push_diag(err);
                    None
                }
            }
        });
        Ok(())
    }
}

pub fn resolve(_resolver: Box<dyn Resolver>) -> CompilationResult<Library, ResolveError> {
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
