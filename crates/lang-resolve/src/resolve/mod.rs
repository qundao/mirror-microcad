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
//! 8) `normalize`: Canonicalize and simplify expressions.
//! 9) `reduce`: Lower to RST and evaluate attributes.
//!
//! Each sub-step is implemented in a separate module.

mod bind;
mod case_check;
mod error;
mod resolver;
mod type_check;

use microcad_lang_base::{CompilationResult, Diagnostics};

pub use error::ResolveError;

pub use resolver::Resolver;

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, Box<ResolveError>>;

/// Resolve Context
pub struct ResolveContext {
    pub resolver: Box<dyn Resolver>,
    pub diagnostics: Diagnostics,
}

/// The scaffolding step builds the unresolved tree from a workspace directory.
///
/// The workspace can be resolved in two modes:
/// - `Lib`: We resolve the workspace as a library by loading the `lib.mu` file, if it exists.
///          If it does not exist, we load every file in the workspace root directory.
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
    pub fn new(resolver: Box<dyn Resolver>) -> Self {
        Self {
            resolver,
            diagnostics: Diagnostics::default(),
        }
    }

    pub fn scaffold(&mut self) {
        // Try to load on optional `mu.toml` manifest file.
        //let manifest: Option<Manifest> = self.resolver.load_manifest()?;

        todo!()

        //let mut builder = TreeBuilder::new(mir::Workspace::from(manifest));

        // Handle loading externals
        /*
        // Add externals to the tree (the mu node)
        builder.enter(UnresolvedSymbolDef::Externals);

        // Add the built-in library
        builder.add(builtin());

        match manifest { // Try load mu.toml
            // A `mu.toml` exists within the workspace directory
            Some(manifest) => {
                if !manifest.no_std {
                    builder.add(resolver.load_std());
                }

                manifest.dependencies().iter().try_for_each(|dep| builder.add(resolver.load_external(dep)));
            },
            // No `mu.toml` in the workspace root, we simply add the standard library
            None => {
                builder.add(resolver.load_std());
            }
        };
        // builder.exit();
        */

        // Load all sources as MIRs in the project tree
        /*
        let source_mirs = match self.resolver.mode() {
            // Load the lib.mu file
            ResolveMode::Lib => {
                self.resolver.load_workspace_files_from_lib_mu()
            }
            // Load some source file in the workspace.
            ResolveMode::SourceFile(source_path) => {
                self.resolver.load_workspace_files(source_path)
            }
        };

        source_mirs.iter().for_each(|mir| builder.add(source_mir));
        */

        // The final tree should now look like this:
        // Workspace # The workspace root node
        // ├── mu # The `mu` node containing all external dependencies (already resolved)
        // |   ├── std
        // |   └── ... # Any other external dependency
        // ├── use ::mu::* #   wildcard to include everything from mu by default.
        // ├── use std::geo2d::Circle; # Optional default alias
        // ├── foo # Loaded from `foo.mu`
        // |   └── bar # Loaded from `foo/bar.mu`
        // ├── baz # Loaded from `baz.mu`
        // ├── ... # Any file in the workspace directory tree

        // Ok(builder.build())
    }
}

pub fn resolve(_resolver: Box<dyn Resolver>) -> CompilationResult<microcad_package::SymbolTree> {
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
