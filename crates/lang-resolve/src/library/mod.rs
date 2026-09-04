// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod manifest;

mod display;

pub mod symbol;
pub mod symbol_path;
pub mod visitor;

pub use manifest::{Dependency, LibrarySection, Manifest, ManifestError};
use microcad_lang_base::{
    Diagnostic, PushDiag,
    tree::{self, NodeRef, adopt_tree_to_arena},
};
use microcad_lang_lower::ir;
use serde::{Deserialize, Serialize};

pub use symbol::*;

use crate::{ResolveContext, ResolveError, ResolveResult, SourceUnit, locate};

#[derive(Debug, Default, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct LibraryRoot {
    /// Manifest
    manifest: Option<Manifest>,
    /// Optional lib.rs file
    entry: Option<Source>,
}

impl LibraryRoot {
    fn load(_path: impl AsRef<std::path::Path>) -> miette::Result<Self> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Library {
    /// The symbol root.
    pub root: SymbolNodeId,

    /// Symbol node id of `mu` symbol that contains all external dependen
    pub mu: SymbolNodeId,
    pub arena: SymbolArena,
}

impl std::hash::Hash for Library {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.root().descendants().for_each(|node| node.hash(state));
    }
}

impl Library {
    pub fn new(library_root: impl Into<LibraryRoot>) -> Self {
        let mut arena = SymbolArena::default();
        let library_root = library_root.into();

        let root = arena.new_node(Symbol::root(library_root));
        let mu = root.append_value(Symbol::mu(), &mut arena);
        Self { root, mu, arena }
    }

    pub fn name(&self) -> Option<&String> {
        self.library_root()
            .manifest
            .as_ref()
            .map(|manifest| &manifest.library.name)
    }

    pub fn root<'a>(&'a self) -> SymbolNodeRef<'a> {
        SymbolNodeRef::new(self.root, &self.arena)
    }

    pub fn root_mut<'a>(&'a mut self) -> SymbolNodeMut<'a> {
        SymbolNodeMut::new(self.root, &mut self.arena)
    }

    pub fn library_root(&self) -> &LibraryRoot {
        match &self.root().get().def {
            SymbolDef::Root(library_root) => &library_root,
            _ => panic!("This should not happen"),
        }
    }

    pub fn append_symbol(&mut self, symbol: impl Into<Symbol>) -> SymbolNodeId {
        self.root.append_value(symbol.into(), &mut self.arena)
    }

    pub fn _load_source(
        &mut self,
        path: impl AsRef<std::path::Path>,
        parent_id: SymbolNodeId,
        context: &mut ResolveContext,
    ) -> ResolveResult<SymbolNodeId> {
        let source_unit = SourceUnit::load(&path)?;
        let path = path.as_ref().to_path_buf();

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
                let id = self.arena.new_node(symbol);

                // Traverse children recursively
                for child in source_node.children() {
                    let item = child.get();
                    let child_id = match child.get().def {
                        ir::Def::FileModule(_) => self._load_source(
                            locate::file_module_path(&path, item.name())?,
                            id,
                            context,
                        )?,
                        _ => tree::adopt_tree_to_arena(&mut self.arena, child.id, source_arena),
                    };
                    id.append(child_id, &mut self.arena);
                }
                parent_id.append(id, &mut self.arena);

                Ok(id)
            }
            None => Err(Box::new(ResolveError::CompileError { path })),
        }
    }

    pub fn load_source(
        &mut self,
        path: impl AsRef<std::path::Path>,
        context: &mut ResolveContext,
    ) -> ResolveResult<SymbolNodeId> {
        self._load_source(path, self.root, context)
    }
}
