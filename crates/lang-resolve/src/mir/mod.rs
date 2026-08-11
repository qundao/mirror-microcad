// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mid-level intermediate representation (MIR).

use derive_more::From;
use microcad_lang_base::{HashId, Name, Refer, SrcRef};
use microcad_lang_proc_macros::Artifact;
use serde::{Deserialize, Serialize};

use microcad_lang_lower::ir;

pub use microcad_package::tree::{SymbolHandle, SymbolMetadata};

pub type SymbolPath = ir::Path;
pub type Type = ir::Type;

pub type Path = ir::Path;

pub type ConstantExpression = ir::ConstantExpression;

pub type Attributes = ir::Attributes;

pub type FunctionSignature = ir::FunctionSignature;
pub type Parameter = ir::Parameter;
pub type ParameterList = ir::ParameterList;

pub use ir::{Identifier, Visibility};

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Constant {
    pub attr: Attributes,
    pub ty: Type,
    pub expr: ConstantExpression,
}

pub type FunctionExpression = ir::FunctionExpression;
pub type FunctionStatement = ir::FunctionStatement;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub attr: Attributes,
    pub parameters: ParameterList,
    pub return_ty: Option<Type>,
    pub statements: Box<[FunctionStatement]>,
}

pub type WorkbenchExpression = ir::WorkbenchExpression;
pub type WorkbenchStatement = ir::WorkbenchStatement;
pub type WorkbenchKind = ir::WorkbenchKind;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]

pub struct InitStatement {
    pub id: Identifier,
    pub ty: Type,
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
pub struct Alias(pub Path);

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wildcard(pub Path);

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
