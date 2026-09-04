// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad resolved symbol tree (RST).

use microcad_lang_lower::ir;

use std::hash::Hash;

use serde::{Deserialize, Serialize};

use derive_more::From;

pub use microcad_lang_base::{Identifier, SymbolId};

/// Function definitions from IR.
pub mod function {
    use microcad_lang_lower::ir;

    pub use ir::{
        Function, FunctionCall, FunctionExpression, FunctionIf, FunctionSignature,
        FunctionStatement, ReturnStatement, Scope,
    };
    pub type Argument = ir::Argument<FunctionExpression>;
    pub type ArgumentList = ir::ArgumentList<FunctionExpression>;
}

pub use function::{Function, FunctionExpression, FunctionStatement};

/// Workbench definitions from IR.
pub mod workbench {
    use microcad_lang_lower::ir;

    pub use ir::{
        Group, Init, InitStatement, Marker, ModelAttributes, Workbench, WorkbenchCall,
        WorkbenchExpression, WorkbenchIf, WorkbenchKind, WorkbenchSignature, WorkbenchStatement,
    };
    pub type Argument = ir::Argument<WorkbenchExpression>;
    pub type ArgumentList = ir::ArgumentList<WorkbenchExpression>;
}

pub use workbench::{
    ModelAttributes, Workbench, WorkbenchExpression, WorkbenchKind, WorkbenchStatement,
};

pub mod constant {
    use microcad_lang_lower::ir;

    pub use ir::{Constant, ConstantExpression, ConstantValue};

    pub type Argument = ir::Argument<ir::ConstantExpression>;
    pub type ArgumentList = ir::ArgumentList<ir::ConstantExpression>;
}

pub use ir::{
    Alias, Argument, ArgumentList, Call, Constant, ConstantExpression, ConstantValue,
    ExportAttribute, ExprSpec, InlineModule, Meta, Parameter, ParameterList, Path, Source,
    SourceStatement, Visibility, Wildcard,
};

use crate::{
    Library,
    library::{LibraryRoot, symbol_path::SymbolAbsPath},
};

/// Symbol definition
#[derive(Debug, Clone, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum SymbolDef {
    Root(LibraryRoot),

    /// External Library dependency
    Library(Library),

    /// Source file symbol.
    Source(Source),
    /// Inline Module symbol: `mod foo {}`
    InlineModule(InlineModule),
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

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Symbol {
    pub meta: Meta,
    pub doc: Option<ir::DocBlock>,
    pub def: SymbolDef,
}

impl Symbol {
    pub fn root(root: LibraryRoot) -> Self {
        Self {
            meta: Meta {
                vis: Visibility::Public,
                ..Default::default()
            },
            doc: None,
            def: SymbolDef::Root(root),
        }
    }

    pub fn mu() -> Self {
        Self {
            meta: Meta {
                name: Some("mu".into()),
                vis: Visibility::Public,
                ..Default::default()
            },
            doc: None,
            def: SymbolDef::InlineModule(InlineModule {}),
        }
    }

    pub fn library(lib: Library) -> Self {
        Self {
            meta: Meta {
                name: lib.name().map(|name| name.clone().into()), // TODO Check if library actually has a name and how to handle anonymous libraries
                vis: Visibility::Public,
                ..Default::default()
            },
            doc: None,
            def: SymbolDef::Library(lib),
        }
    }
}

pub type SymbolArena = microcad_lang_base::tree::Arena<Symbol>;
pub type SymbolNode = microcad_lang_base::tree::Node<Symbol>;
pub type SymbolNodeRef<'a> = microcad_lang_base::tree::NodeRef<'a, Symbol>;
pub type SymbolNodeMut<'a> = microcad_lang_base::tree::NodeMut<'a, Symbol>;
pub type SymbolNodeId = microcad_lang_base::tree::NodeId;

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
