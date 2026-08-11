// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mid-level intermediate representation (MIR).

use derive_more::From;
use microcad_lang_base::{HashId, Name, Refer, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::Artifact;
use serde::{Deserialize, Serialize};

use microcad_lang_lower::ir;

pub use microcad_package::tree::{SymbolHandle, SymbolMetadata};
use microcad_package::{manifest::Manifest, rst::Rst};

pub type SymbolPath = ir::SymbolPath;
pub type Type = ir::Type;
pub type TypeAnnotation = Option<ir::TypeAnnotation>;

/// The name of a symbol in an unresolved tree
#[derive(Debug, Clone, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum Name {
    /// This name
    Local(Name),
    Symbol(SymbolHandle),
    ToBeResolved(ir::SymbolPath),
}

impl ir::NameSpec for Name {}

impl SrcReferrer for Name {
    fn src_ref(&self) -> SrcRef {
        SrcRef::none()
    }
}

pub type ConstantExpression = ir::ConstantExpression<Name>;

pub type Attributes = ir::Attributes<Name>;

pub type FunctionSignature = ir::FunctionSignature<Name>;
pub type Parameter = ir::Parameter<Name>;
pub type ParameterList = ir::ParameterList<Name>;

pub use ir::{Identifier, Visibility};

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Constant {
    pub attr: Attributes,
    pub ty: TypeAnnotation,
    pub expr: ConstantExpression,
}

pub type FunctionExpression = ir::FunctionExpression<Name>;
pub type FunctionStatement = ir::FunctionStatement<Name>;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub attr: Attributes,
    pub parameters: ParameterList,
    pub return_ty: TypeAnnotation,
    pub statements: Box<[FunctionStatement]>,
}

pub type WorkbenchExpression = ir::WorkbenchExpression<Name>;
pub type WorkbenchStatement = ir::WorkbenchStatement<Name>;
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
    pub statements: Box<[WorkbenchStatement]>,
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
pub struct Alias(pub Name);

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wildcard(pub Name);

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceFile {
    pub attr: Attributes,
    pub statements: Box<[WorkbenchStatement]>,
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineModule {
    pub attr: Attributes,
}

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileModule {
    /// Attributes for this file module
    pub attr: Attributes,
}

#[derive(Debug, Hash, From, PartialEq, Serialize, Deserialize)]
pub struct External {
    //pub rst: Rst,
}

#[derive(Debug, Hash, From, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    name: Option<Name>,
}

impl From<Workspace> for UnresolvedSymbolTree {
    fn from(workspace: Workspace) -> Self {
        UnresolvedSymbol::new(
            SymbolMetadata {
                id: None,
                visibility: Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            workspace,
        )
        .into()
    }
}

/// Symbol definition
#[derive(Debug, Hash, From, PartialEq, Serialize, Deserialize)]
pub enum UnresolvedSymbolDef {
    /// The workspace
    Workspace(Workspace),
    /// The `mu` node
    Externals,

    External(External),

    /// Source file symbol.
    SourceFile(SourceFile),
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

pub type UnresolvedSymbol = microcad_package::tree::Symbol<UnresolvedSymbolDef>;

pub type UnresolvedSymbolRef<'mir> = microcad_package::tree::SymbolRef<'mir, UnresolvedSymbolDef>;

pub type UnresolvedSymbolTree = microcad_package::tree::SymbolTree<UnresolvedSymbolDef>;

#[derive(Debug, PartialEq, Artifact, Serialize, Deserialize)]
pub struct Mir {
    pub input_hash: HashId,
    pub output_hash: HashId,
    pub tree: UnresolvedSymbolTree,
}
