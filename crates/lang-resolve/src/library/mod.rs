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
    tree::{self, adopt_tree_to_arena},
};
use serde::{Deserialize, Serialize};

pub use symbol::*;

use crate::{ResolveContext, ResolveError, ResolveResult, SourceUnit};

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

    pub fn load_source(
        &mut self,
        path: impl AsRef<std::path::Path>,
        context: &mut ResolveContext,
    ) -> ResolveResult<()> {
        let source_unit = SourceUnit::load(path)?;

        match source_unit.ir.fetch_artifact() {
            Some(ir) => {
                tree::adopt_tree_to_arena::<Symbol, microcad_lang_lower::ir::Item>(
                    &mut self.arena,
                    ir.tree.root_id(),
                    ir.tree.arena(),
                );
                Ok(())
            }
            None => todo!("Error handling"),
        }
    }
}
