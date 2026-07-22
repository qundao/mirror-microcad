use serde::{Deserialize, Serialize};

use super::{SymbolTree, iterators};

pub use microcad_lang_base::{Id, Identifier};

pub use microcad_lang_lower::ir::SymbolPath;

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

impl SymbolIndex {
    pub fn insert(&mut self, hash: SymbolHandle) {
        self.items.push(hash);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&SymbolHandle> {
        self.items.get(index)
    }

    pub fn get_by_id<'tree, DEF: Serialize>(
        &self,
        tree: &'tree SymbolTree<DEF>,
        id: Id,
    ) -> Option<SymbolRef<'tree, DEF>> {
        self.items
            .iter()
            .filter_map(|hash| tree.get(*hash))
            .find(|symbol| match symbol.id() {
                Some(symbol_id) => &symbol_id.id() == &id,
                None => false,
            })
    }

    pub fn refs<'tree, DATA: Serialize>(
        &self,
        tree: &'tree SymbolTree<DATA>,
    ) -> impl Iterator<Item = SymbolRef<'tree, DATA>> {
        self.items.iter().filter_map(|hash| tree.get(*hash))
    }
}

impl FromIterator<SymbolHandle> for SymbolIndex {
    fn from_iter<T: IntoIterator<Item = SymbolHandle>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

pub mod meta {
    use microcad_lang_base::Refer;

    use microcad_lang_lower::ir;

    pub use microcad_lang_base::SrcRef;
    use serde::{Deserialize, Serialize};

    pub type Visibility = ir::Visibility;
}

/// Symbol content
#[derive(Debug, Default, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolMetadata {
    pub id: Option<Identifier>,

    /// Visibility
    pub visibility: meta::Visibility,

    /// Source code reference of the symbol definition
    pub src_ref: meta::SrcRef,

    /// Source code reference of symbol's keyword
    pub keyword_src_ref: meta::SrcRef,
}

#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]

pub struct Symbol<DEF: Serialize> {
    pub meta_data: SymbolMetadata,
    pub def: DEF,
    pub parent: Option<SymbolHandle>,
    pub children: SymbolIndex,
}

impl<DEF: Serialize> Symbol<DEF> {
    pub fn new(meta_data: impl Into<SymbolMetadata>, def: impl Into<DEF>) -> Self {
        Self {
            meta_data: meta_data.into(),
            def: def.into(),
            parent: None,
            children: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolRef<'tree, DEF: Serialize>
where
    DEF: 'tree,
{
    symbol: &'tree Symbol<DEF>,
    tree: &'tree SymbolTree<DEF>,
    handle: SymbolHandle,
}

impl<'tree, DEF: Serialize> Clone for SymbolRef<'tree, DEF> {
    fn clone(&self) -> Self {
        Self {
            symbol: self.symbol,
            tree: self.tree,
            handle: self.handle,
        }
    }
}

impl<'tree, DEF: Serialize> Copy for SymbolRef<'tree, DEF> {}

impl<'tree, DEF: Serialize> std::ops::Deref for SymbolRef<'tree, DEF> {
    type Target = Symbol<DEF>;

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
