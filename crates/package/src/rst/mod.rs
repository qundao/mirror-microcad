// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad resolved symbol tree (RST).

pub mod function;
pub mod workbench;

mod parameter;

pub use parameter::{Parameter, ParameterList};

pub use function::{Function, FunctionExpression, FunctionStatement};
pub use workbench::{Workbench, WorkbenchExpression, WorkbenchKind, WorkbenchStatement};

pub use microcad_lang_base::{Identifier, SymbolId};

use crate::tree::SymbolHandle;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use derive_more::From;
use microcad_lang_base::{HashId, Refer};
use microcad_lang_proc_macros::Artifact;
use microcad_lang_types::Value;

pub use microcad_lang_lower::ir::Path;

#[derive(Debug, Default, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocBlock(pub Refer<String>);

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Constant {
    pub doc: DocBlock,
    //attr: ConstantAttributes,
    pub value: Refer<Value>,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Alias(SymbolHandle);

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Wildcard(SymbolHandle);

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceFile {
    //attr: SourceFileAttributes,
    statements: Box<[WorkbenchStatement]>,
}

/// Symbol definition
#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResolvedSymbolDef {
    /// Source file symbol.
    SourceFile(SourceFile),
    /// Inline Module symbol: `mod foo {}`
    InlineModule,
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

pub type ResolvedSymbol = crate::tree::Symbol<ResolvedSymbolDef>;

pub type ResolvedSymbolRef<'rst> = crate::tree::SymbolRef<'rst, ResolvedSymbolDef>;

pub type ResolvedSymbolTree = crate::tree::SymbolTree<ResolvedSymbolDef>;

#[derive(Debug, From, Hash, PartialEq, Serialize, Artifact)]
pub struct Rst {
    pub input_hash: HashId,
    pub output_hash: HashId,
    pub tree: ResolvedSymbolTree,
}
