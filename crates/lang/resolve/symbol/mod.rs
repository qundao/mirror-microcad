// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod iterators;
mod symbol_definition;
mod symbol_info;
mod symbol_inner;
mod symbol_map;
mod symbols;

use indexmap::IndexSet;
pub use iterators::*;
pub use symbol_definition::*;
pub use symbol_info::*;
pub(crate) use symbol_map::*;
pub(crate) use symbols::*;

use symbol_inner::*;

use crate::{
    builtin::*, rc::*, resolve::*, src_ref::*, syntax::*, tree_display::*, ty::*, value::*,
};

/// Symbol
#[derive(Clone)]
pub struct Symbol {
    visibility: std::cell::RefCell<Visibility>,
    src_ref: SrcRef,
    inner: RcMut<SymbolInner>,
}

// creation
impl Symbol {
    /// Create new symbol without children.
    /// # Arguments
    /// - `visibility`: Visibility of the symbol
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
    pub(super) fn new_with_visibility(
        visibility: Visibility,
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
    pub(crate) fn new_builtin(builtin: Builtin) -> Symbol {
        Symbol::new(SymbolDef::Builtin(Rc::new(builtin)), None)
    }

    /// New builtin function as symbol.
    pub fn new_builtin_fn(
        name: &'static str,
        parameters: impl Iterator<Item = (Identifier, ParameterValue)>,
        f: &'static BuiltinFn,
        doc: Option<&'static str>,
    ) -> Symbol {
        Self::new_builtin(Builtin {
            id: Identifier::no_ref(name),
            parameters: parameters.collect(),
            kind: BuiltinKind::Function,
            f,
            doc: doc.map(DocBlock::new_builtin),
        })
    }

    /// Replace inner of a symbol with the inner of another.
    pub(super) fn replace(&mut self, replacement: Symbol) {
        replacement
            .inner
            .borrow()
            .children
            .iter()
            .for_each(|(_, child)| child.inner.borrow_mut().parent = Some(self.clone()));
        self.inner.replace(replacement.inner.take());
    }
}

impl Symbol {
    /// Search all ids which require target mode (e.g. `assert_valid`)
    pub(super) fn search_target_mode_ids(&self) -> IdentifierSet {
        self.riter()
            .filter(|symbol| symbol.is_target_mode())
            .map(|symbol| symbol.id())
            .collect()
    }

    /// Return a list of unused private symbols
    ///
    /// Use this after eval for any useful result.
    pub(crate) fn unused_private(&self) -> Symbols {
        let used_in_module = &mut IndexSet::new();
        let mut symbols: Symbols = self
            .riter()
            .skip(1) // skip root
            .filter(|symbol| {
                if let Some(in_module) = symbol.in_module()
                    && symbol.is_used()
                {
                    used_in_module.insert(in_module);
                }
                symbol.is_unused_private()
            })
            .collect();

        symbols.retain(|symbol| {
            if let Some(in_module) = symbol.in_module() {
                !used_in_module.contains(&in_module)
            } else {
                true
            }
        });
        symbols.sort_by_key(|s| s.full_name());
        symbols
    }

    /// Search a *symbol* by it's *qualified name* **and** within a *symbol* given by name.
    ///
    /// If both are found
    /// # Arguments
    /// - `name`: *qualified name* to search for.
    /// - `within`: Searches in the *symbol* with this name too.
    /// - `target`: What to search for
    pub(crate) fn lookup_within_name(
        &self,
        name: &QualifiedName,
        within: &QualifiedName,
        target: LookupTarget,
    ) -> ResolveResult<Symbol> {
        self.lookup_within(name, &self.search(within, false)?, target)
    }
}

// tree structure
impl Symbol {
    /// Get any child with the given `id`.
    /// # Arguments
    /// - `id`: Anticipated *id* of the possible child.
    pub(super) fn get_child(&self, id: &Identifier) -> Option<Symbol> {
        self.inner.borrow().children.get(id).cloned()
    }

    /// Add a new symbol to children.
    pub(crate) fn add_symbol(&mut self, symbol: Symbol) -> ResolveResult<()> {
        self.insert_symbol(symbol.id(), symbol.clone())
    }

    /// Add a new symbol to children with specific id.
    pub(super) fn insert_symbol(&mut self, id: Identifier, symbol: Symbol) -> ResolveResult<()> {
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

    /// Apply a FnMut for each child.
    pub fn with_children(&self, f: impl FnMut((&Identifier, &Symbol))) {
        self.inner.borrow().children.iter().for_each(f)
    }

    /// Create a vector of cloned children.
    fn public_children(&self, visibility: Visibility, src_ref: SrcRef) -> SymbolMap {
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

    /// True if symbol has any children
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.borrow().children.is_empty()
    }

    /// Get parent symbol.
    pub(crate) fn get_parent(&self) -> Option<Symbol> {
        self.inner.borrow().parent.clone()
    }

    /// Set new parent.
    pub(super) fn set_parent(&mut self, parent: Symbol) {
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
}

// visibility
impl Symbol {
    /// Return `true` if symbol's visibility is private
    pub(super) fn visibility(&self) -> Visibility {
        self.visibility.borrow().clone()
    }

    /// Return `true` if symbol's visibility set to is public.
    pub(super) fn is_public(&self) -> bool {
        matches!(self.visibility(), Visibility::Public)
    }

    pub(super) fn is_deleted(&self) -> bool {
        matches!(self.visibility(), Visibility::Deleted)
    }

    pub(super) fn delete(&self) {
        self.visibility.replace(Visibility::Deleted);
    }

    /// Clone this symbol but give the clone another visibility.
    pub(crate) fn clone_with(&self, visibility: Visibility, src_ref: SrcRef) -> Self {
        Self {
            visibility: std::cell::RefCell::new(visibility),
            src_ref,
            inner: self.inner.clone(),
        }
    }

    pub(crate) fn reset_visibility(&self) {
        self.visibility
            .replace(self.with_def(|def| def.visibility()));
    }
}

// definition dependent
impl Symbol {
    /// Return the internal *id* of this symbol.
    pub(crate) fn id(&self) -> Identifier {
        self.inner.borrow().def.id()
    }

    /// check if a private symbol may be declared within this symbol
    pub(super) fn can_const(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDef::Module(..) | SymbolDef::SourceFile(..) | SymbolDef::Workbench(..)
        )
    }

    /// check if a value on the stack may be declared within this symbol
    pub(super) fn can_value(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDef::Function(..) | SymbolDef::Workbench(..) | SymbolDef::SourceFile(..)
        )
    }

    /// check if a property may be declared within this symbol
    pub(super) fn can_prop(&self) -> bool {
        matches!(self.inner.borrow().def, SymbolDef::Workbench(..))
    }

    fn is_root(&self) -> bool {
        matches!(self.inner.borrow().def, SymbolDef::Root)
    }

    pub(crate) fn is_source(&self) -> bool {
        matches!(self.inner.borrow().def, SymbolDef::SourceFile(..))
    }

    pub(crate) fn is_module(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDef::SourceFile(..) | SymbolDef::Module(..)
        )
    }

    pub(crate) fn is_workbench(&self) -> bool {
        matches!(self.inner.borrow().def, SymbolDef::Workbench(..))
    }

    /// Overwrite any value in this symbol
    pub(crate) fn set_value(&self, new_value: Value) -> ResolveResult<()> {
        let is_a_value = match &mut self.inner.borrow_mut().def {
            SymbolDef::Constant(.., value) => {
                *value = new_value;
                true
            }
            _ => false,
        };
        match is_a_value {
            true => Ok(()),
            false => Err(ResolveError::NotAValue(self.full_name())),
        }
    }

    /// Return file path of top level parent source file.
    pub(super) fn source_path(&self) -> Option<std::path::PathBuf> {
        if let SymbolDef::SourceFile(source_file) = &self.inner.borrow().def {
            return source_file
                .filename()
                .parent()
                .map(|path| path.to_path_buf());
        }
        self.get_parent().and_then(|parent| parent.source_path())
    }

    pub(super) fn is_resolvable(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDef::SourceFile(..)
                | SymbolDef::Module(..)
                | SymbolDef::Workbench(..)
                | SymbolDef::UseAll(..)
                | SymbolDef::Alias(..)
        ) && !self.is_deleted()
    }

    pub(super) fn is_link(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDef::UseAll(..) | SymbolDef::Alias(..)
        )
    }

    pub(super) fn is_alias(&self) -> bool {
        matches!(self.inner.borrow().def, SymbolDef::Alias(..))
    }

    pub(super) fn get_link(&self) -> Option<QualifiedName> {
        self.with_def(|def| match def {
            SymbolDef::UseAll(_, name) | SymbolDef::Alias(.., name) => Some(name.clone()),
            _ => None,
        })
    }

    pub(super) fn has_links(&self) -> bool {
        if self.is_link() {
            true
        } else {
            self.inner
                .borrow()
                .children
                .values()
                .filter(|symbol| !symbol.is_deleted())
                .any(|symbol| symbol.has_links())
        }
    }

    /// Work with the symbol definition.
    pub(crate) fn with_def<T>(&self, mut f: impl FnMut(&SymbolDef) -> T) -> T {
        f(&self.inner.borrow().def)
    }

    /// Work with the mutable symbol definition.
    pub(crate) fn with_def_mut<T>(&self, mut f: impl FnMut(&mut SymbolDef) -> T) -> T {
        f(&mut self.inner.borrow_mut().def)
    }
}

// check
impl Symbol {
    /// Mark this symbol as *checked*.
    pub(super) fn set_check(&self) {
        let _ = self.inner.borrow().checked.set(());
    }

    pub(super) fn is_checked(&self) -> bool {
        self.inner.borrow().checked.get().is_some()
    }

    /// check names in symbol definition
    pub(super) fn check(
        &self,
        context: &mut ResolveContext,
        exclude_ids: &IdentifierSet,
    ) -> ResolveResult<()> {
        if !matches!(self.visibility.take(), Visibility::Deleted) {
            // get names of symbol definitions
            let names = match &self.inner.borrow().def {
                SymbolDef::SourceFile(sf) => sf.names(),
                SymbolDef::Module(m) => m.names(),
                SymbolDef::Workbench(wb) => wb.names(),
                SymbolDef::Function(f) => f.names(),
                SymbolDef::Assignment(a) => a.names(),
                SymbolDef::Alias(_, _, name) | SymbolDef::UseAll(_, name) => {
                    log::error!("Resolve Context:\n{context:?}");
                    return Err(ResolveError::ResolveCheckFailed(name.src_ref()));
                }
                _ => Default::default(),
            };

            if !names.is_empty() {
                log::debug!("checking symbols:\n{names:?}");
                // lookup names
                names
                    .iter()
                    .filter(|name| {
                        exclude_ids.contains(name.last().expect("symbol with empty name"))
                    })
                    // search in symbol table
                    .try_for_each(|name| {
                        match context.root.lookup(name, LookupTarget::Any) {
                            Ok(_) => Ok::<_, ResolveError>(()),
                            Err(err) => {
                                // get name of current module
                                let module = match context.root.search(&self.module_name(), false) {
                                    Ok(module) => module,
                                    Err(err) => {
                                        context.error(&self.id(), err)?;
                                        return Ok(());
                                    }
                                };
                                // search within current module
                                if context
                                    .root
                                    .lookup_within(name, &module, LookupTarget::Module)
                                    .is_err()
                                {
                                    context.error(name, err)?;
                                }
                                Ok(())
                            }
                        }
                    })?;
            }

            // check children
            let children = self.inner.borrow().children.clone();
            children
                .values()
                .try_for_each(|symbol| symbol.check(context, exclude_ids))
        } else {
            Ok(())
        }
    }

    fn module_name(&self) -> QualifiedName {
        match self.is_module() {
            true => {
                if let Some(parent) = &self.get_parent() {
                    parent.module_name().with_suffix(&self.id())
                } else {
                    QualifiedName::from_id(self.id())
                }
            }
            false => {
                if let Some(parent) = &self.get_parent() {
                    parent.module_name()
                } else {
                    unreachable!("root must be source file")
                }
            }
        }
    }

    pub(crate) fn kind_str(&self) -> String {
        self.inner.borrow().def.kind_str()
    }

    pub(super) fn source_hash(&self) -> u64 {
        self.inner.borrow().def.source_hash()
    }
}

impl Symbol {
    pub(crate) fn is_used(&self) -> bool {
        self.inner.borrow().used.get().is_some()
    }

    /// Mark this symbol as *used*.
    pub(crate) fn set_used(&self) {
        let _ = self.inner.borrow().used.set(());
    }

    pub(crate) fn is_unused_private(&self) -> bool {
        !self.is_used() && !self.is_public() && !self.is_deleted()
    }

    pub(crate) fn in_module(&self) -> Option<QualifiedName> {
        if let Visibility::PrivateUse(module) = self.visibility() {
            Some(module.clone())
        } else {
            None
        }
    }

    /// Resolve aliases and use statements in this symbol.
    pub(super) fn resolve(&self, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        log::trace!("resolving: {self}");

        // retrieve symbols from any use statements
        let mut from_self = {
            let inner = self.inner.borrow();
            match &inner.def {
                SymbolDef::Alias(visibility, id, name) => {
                    log::trace!("resolving use (as): {self} => {visibility}{id} ({name})");
                    let symbol = context
                        .root
                        .lookup_within_opt(name, &inner.parent, LookupTarget::Any)?
                        .clone_with(visibility.clone(), name.src_ref.clone());
                    self.delete();
                    [(id.clone(), symbol)].into_iter().collect()
                }
                SymbolDef::UseAll(visibility, name) => {
                    let visibility = &if matches!(visibility, &Visibility::Private) {
                        Visibility::PrivateUse(name.clone())
                    } else {
                        visibility.clone()
                    };
                    log::trace!("resolving use all: {self} => {visibility}{name}");
                    let symbols = context
                        .root
                        .lookup_within_opt(name, &inner.parent, LookupTarget::Any)?
                        .public_children(visibility.clone(), name.src_ref.clone());
                    if !symbols.is_empty() {
                        self.delete();
                    }
                    symbols
                }
                // skip others
                _ => SymbolMap::default(),
            }
        };

        let resolved = from_self.resolve_all(context)?;
        from_self.extend(resolved.iter().map(|(k, v)| (k.clone(), v.clone())));
        // collect symbols resolved from children
        let from_children = self.inner.borrow().children.resolve_all(context)?;
        self.inner
            .borrow_mut()
            .children
            .extend(from_children.iter().map(|(k, v)| (k.clone(), v.clone())));
        // return symbols collected from self
        Ok(from_self)
    }

    /// Returns `true` if builtin symbol uses parameter of type Name
    ///
    /// (for assert_valid() and assert_invalid())
    pub(crate) fn is_target_mode(&self) -> bool {
        self.with_def(|def| match def {
            SymbolDef::Builtin(builtin) => builtin
                .parameters
                .values()
                .any(|param| param.ty() == Type::Target),
            _ => false,
        })
    }

    /// Search down the symbol tree for a qualified name.
    /// # Arguments
    /// - `name`: Name to search for.
    pub(crate) fn search(&self, name: &QualifiedName, respect: bool) -> ResolveResult<Symbol> {
        log::trace!("Searching {name} in {:?}", self.full_name());
        if let Some(id) = name.first() {
            if id.is_super() {
                if let Some(parent) = self.get_parent() {
                    return parent.search(&name[1..].iter().cloned().collect(), respect);
                }
            }
        }
        self.search_inner(name, true, respect)
    }

    fn search_inner(
        &self,
        name: &QualifiedName,
        top_level: bool,
        respect: bool,
    ) -> ResolveResult<Symbol> {
        if let Some(first) = name.first() {
            if let Some(child) = self.get_child(first) {
                if respect && !top_level && !child.is_public() {
                    log::trace!("Symbol {:?} is private", child.full_name());
                    Err(ResolveError::SymbolIsPrivate(child.full_name().clone()))
                } else if name.is_single_identifier() && !child.is_deleted() {
                    log::trace!("Found {name:?} in {:?}", self.full_name());
                    self.set_used();
                    Ok(child.clone())
                } else {
                    let name = &name.remove_first();
                    child.search_inner(name, false, respect)
                }
            } else {
                log::trace!("No child in {:?} while searching for {name:?}", self.id());
                Err(ResolveError::SymbolNotFound(name.clone()))
            }
        } else {
            log::warn!("Cannot search for an anonymous name");
            Err(ResolveError::SymbolNotFound(name.clone()))
        }
    }

    /// Print out symbols from that point.
    /// # Arguments
    /// - `f`: Output formatter
    /// - `id`: Overwrite symbol's internal `id` with this one if given (e.g. when using in a map).
    /// - `state`: TreeState
    pub(super) fn print_symbol(
        &self,
        f: &mut impl std::fmt::Write,
        id: Option<&Identifier>,
        state: TreeState,
        children: bool,
    ) -> std::fmt::Result {
        let self_id = &self.id();
        let id = id.unwrap_or(self_id);
        let def = &self.inner.borrow().def;
        let full_name = self.full_name();
        let visibility = self.visibility();
        let hash = self.source_hash();
        let depth = state.depth;
        if state.debug && cfg!(feature = "ansi-color") {
            let checked = if self.is_checked() { " ✓" } else { "" };
            if self.is_used() {
                write!(
                    f,
                    "{:depth$}{visibility:?}{id:?} {def:?} [{full_name:?}] #{hash:#x}{checked}",
                    "",
                )?;
            } else {
                color_print::cwrite!(
                    f,
                    "{:depth$}<#606060>{visibility:?}{id:?} {def:?} [{full_name:?}] #{hash:#x}</>{checked}",
                    "",
                )?;
            }
        } else {
            write!(f, "{:depth$}{id} {def} [{full_name}]", "",)?;
        }
        if children {
            writeln!(f)?;
            self.try_children(|(id, child)| {
                child.print_symbol(f, Some(id), state.indented(), true)
            })?;
        }
        Ok(())
    }

    pub(super) fn set_src_ref(&mut self, src_ref: SrcRef) {
        self.src_ref = src_ref;
    }
}

impl FullyQualify for Symbol {
    /// Get fully qualified name.
    fn full_name(&self) -> QualifiedName {
        let id = self.id();
        match &self.get_parent() {
            Some(parent) => {
                let mut name = parent.full_name();
                name.push(id);
                name
            }

            None => {
                let src_ref = id.src_ref();
                QualifiedName::new(vec![id], src_ref)
            }
        }
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
            src_ref: SrcRef(None),
            visibility: std::cell::RefCell::new(Visibility::default()),
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

impl Info for Symbol {
    fn info(&self) -> SymbolInfo {
        self.with_def(|def| def.info())
    }
}

impl TreeDisplay for Symbol {
    fn tree_print(&self, f: &mut std::fmt::Formatter, state: TreeState) -> std::fmt::Result {
        if self.is_root() {
            self.try_children(|(_, symbol)| symbol.tree_print(f, TreeState::new_debug(1)))
        } else {
            self.print_symbol(f, Some(&self.id()), state, true)
        }
    }
}

#[test]
fn test_symbol_resolve() {
    let root = SourceFile::load_from_str(
        Some("root"),
        "",
        "
        use my; 
        x = my::target;

        use my::target; 
        x = target;
        ",
    )
    .expect("parse error");

    let my = SourceFile::load_from_str(
        Some("my"),
        "",
        "
        pub const target = 1;
        ",
    )
    .expect("parse error");

    let mut context =
        ResolveContext::test_create(root, ResolveMode::Symbolized).expect("resolve error");
    context.test_add_file(my);
    log::trace!("{context:?}");
    context.resolve().expect("resolve error");
}

impl Lookup for Symbol {
    /// Lookup a symbol from global symbols.
    fn lookup(&self, name: &QualifiedName, target: LookupTarget) -> ResolveResult<Symbol> {
        log::trace!(
            "{lookup} for global symbol '{name:?}'",
            lookup = crate::mark!(LOOKUP)
        );
        self.deny_super(name)?;

        let symbol = match self.search(name, true) {
            Ok(symbol) => {
                if target.matches(&symbol) {
                    symbol
                } else {
                    log::trace!(
                        "{not_found} global symbol: {name:?}",
                        not_found = crate::mark!(NOT_FOUND),
                    );
                    return Err(ResolveError::WrongTarget);
                }
            }
            Err(err) => {
                log::trace!(
                    "{not_found} global symbol: {name:?}",
                    not_found = crate::mark!(NOT_FOUND),
                );
                return Err(err)?;
            }
        };
        symbol.set_check();
        log::trace!(
            "{found} global symbol: {symbol:?}",
            found = crate::mark!(FOUND),
        );
        Ok(symbol)
    }

    fn ambiguity_error(ambiguous: QualifiedName, others: QualifiedNames) -> ResolveError {
        ResolveError::AmbiguousSymbol(ambiguous, others)
    }
}
