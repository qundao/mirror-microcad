// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Load a library

use microcad_lang_base::{HashId, LibraryId, PushDiag, Source, ToCompactString, tree};
use microcad_lang_lower::ir;

use crate::{
    Library, Manifest, ResolveContext, ResolveError, ResolveResult,
    error::ResolveErrorKind,
    library::{Symbol, SymbolArena, SymbolNodeId},
    locate,
    resolve::locals::ResolveVisitor,
};

impl ResolveContext {
    /// Load a single file as library.
    pub fn load_file(&mut self, file_mu: impl AsRef<std::path::Path>) -> ResolveResult<Library> {
        let mut arena = SymbolArena::default();
        let root = arena.new_node(Symbol::root(None));

        let mut lib = Library::new(None, Symbol::root(None));
        if !lib.no_std() {
            lib.add_std();
        }

        self._load_source(&mut lib, &file_mu, Some(root))?;
        Ok(lib)
    }

    /// Load a library from a directory containing a `mu.toml` file.
    pub fn load_library(
        &mut self,
        lib_path: impl AsRef<std::path::Path>,
    ) -> ResolveResult<Library> {
        // Locate `lib.mu` in directory
        let lib_mu = locate::lib_mu_path(&lib_path)?;
        log::debug!("Loading library from {lib_mu:?}");

        // Load manifest and load library
        let mut lib = match Manifest::load(locate::mu_toml_path(lib_path)) {
            Ok(manifest) => {
                log::debug!("Loaded manifest:\n{manifest}");

                let mut lib = Library::new(Some(manifest), Symbol::root(None));
                let manifest = lib.manifest.clone().unwrap();
                if !lib.no_std() {
                    lib.add_std();
                }

                if let Some(deps) = &manifest.dependencies {
                    for (name, dep) in deps.iter() {
                        let lib_id = self.load_external_library(dep.path.as_ref().unwrap())?;
                        lib.dependencies.insert(name.to_compact_string(), lib_id);
                    }
                }
                lib
            }
            // Error loading manifest or manifest not found, fallback to defaults.
            Err(err) => {
                log::warn!("No manifest:\n{err:?}");
                self.push_diag(ResolveError::new(err));
                Library::new(None, Symbol::root(None))
            }
        };
        self._load_source(&mut lib, &lib_mu, None)?;

        let mut locals = ResolveVisitor::new();
        use crate::library::visitor::VisitorMut;
        locals.visit(&mut lib);

        Ok(lib)
    }

    fn load_source(&mut self, path: impl AsRef<std::path::Path>) -> ResolveResult<HashId> {
        let source = Source::load(path)?;
        let id = self.src_cache.insert(source);
        let errors = self
            .src_cache
            .get(id)
            .iter()
            .flat_map(|source_unit| source_unit.errors())
            .collect::<Vec<_>>();

        self.append_diags(errors);

        Ok(id)
    }

    /// Load lib with a path of directory containing a `mu.toml` file
    pub fn load_external_library(
        &mut self,
        lib_path: impl AsRef<std::path::Path>,
    ) -> ResolveResult<LibraryId> {
        let lib = self.load_library(lib_path)?;
        self.lib_cache.insert(lib)
    }

    pub fn _load_source(
        &mut self,
        library: &mut Library,
        path: impl AsRef<std::path::Path>,
        parent_id: Option<SymbolNodeId>,
    ) -> ResolveResult<SymbolNodeId> {
        let path = path.as_ref().to_path_buf();
        let id = self.load_source(&path)?;
        println!("Load source file  {path:?}");

        let ir = self.get_ir(id).cloned();
        match ir {
            // We have a successfully compiled IR, add it to the tree and load file recursively, if necessary.
            Some(ir) => {
                let source_arena = ir.tree.arena();
                let source_node = ir.tree.root();

                // Create matching node in target arena
                let symbol = match &source_node.def {
                    ir::Def::Source(source) => {
                        Symbol::loaded_source_file(source_node.get(), path.clone(), source.clone())
                    }
                    _ => source_node.get().clone().into(),
                };
                let id = if let Some(parent_id) = parent_id {
                    let id = library.arena.new_node(symbol);
                    parent_id.append(id, &mut library.arena);
                    id
                } else {
                    *library.root_mut().get_mut() = symbol;
                    library.root.clone()
                };

                // Traverse children recursively
                for child in source_node.children() {
                    let item = child.get();
                    let child_id = match child.get().def {
                        ir::Def::FileModule(_) => {
                            let path = locate::file_module_path(&path, item.name())?;
                            match self._load_source(library, path, Some(id)) {
                                Ok(id) => id,
                                Err(err) => {
                                    self.push_diag(err);
                                    library.arena.new_node(Symbol::root(None))
                                }
                            }
                        }
                        _ => tree::adopt_tree_to_arena(&mut library.arena, child.id, source_arena),
                    };
                    id.append(child_id, &mut library.arena);
                }

                Ok(id)
            }
            None => Err(ResolveErrorKind::CompileError { path }.into()),
        }
    }

    /*
    /// Load standard library from path.
    ///
    /// Installs the standard library, if it is not installed.
    fn _load_and_install_std(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> ResolveResult<Library> {
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

    pub fn try_load_std(&mut self) -> ResolveResult<LibraryId> {
        let info = microcad_std::StdLib::info();
        let id = info.id();

        // A standard library is already loaded, we can stop here.
        if let Some(_) = self.lib_cache.get(&id) {
            log::debug!("Standard library is already loaded.");
            return Ok(id);
        }

        let paths = self.lib_search_paths.clone();
        let std_lib = paths.iter().find_map(|search_path| {
            let std_lib_path = search_path.join(info.path());
            match self._load_and_install_std(std_lib_path) {
                Ok(std_lib) => Some(std_lib),
                Err(err) => {
                    self.push_diag(err);
                    None
                }
            }
        });

        match std_lib {
            Some(std_lib) => self.lib_cache.insert(std_lib),
            None => {
                todo!("Could not load standard library")
            }
        }
    }*/
}
