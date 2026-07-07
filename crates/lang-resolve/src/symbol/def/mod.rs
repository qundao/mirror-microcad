// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol definitions

mod attribute;

use derive_more::From;
use microcad_lang_base::{HashId, Identifier, SrcRef};
use microcad_lang_lower::ir;
use microcad_lang_types::{Type, Value};
use serde::{Deserialize, Serialize};

pub use ir::QualifiedName;

/// A resolved name
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Name {
    /// Resolved into a hash id
    Symbol(HashId),
    /// Resolved into a local identifier
    Local(Identifier),
    /// Error during resolve
    Error(QualifiedName),
}

#[derive(Debug, PartialEq)]
pub struct ParameterAttributes;

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub attr: ParameterAttributes,
    id: Identifier,
    ty: Type,
    default_value: Value,
    src_ref: SrcRef,
}

pub struct ParameterList {
    parameters: Box<[Parameter]>,
}

pub type ModelExpression = ir::WorkbenchExpression<Name>;
pub type ValueExpression = ir::FunctionExpression<Name>;

pub struct ModelAttributes;

pub struct WorkbenchStatement {
    attr: ModelAttributes,
    id: Option<Identifier>,
    ty: Type,
    expr: ModelExpression,
}

pub type FunctionStatment = ir::FunctionStatement<Name>;

/*
    pub struct FileModule {
        attr: ModuleAttributes,
    }

    pub struct InitAttributes {
        doc: ir::DocBlock,
    }

    pub struct Init {
        attr: InitAttributes,
        parameters: ParameterList,
        is_default: bool,
    }

    pub struct Workbench {
        attr: WorkbenchAttributes,
        inits: Box<[Init]>,
        statements: Box<[WorkbenchStatement]>,
    }

    pub struct Function {
        attr: FunctionAttributes,
        parameters: ParameterList,
        return_type: Option<Type>,
        statements: Box<[FunctionStatement]>,
    }

    pub struct Wildcard;
*/

#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct SourceFile {
    //attr: SourceFileAttributes,
    //statements: Box<[WorkbenchStatement]>,
}

#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct InlineModule;

#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct Constant(Value);

/// Symbol definition
#[derive(Debug, From, Hash, Serialize, Deserialize)]
pub enum SymbolDef {
    /// Root symbol
    Root,
    /// Source file symbol.
    SourceFile(SourceFile),
    /// Inline Module symbol: `mod foo {}`
    InlineModule(InlineModule),
    /// File Module Symbol: `mod foo;`
    FileModule,
    /// Workbench symbol.
    Workbench,
    /// Function symbol.
    Function,
    /// Constant.
    Constant(Value),
    /// Builtin symbol.
    Builtin,
    /// Alias of a pub use statement.
    Alias,
    /// Use all available symbols in the module with the given name.
    Wildcard,
}
