// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod manifest;

mod display;

pub mod symbol;
pub mod symbol_path;
pub mod visitor;

use std::collections::BTreeMap;

pub use manifest::{Dependency, LibrarySection, Manifest, ManifestError};
use microcad_lang_base::{MICROCAD_EXTENSION, Name, PushDiag, ToCompactString, tree};

use microcad_lang_lower::ir;
use serde::{Deserialize, Serialize};

pub use symbol::*;

use crate::{
    ResolveContext, ResolveError, ResolveResult, SourceUnit,
    error::ResolveErrorKind,
    locate::{self, mu_toml_path},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Library {
    pub manifest: Option<Manifest>,

    /// Symbol node id of `mu` symbol that contains all external dependencies
    pub dependencies: BTreeMap<Name, Library>,

    /// The symbol root.
    pub root: SymbolNodeId,

    pub arena: SymbolArena,
}

impl std::hash::Hash for Library {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.root().descendants().for_each(|node| node.hash(state));
    }
}

impl Library {
    /// An empty lib without dependencies or source file.
    pub fn new(manifest: Option<Manifest>, root: impl Into<Symbol>) -> Self {
        let mut arena = SymbolArena::default();
        let root = arena.new_node(root.into());
        Self {
            manifest,
            dependencies: Default::default(),
            root,
            arena,
        }
    }

    /// Load a single file as library with `mu.toml` file.
    pub fn load_file(
        file_mu: impl AsRef<std::path::Path>,
        context: &mut ResolveContext,
    ) -> ResolveResult<Self> {
        let mut arena = SymbolArena::default();
        let root = arena.new_node(Symbol::root(None));

        let mut lib = Self::new(None, Symbol::root(None));
        lib.load_std(context)?;
        lib._load_source(&file_mu, Some(root), context)?;

        Ok(lib)
    }

    /// Load lib with a path of directory containing a `mu.toml` file
    pub fn load(
        lib_path: impl AsRef<std::path::Path>,
        context: &mut ResolveContext,
    ) -> ResolveResult<Self> {
        // Locate `lib.mu` in directory
        let lib_mu = locate::lib_mu_path(&lib_path)?;
        println!("Loading library from {lib_mu:?}");

        // Load manifest and load library
        let mut lib = match Manifest::load(mu_toml_path(lib_path)) {
            Ok(manifest) => {
                println!("Loaded manifest:\n{manifest}");

                let mut lib = Self::new(Some(manifest), Symbol::root(None));
                let manifest = lib.manifest.clone().unwrap();

                if !lib.no_std() {
                    lib.load_std(context)?;
                }

                if let Some(deps) = &manifest.dependencies {
                    for (name, dep) in deps.iter() {
                        lib.add_dependency(name, dep.path.as_ref().unwrap(), context)?;
                    }
                }
                lib
            }
            // Error loading manifest or manifest not found, fallback to defaults.
            Err(err) => {
                println!("No manifest:\n{err:?}");

                context.push_diag(ResolveError::new(err));
                let mut lib = Self::new(None, Symbol::root(None));

                lib.load_std(context)?;
                lib
            }
        };

        lib._load_source(&lib_mu, None, context)?;
        Ok(lib)
    }

    pub fn add_dependency(
        &mut self,
        name: impl ToCompactString,
        path: impl AsRef<std::path::Path>,
        context: &mut ResolveContext,
    ) -> ResolveResult<&Library> {
        let name = name.to_compact_string();
        match self.dependencies.get(&name) {
            Some(_) => {
                todo!("Error: Dependency already exists")
            }
            None => {
                let path = path.as_ref();
                println!("Load dep: {name} = {path:?}");
                self.dependencies
                    .insert(name.clone(), Library::load(path, context)?);
                Ok(self.dependencies.get(&name).expect("A name"))
            }
        }
    }

    pub fn load_std(&mut self, context: &mut ResolveContext) -> ResolveResult<&Library> {
        self.add_dependency(
            "std",
            std::path::Path::new("../..").join(microcad_std::StdLib::default_path()),
            context,
        )
    }

    pub fn no_std(&self) -> bool {
        match &self.manifest {
            Some(manifest) => manifest.library.no_std.unwrap_or(false),
            None => false, // Load std by default
        }
    }

    pub fn name(&self) -> Option<&String> {
        self.manifest
            .as_ref()
            .map(|manifest| &manifest.library.name)
    }

    pub fn root<'a>(&'a self) -> SymbolNodeRef<'a> {
        SymbolNodeRef::new(self.root, &self.arena)
    }

    pub fn root_mut<'a>(&'a mut self) -> SymbolNodeMut<'a> {
        SymbolNodeMut::new(self.root, &mut self.arena)
    }

    pub fn append_symbol(&mut self, symbol: impl Into<Symbol>) -> SymbolNodeId {
        self.root.append_value(symbol.into(), &mut self.arena)
    }

    pub fn _load_source(
        &mut self,
        path: impl AsRef<std::path::Path>,
        parent_id: Option<SymbolNodeId>,
        context: &mut ResolveContext,
    ) -> ResolveResult<SymbolNodeId> {
        let source_unit = SourceUnit::load(&path)?;
        let path = path.as_ref().to_path_buf();
        println!("Load source file  {path:?}");

        for err in source_unit.ir.errors() {
            println!("{err}")
        }

        match source_unit.ir.fetch_artifact() {
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
                    let id = self.arena.new_node(symbol);
                    parent_id.append(id, &mut self.arena);
                    id
                } else {
                    *self.root_mut().get_mut() = symbol;
                    self.root.clone()
                };

                // Traverse children recursively
                for child in source_node.children() {
                    let item = child.get();
                    let child_id = match child.get().def {
                        ir::Def::FileModule(_) => {
                            let path = locate::file_module_path(&path, item.name())?;
                            match self._load_source(path, Some(id), context) {
                                Ok(id) => id,
                                Err(err) => {
                                    context.push_diag(err);
                                    self.arena.new_node(Symbol::root(None))
                                }
                            }
                        }
                        _ => tree::adopt_tree_to_arena(&mut self.arena, child.id, source_arena),
                    };
                    id.append(child_id, &mut self.arena);
                }

                Ok(id)
            }
            None => Err(ResolveErrorKind::CompileError { path }.into()),
        }
    }

    pub fn load_source(
        &mut self,
        path: impl AsRef<std::path::Path>,
        context: &mut ResolveContext,
    ) -> ResolveResult<SymbolNodeId> {
        self._load_source(path, Some(self.root), context)
    }
}
