// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad resolved symbol tree (RST).

mod builtin;

pub mod function;
pub mod workbench;

mod parameter;

pub use parameter::{Parameter, ParameterList};

pub use function::{Function, FunctionExpression, FunctionStatement};
pub use workbench::{Workbench, WorkbenchExpression, WorkbenchKind, WorkbenchStatement};

pub use microcad_lang_base::Identifier;

use builtin::Builtin;

use crate::tree::{SymbolHandle, SymbolPath};
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use derive_more::From;
use microcad_lang_base::{HashId, Refer, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Artifact;
use microcad_lang_types::Value;

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResolvedName {
    /// Name for local variable in a function or workbench.
    Local(Identifier),
    /// Name for a builtin method.
    Method(Identifier),

    Symbol(Refer<SymbolHandle>),
    Error(SymbolPath),
}

impl SrcReferrer for ResolvedName {
    fn src_ref(&self) -> SrcRef {
        match self {
            ResolvedName::Local(identifier) => identifier.src_ref(),
            ResolvedName::Method(identifier) => identifier.src_ref(),
            ResolvedName::Symbol(refer) => refer.src_ref(),
            ResolvedName::Error(symbol_path) => symbol_path.src_ref(),
        }
    }
}

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
    /*/// Workspace root
    Root(Workspace),
    /// External dependency
    External(External),
    */
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
    /// Builtin symbol.
    Builtin(Builtin),
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
