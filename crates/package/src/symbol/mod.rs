// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad resolved symbol tree (RST).

use microcad_lang_lower::ir;

use microcad_lang_lower::ir::Visibility;

pub use workbench::{Workbench, WorkbenchExpression, WorkbenchKind, WorkbenchStatement};

pub use microcad_lang_base::{Identifier, SymbolId};

/// Function definitions from IR.
/// TODO Check if this can be moved to IR eventually.
pub mod function {
    use microcad_lang_lower::ir;

    pub use ir::{Function, FunctionExpression, FunctionSignature, FunctionStatement};
    pub type Call = ir::Call<FunctionExpression>;
    pub type Argument = ir::Argument<FunctionExpression>;
    pub type ArgumentList = ir::ArgumentList<FunctionExpression>;
    pub type If = ir::If<FunctionExpression>;
    pub type Scope = ir::Scope;
    pub type ReturnStatement = ir::ReturnStatement;
}

/// Workbench definitions from IR.
/// TODO Check if this can be moved to IR eventually.
pub mod workbench {
    use microcad_lang_lower::ir;

    pub use ir::{
        Group, Init, InitStatement, Marker, Workbench, WorkbenchExpression, WorkbenchKind,
        WorkbenchSignature, WorkbenchStatement,
    };
    pub type Call = ir::Call<WorkbenchExpression>;
    pub type Argument = ir::Argument<WorkbenchExpression>;
    pub type ArgumentList = ir::ArgumentList<WorkbenchExpression>;
    pub type If = ir::If<WorkbenchExpression>;
}

pub use ir::{Attributes, ConstantValue, Parameter, ParameterList};

pub use function::{Function, FunctionExpression, FunctionStatement};

use std::hash::Hash;

use serde::{Deserialize, Serialize};

use derive_more::From;

pub use microcad_lang_lower::ir::Path;

#[derive(Debug, Default, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineModule;

/// Symbol definition
#[derive(Debug, Clone, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum SymbolDef {
    /// Source file symbol.
    Source(ir::Source),
    /// Inline Module symbol: `mod foo {}`
    InlineModule(InlineModule),
    /// Workbench symbol.
    Workbench(Workbench),
    /// Function symbol.
    Function(Function),
    /// Constant.
    Constant(ir::Constant),
    /// Alias of a pub use statement.
    Alias(ir::Alias),
    /// Use all available symbols in the module with the given name.
    Wildcard(ir::Wildcard),
}

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    pub meta: ir::Meta,
    pub doc: Option<ir::DocBlock>,
    pub def: SymbolDef,
}

pub type SymbolArena = microcad_lang_base::tree::Arena<Symbol>;
pub type SymbolNode = microcad_lang_base::tree::Node<Symbol>;
pub type SymbolNodeRef<'a> = microcad_lang_base::tree::NodeRef<'a, Symbol>;
pub type SymbolNodeMut<'a> = microcad_lang_base::tree::NodeMut<'a, Symbol>;
pub type SymbolNodeId = microcad_lang_base::tree::NodeId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolTree {
    /// The symbol root.
    pub root: SymbolNodeId,

    /// Symbol node id of `mu` symbol that contains all external dependen
    pub mu: SymbolNodeId,

    pub arena: SymbolArena,
}

pub struct SymbolAbsPath {
    pub parts: Vec<Identifier>,
}

impl SymbolAbsPath {
    /// Returns an iterator over references to the path components (from root to leaf).
    pub fn iter(&self) -> std::slice::Iter<'_, Identifier> {
        self.parts.iter()
    }
}

// 1. Enables: for part in &abs_path { ... }
impl<'a> IntoIterator for &'a SymbolAbsPath {
    type Item = &'a Identifier;
    type IntoIter = std::slice::Iter<'a, Identifier>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// 2. Enables: for part in abs_path { ... } (takes ownership)
impl IntoIterator for SymbolAbsPath {
    type Item = Identifier;
    type IntoIter = std::vec::IntoIter<Identifier>;

    fn into_iter(self) -> Self::IntoIter {
        self.parts.into_iter()
    }
}

// 3. Enables indexing: abs_path[0]
impl std::ops::Index<usize> for SymbolAbsPath {
    type Output = Identifier;

    fn index(&self, index: usize) -> &Self::Output {
        &self.parts[index]
    }
}

/// Extension trait for [`SymbolNode`] .
pub trait SymbolNodeExt {
    fn name(&self) -> Option<&Identifier>;

    fn is_public(&self) -> bool;

    /// Absolute path of this symbol, starting with the package root symbol.
    fn abs_path(&self) -> SymbolAbsPath;

    fn def(&self) -> &SymbolDef;

    /// Get documentation string for this symbol.
    /// The string only contains
    fn doc(&self) -> Option<&String>;
}

impl<'a> SymbolNodeExt for SymbolNodeRef<'a> {
    fn name(&self) -> Option<&Identifier> {
        self.meta.name.as_ref()
    }

    fn is_public(&self) -> bool {
        self.meta.vis == Visibility::Public
    }

    fn abs_path(&self) -> SymbolAbsPath {
        let mut parts: Vec<_> = self
            .ancestors()
            .filter_map(|symbol| symbol.name().cloned())
            .collect();

        // Ancestors walk leaf -> root; reverse to get root -> leaf path
        parts.reverse();

        SymbolAbsPath { parts }
    }

    fn def(&self) -> &SymbolDef {
        &self.get().def
    }

    fn doc(&self) -> Option<&String> {
        self.get().doc.as_ref().map(|doc| &doc.content)
    }
}
