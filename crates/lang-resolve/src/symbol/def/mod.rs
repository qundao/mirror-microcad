// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol definitions

mod attribute;

use microcad_lang_base::{HashId, Identifier, SrcRef};
use microcad_lang_lower::ir;
use microcad_lang_types::{Type, Value};
use serde::{Deserialize, Serialize};

use crate::{Symbol, symbol::def::def::SourceFile};

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

pub struct SymbolIndex {
    index: microcad_lang_base::HashMap<HashId, Symbol>,
}

impl Symbol {
    /// Find the name for a symbol
    pub fn resolve_name(&self, name: &ir::QualifiedName) -> Name {
        todo!()
    }

    pub fn hash(&self) -> HashId {
        todo!()
    }
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

mod def {
    use microcad_lang_lower::ir;

    pub struct SourceFile {
        attr: SourceFileAttributes,
        statements: Box<[WorkbenchStatement]>,
    }

    pub struct InlineModule {
        attr: ModuleAttributes,
    }

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

    pub struct Constant(Value);

    pub struct Wildcard;
}

/// Symbol definition
#[derive(Debug, Default)]
pub enum SymbolDef {
    /// An empty definition, used during building the symbol.
    #[default]
    Empty,

    /// Source file symbol.
    SourceFile(SourceFile),
    /// Inline Module symbol: `mod foo {}`
    InlineModule(InlineModule),
    /// File Module Symbol: `mod foo;`
    FileModule(FileModule),
    /// Workbench symbol.
    Workbench(),
    /// Function symbol.
    Function,
    /// Constant.
    Constant(Value),
    /// Builtin symbol.
    Builtin,
    /// Alias of a pub use statement.
    Alias(Symbol),
    /// Use all available symbols in the module with the given name.
    Wildcard(Wildcard),
}
