// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad symbol tree.

mod def;
mod iterators;
mod symbol_definition;
mod symbol_inner;
mod symbol_map;
mod symbols;

use indexmap::IndexSet;
use microcad_lang_base::{RcMut, SrcRef, SrcReferrer, TreeDisplay, TreeState};

pub use iterators::*;
pub use symbol_definition::*;
pub(crate) use symbol_map::*;
pub(crate) use symbols::*;

use symbol_inner::*;

/// Symbol
#[derive(Clone)]
pub struct Symbol {
    visibility: std::cell::RefCell<ir::Visibility>,
    src_ref: SrcRef,
    inner: RcMut<SymbolInner>,
}

// creation
impl Symbol {
    /// Create new symbol without children.
    /// # Arguments
    /// - `def`: Symbol definition
    /// - `parent`: Symbol's parent symbol or none for root
    pub(crate) fn new(def: SymbolDef, parent: Option<Symbol>) -> Self {
        Symbol {
            visibility: std::cell::RefCell::new(def.visibility()),
            inner: RcMut::new(SymbolInner {
                def,
                parent,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    /// Create new symbol without children.
    /// # Arguments
    /// - `visibility`: Visibility of the symbol
    /// - `def`: Symbol definition
    /// - `parent`: Symbol's parent symbol or none for root
    pub(crate) fn new_with_visibility(
        visibility: ir::Visibility,
        def: SymbolDef,
        parent: Option<Symbol>,
    ) -> Self {
        Symbol {
            visibility: std::cell::RefCell::new(visibility),
            inner: RcMut::new(SymbolInner {
                def,
                parent,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    /// Create a symbol node for a built-in.
    /// # Arguments
    /// - `id`: Name of the symbol
    /// - `parameters`: Optional parameter list
    /// - `f`: The builtin function
    pub(crate) fn new_builtin(builtin: impl Into<Builtin>) -> Symbol {
        Symbol::new(SymbolDef::Builtin(builtin.into()), None)
    }

    /// New builtin function as symbol.
    pub fn new_builtin_fn(
        name: &'static str,
        parameters: impl Iterator<Item = (Identifier, ParameterValue)>,
        f: &'static BuiltinFn,
        doc: Option<&'static str>,
    ) -> Symbol {
        Self::new_builtin(BuiltinFunction {
            id: Identifier::no_ref(name),
            parameters: parameters.collect(),
            f,
            doc: doc.map(ir::DocBlock::new_builtin),
        })
    }

    /// Get fully qualified name.
    pub fn full_name(&self) -> ir::QualifiedName {
        let id = self.id();
        match &self.get_parent() {
            Some(parent) => {
                let mut name = parent.full_name();
                name.push(id);
                name
            }

            None => {
                let src_ref = id.src_ref();
                ir::QualifiedName::new(vec![id], src_ref)
            }
        }
    }
}

// tree structure
impl Symbol {
    /// Get any child with the given `id`.
    /// # Arguments
    /// - `id`: Anticipated *id* of the possible child.
    pub fn get_child(&self, id: &Identifier) -> Option<Symbol> {
        self.inner.borrow().children.get(id).cloned()
    }

    /// Add a new symbol to children.
    pub(crate) fn add_symbol(&mut self, symbol: Symbol) -> ResolveResult<()> {
        self.insert_symbol(symbol.id(), symbol.clone())
    }

    /// Add a new symbol to children with specific id.
    pub fn insert_symbol(&mut self, id: Identifier, symbol: Symbol) -> ResolveResult<()> {
        log::trace!("insert symbol: {id}");
        if let Some(symbol) = self.inner.borrow_mut().children.insert(id, symbol.clone()) {
            Err(ResolveError::SymbolAlreadyDefined(symbol.full_name()))
        } else {
            Ok(())
        }
    }

    /// Insert child and change parent of child to new parent.
    /// # Arguments
    /// - `parent`: New parent symbol (will be changed in child!).
    /// - `child`: Child to insert
    pub(crate) fn add_child(parent: &Symbol, child: Symbol) {
        child.inner.borrow_mut().parent = Some(parent.clone());
        let id = child.id();
        parent.inner.borrow_mut().children.insert(id, child);
    }

    /// Initially set children.
    ///
    /// Panics if children already exist.
    pub(super) fn set_children(&self, new_children: SymbolMap) {
        assert!(self.inner.borrow().children.is_empty());
        self.inner.borrow_mut().children = new_children;
    }

    /// Try to apply a FnMut for each child.
    pub(crate) fn try_children<E: std::error::Error>(
        &self,
        f: impl FnMut((&Identifier, &Symbol)) -> Result<(), E>,
    ) -> Result<(), E> {
        self.inner.borrow().children.iter().try_for_each(f)
    }

    /// Try to apply a FnMut for each child.
    pub(crate) fn try_children_sorted<E: std::error::Error>(
        &self,
        f: impl FnMut((&Identifier, &Symbol)) -> Result<(), E>,
    ) -> Result<(), E> {
        let mut children = self.inner.borrow().children.clone();
        children.sort_by(|id1, _, id2, _| id1.cmp(id2));
        children.iter().try_for_each(f)
    }

    /// Apply a FnMut for each child.
    pub fn with_children(&self, f: impl FnMut((&Identifier, &Symbol))) {
        self.inner.borrow().children.iter().for_each(f)
    }

    /// Create a vector of cloned children.
    fn public_children(&self, visibility: ir::Visibility, src_ref: SrcRef) -> SymbolMap {
        let inner = self.inner.borrow();

        inner
            .children
            .values()
            .filter(|symbol| {
                if symbol.is_public() {
                    true
                } else {
                    log::trace!("Skipping private symbol:\n{symbol:?}");
                    false
                }
            })
            .map(|symbol| symbol.clone_with(visibility.clone(), src_ref.clone()))
            .map(|symbol| (symbol.id(), symbol))
            .collect()
    }

    /// Get parent symbol.
    pub(crate) fn get_parent(&self) -> Option<Symbol> {
        self.inner.borrow().parent.clone()
    }

    /// Set new parent.
    pub(crate) fn set_parent(&mut self, parent: Symbol) {
        self.inner.borrow_mut().parent = Some(parent);
    }

    /// Return iterator over symbol's children.
    pub fn iter(&self) -> Children {
        Children::new(self.clone())
    }

    /// Iterate recursively
    pub fn riter(&self) -> RecurseChildren {
        RecurseChildren::new(self.clone())
    }

    /// Get the `SrcRef` for the kind keyword of this symbol, if any
    pub fn kind_ref(&self) -> Option<SrcRef> {
        self.inner.borrow().kind_ref()
    }
}

// visibility
impl Symbol {
    /// Return `true` if symbol's visibility is private
    pub fn visibility(&self) -> ir::Visibility {
        self.visibility.borrow().clone()
    }

    /// Return `true` if symbol's visibility set to is public.
    pub fn is_public(&self) -> bool {
        matches!(self.visibility(), ir::Visibility::Public)
    }

    /// Clone this symbol but give the clone another visibility.
    pub(crate) fn clone_with(&self, visibility: ir::Visibility, src_ref: SrcRef) -> Self {
        Self {
            visibility: std::cell::RefCell::new(visibility),
            src_ref,
            inner: self.inner.clone(),
        }
    }
}

// definition dependent
impl Symbol {
    /// Return the internal *id* of this symbol.
    pub fn id(&self) -> Identifier {
        self.inner.borrow().def.id()
    }

    /// Work with the symbol definition.
    pub fn with_def<T>(&self, mut f: impl FnMut(&SymbolDef) -> T) -> T {
        f(&self.inner.borrow().def)
    }

    /// Work with the mutable symbol definition.
    pub(crate) fn with_def_mut<T>(&self, mut f: impl FnMut(&mut SymbolDef) -> T) -> T {
        f(&mut self.inner.borrow_mut().def)
    }
}

// check
impl Symbol {
    pub fn source_hash(&self) -> u64 {
        self.inner.borrow().def.source_hash()
    }

    /// Print out symbols from that point.
    /// # Arguments
    /// - `f`: Output formatter
    /// - `id`: Overwrite symbol's internal `id` with this one if given (e.g. when using in a map).
    /// - `state`: TreeState
    pub fn print_symbol(
        &self,
        f: &mut impl std::fmt::Write,
        id: Option<&ir::Identifier>,
        state: TreeState,
        children: bool,
    ) -> std::fmt::Result {
        let self_id = &self.id();
        let id = id.unwrap_or(self_id);
        let def = &self.inner.borrow().def;
        let full_name = self.full_name();
        let depth = state.depth;

        write!(f, "{:depth$}{id} {def} [{full_name}]", "",)?;
        if children {
            writeln!(f)?;
            if state.debug {
                self.try_children(|(id, child)| {
                    child.print_symbol(f, Some(id), state.indented(), true)
                })?;
            } else {
                self.try_children_sorted(|(id, child)| {
                    child.print_symbol(f, Some(id), state.indented(), true)
                })?;
            }
        }
        Ok(())
    }
}

impl SrcReferrer for Symbol {
    fn src_ref(&self) -> SrcRef {
        if self.src_ref.is_none() {
            self.inner.borrow().src_ref()
        } else {
            self.src_ref.clone()
        }
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            src_ref: SrcRef::none(),
            visibility: std::cell::RefCell::new(ir::Visibility::default()),
            inner: RcMut::new(Default::default()),
        }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        // just compare the pointers - not the content
        self.inner.as_ptr() == other.inner.as_ptr()
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_symbol(f, None, TreeState::new_display(), false)
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tree_print(f, TreeState::new_debug(0))
    }
}

impl TreeDisplay for Symbol {
    fn tree_print(&self, f: &mut std::fmt::Formatter, state: TreeState) -> std::fmt::Result {
        if self.is_root() {
            if state.debug {
                self.try_children(|(_, symbol)| symbol.tree_print(f, state))
            } else {
                self.try_children_sorted(|(_, symbol)| symbol.tree_print(f, state))
            }
        } else {
            self.print_symbol(f, Some(&self.id()), state, true)
        }
    }
}
