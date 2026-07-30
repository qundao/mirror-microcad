// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad resolved symbol tree (RST).

mod builtin;

use builtin::Builtin;

use crate::tree::{SymbolHandle, SymbolPath};
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use derive_more::From;
use microcad_lang_base::{HashId, Id, Identifier, Refer, SrcRef};
use microcad_lang_proc_macros::Artifact;
use microcad_lang_types::{Type, Value};

use microcad_lang_lower::ir;

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResolvedName {
    Local(Id),
    Symbol(SymbolHandle),
    Error(SymbolPath),
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
pub struct Parameter {
    // pub attr: ParameterAttributes,
    pub doc: DocBlock,
    pub id: Identifier,
    pub ty: Type,
    pub default_value: Value,
    pub src_ref: SrcRef,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct ParameterList {
    pub parameters: Box<[Parameter]>,
    // by_id: HashMap<Id, usize>
}

pub type FunctionExpression = ir::FunctionExpression<ResolvedName>;
pub type FunctionStatement = ir::FunctionStatement<ResolvedName>;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function {
    // pub attr: FunctionAttributes,
    pub parameters: ParameterList,
    pub return_ty: Option<Type>,
    pub statements: Box<[FunctionStatement]>,
}

pub type WorkbenchExpression = ir::WorkbenchExpression<ResolvedName>;
pub type WorkbenchStatement = ir::WorkbenchStatement<ResolvedName>;
pub type WorkbenchKind = microcad_lang_base::element::WorkbenchKind;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]

pub struct InitStatement {
    pub id: Identifier,
    pub ty: Type,
    pub expression: WorkbenchExpression,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Init {
    pub parameters: ParameterList,
    pub ty: Type,
    pub statements: Box<[InitStatement]>,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Workbench {
    //pub attr: WorkbenchAttributes
    /// Workbench kind.
    pub kind: Refer<WorkbenchKind>,

    /// Initializers.
    /// The default initializer and tthe workbench parameters are located in the last init.
    pub inits: Box<[Init]>,

    pub statements: Box<[WorkbenchStatement]>,
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
