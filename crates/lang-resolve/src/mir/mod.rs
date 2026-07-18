// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mid-level intermediate representation (MIR).

use derive_more::From;
use microcad_lang_base::{HashId, Refer};
use microcad_lang_proc_macros::Artifact;
use serde::{Deserialize, Serialize};

use microcad_lang_lower::ir;

pub use crate::tree::SymbolMetadata;

pub type Type = ir::Type;
pub type TypeAnnotation = Option<ir::TypeAnnotation>;

pub type UnresolvedName = ir::QualifiedName;

pub type DocBlock = crate::tree::symbol::meta::DocBlock;

pub type ConstantExpression = ir::ConstantExpression<UnresolvedName>;

pub use ir::{Attributes, FunctionSignature, Identifier, Parameter, ParameterList, Visibility};

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Constant {
    pub attr: Attributes,
    pub ty: TypeAnnotation,
    pub expr: ConstantExpression,
}

pub type FunctionExpression = ir::FunctionExpression<UnresolvedName>;
pub type FunctionStatement = ir::FunctionStatement<UnresolvedName>;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub attr: Attributes,
    pub parameters: ParameterList,
    pub return_ty: TypeAnnotation,
    pub statements: Box<[FunctionStatement]>,
}

pub type WorkbenchExpression = ir::WorkbenchExpression<UnresolvedName>;
pub type WorkbenchStatement = ir::WorkbenchStatement<UnresolvedName>;
pub type WorkbenchKind = ir::WorkbenchKind;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]

pub struct InitStatement {
    pub id: Identifier,
    pub ty: TypeAnnotation,
    pub expression: WorkbenchExpression,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Init {
    pub attr: Attributes,
    pub parameters: ParameterList,
    pub statements: Box<[InitStatement]>,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Workbench {
    //pub attr: WorkbenchAttributes
    /// Workbench kind.
    pub kind: Refer<WorkbenchKind>,

    pub parameters: ParameterList,

    /// Initializers.
    pub inits: Box<[Init]>,

    /// Statements.
    pub statements: Box<[WorkbenchStatement]>,
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alias(pub UnresolvedName);

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wildcard(pub UnresolvedName);

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceFile {
    //attr: SourceFileAttributes,
    pub statements: Box<[WorkbenchStatement]>,
}

/// Symbol definition
#[derive(Debug, Hash, From, PartialEq, Serialize, Deserialize)]
pub enum UnresolvedSymbolDef {
    /// Source file symbol.
    SourceFile(SourceFile),
    /// Inline Module symbol: `mod foo {}`
    InlineModule,
    /// File Module Symbol: `mod foo;`
    FileModule,
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

pub type UnresolvedSymbol = crate::tree::Symbol<UnresolvedSymbolDef>;

pub type UnresolvedSymbolRef<'mir> = crate::tree::SymbolRef<'mir, UnresolvedSymbolDef>;

pub type UnresolvedSymbolTree = crate::tree::SymbolTree<UnresolvedSymbolDef>;

#[derive(Debug, PartialEq, Artifact, Serialize, Deserialize)]
pub struct Mir {
    pub input_hash: HashId,
    pub output_hash: HashId,
    pub tree: UnresolvedSymbolTree,
}
