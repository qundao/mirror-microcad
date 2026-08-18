// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad intermediate representation (IR) elements.
//!
//! Every AST element in the µcad language is parsed into an intermediate representation in this module.

pub mod assignment;
pub mod attribute;
pub mod cast_into;
pub mod expression;
pub mod function;
pub mod parameter;
pub mod path;
pub mod workbench;

pub mod desugared;

pub use assignment::*;
pub use attribute::*;
pub use cast_into::*;
pub use expression::*;
pub use function::*;
pub use parameter::*;
pub use path::{Path, UnresolvedPath};
pub use workbench::*;

pub use microcad_lang_base::{Identifier, element::Visibility};
pub use microcad_lang_types::ty::{MatrixType, QuantityType, TupleType, Ty, Unit};

use derive_more::{Deref, Display, From};
use microcad_lang_base::SrcRef;
use serde::{Deserialize, Serialize};

use crate::ir;

#[derive(Debug, Default, Display, Deref, Clone, Hash, PartialEq, Serialize, Deserialize)]
#[display("{}", ty)]
pub struct Type {
    #[deref]
    pub ty: microcad_lang_types::Type,
    pub src_ref: SrcRef,
}

/// Symbol content
#[derive(Debug, Default, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub name: Option<Identifier>,

    /// Visibility
    pub vis: Visibility,

    /// Source code reference of the symbol definition
    pub src_ref: SrcRef,

    /// Source code reference of symbol's keyword
    pub keyword_src_ref: SrcRef,
}

/// A desugared source file.
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// Attributes, combined from Inner and OuterAttributes
    pub attr: ir::Attributes,

    /// Workbench statements
    pub statements: Box<[ir::WorkbenchStatement]>,
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alias {
    /// Attributes, combined from Inner and OuterAttributes
    pub attr: ir::Attributes,

    pub path: Path,
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wildcard {
    /// Attributes, combined from Inner and OuterAttributes
    pub attr: ir::Attributes,

    pub path: Path,
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineModule {
    /// Attributes, combined from Inner and OuterAttributes
    pub attr: ir::Attributes,
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileModule {
    /// Attributes
    pub attr: ir::Attributes,
}

/// A constant definition: `const FOO: Length = 32mm`.
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Constant {
    /// Attributes,
    pub attr: ir::Attributes,
    /// Type of the constant
    pub ty: ir::Type,
    /// Expression type
    pub expr: ir::ConstantExpression,
}

/// IR item definition
#[derive(Debug, Clone, Hash, From, PartialEq, Serialize, Deserialize)]
pub enum Def {
    /// Source file symbol.
    Source(Source),
    /// Inline Module symbol: `mod foo {}`
    InlineModule(InlineModule),
    /// File Module Symbol: `mod foo;`
    FileModule(FileModule),
    /// Workbench symbol.
    Workbench(Workbench),
    /// Function symbol.
    Function(Function),
    /// Constant.
    Constant(Constant),
    /// Alias of a pub use statement.
    Alias(Alias),
    /// Use all available symbols in the module with the given name.
    Wildcard(Wildcard),
}

/// An Ir Item holds a definition and meta data
#[derive(Debug, Clone, Hash, From, PartialEq, Serialize, Deserialize)]
pub struct IrItem {
    /// Item metadata
    pub meta: Meta,
    /// Item definition
    pub def: Def,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tree {
    root: NodeId,
    arena: Arena,
}

impl From<(NodeId, Arena)> for Tree {
    fn from(value: (NodeId, Arena)) -> Self {
        Self {
            root: value.0,
            arena: value.1,
        }
    }
}

impl std::hash::Hash for Tree {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.root().descendants().for_each(|node| node.hash(state));
    }
}

impl Tree {
    pub fn new(root: IrItem) -> Self {
        let mut arena = Arena::default();
        let root = arena.new_node(root);
        Self { root, arena }
    }
    pub fn root<'a>(&'a self) -> NodeRef<'a> {
        NodeRef::new(self.root, &self.arena)
    }
}

pub type Arena = microcad_lang_base::tree::Arena<IrItem>;
pub type Node = microcad_lang_base::tree::Node<IrItem>;
pub type NodeRef<'a> = microcad_lang_base::tree::NodeRef<'a, IrItem>;
pub type NodeMut<'a> = microcad_lang_base::tree::NodeMut<'a, IrItem>;
pub type NodeId = microcad_lang_base::tree::NodeId;
