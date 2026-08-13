use serde::{Deserialize, Serialize};

use crate::symbol::SymbolDef;

use super::{SymbolTree, iterators};

pub use microcad_lang_base::{Identifier, Name};

pub use microcad_lang_lower::ir::Path;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SymbolHandle(pub(super) usize);

impl SymbolHandle {
    pub fn root() -> Self {
        SymbolHandle(0)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct SymbolIndex {
    pub items: Vec<SymbolHandle>,
}

#[derive(Debug)]
pub struct SymbolRef<'pkg> {
    symbol: &'pkg Symbol,
    tree: &'pkg SymbolTree,
    handle: SymbolHandle,
}

impl<'tree> Clone for SymbolRef<'tree> {
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol,
            tree: self.tree,
            handle: self.handle,
        }
    }
}

impl<'pkg> Copy for SymbolRef<'pkg> {}

impl<'pkg> std::ops::Deref for SymbolRef<'pkg> {
    type Target = Symbol;

    fn deref(&self) -> &Self::Target {
        self.symbol
    }
}

impl<'tree, DEF: Serialize> SymbolRef<'tree, DEF> {
    pub fn new(
        symbol: &'tree Symbol<DEF>,
        tree: &'tree SymbolTree<DEF>,
        handle: SymbolHandle,
    ) -> Self {
        Self {
            symbol,
            tree,
            handle,
        }
    }

    pub fn id(&self) -> Option<&Identifier> {
        self.symbol.meta_data.id.as_ref()
    }

    pub fn symbol(&self) -> &'tree Symbol<DEF> {
        self.symbol
    }

    pub fn tree(&self) -> &'tree SymbolTree<DEF> {
        self.tree
    }

    pub fn handle(&self) -> SymbolHandle {
        self.handle
    }

    pub fn data(&self) -> &'tree SymbolMetadata {
        &self.symbol.meta_data
    }

    pub fn children(&self) -> iterators::Children<'tree, DEF> {
        iterators::Children::new(*self)
    }

    pub fn descendants(&self) -> iterators::Descendants<'tree, DEF> {
        iterators::Descendants::new(*self)
    }
}
