// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol definitions

mod attribute;

use derive_more::From;
use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_lower::ir::{self, ConstantExpression};
use microcad_lang_types::{Type, Value};
use serde::{Deserialize, Serialize};

pub use microcad_lang_lower::ir::Visibility;

use crate::{ResolveContext, rst};

#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub struct ParameterAttributes;

#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub attr: ParameterAttributes,
    id: Identifier,
    ty: Type,
    default_value: Constant,
    src_ref: SrcRef,
}

#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub struct ParameterList {
    parameters: Box<[Parameter]>,
}

//pub type ModelExpression = ir::WorkbenchExpression<Name>;
pub type ValueExpression<NAME> = ir::FunctionExpression<NAME>;

pub type FunctionStatement<NAME> = ir::FunctionStatement<NAME>;

/*
pub struct ModelAttributes;

pub struct WorkbenchStatement {
    attr: ModelAttributes,
    id: Option<Identifier>,
    ty: Type,
    expr: ModelExpression,
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


    pub struct Wildcard;
*/

#[derive(Debug, Hash, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceFile {
    //attr: SourceFileAttributes,
    //statements: Box<[WorkbenchStatement]>,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineModule;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum Constant {
    Resolved(Value),
    Unresolved(ConstantExpression<rst::UnresolvedName>),
}

impl Constant {
    pub fn resolve<'rst>(
        &self,
        symbol: &rst::SymbolRef<'rst, rst::SymbolData<rst::UnresolvedName>>,
    ) -> Self {
        match self {
            Constant::Resolved(value) => Self::Resolved(value.clone()),
            Constant::Unresolved(constant_expression) => match constant_expression {
                ConstantExpression::Invalid => todo!(),
                ConstantExpression::Literal(literal) => Self::Resolved(literal.value().clone()),
                ConstantExpression::Call(call) => todo!(),
                ConstantExpression::Name(rst::UnresolvedName(path)) => {
                    match symbol.resolve(path.clone()) {
                        Some(symbol) => match &symbol.data.def {
                            SymbolDef::Root => todo!(),
                            SymbolDef::SourceFile(source_file) => todo!(),
                            SymbolDef::InlineModule(inline_module) => todo!(),
                            SymbolDef::FileModule => todo!(),
                            SymbolDef::Workbench => todo!(),
                            SymbolDef::Function(function) => todo!(),
                            SymbolDef::Constant(constant) => constant.resolve(&symbol),
                            SymbolDef::Builtin => todo!(),
                            SymbolDef::Alias => todo!(),
                            SymbolDef::Wildcard => todo!(),
                        },
                        None => todo!("Error handling"),
                    }
                }
                ConstantExpression::FormatString(format_string) => todo!(),
                ConstantExpression::ArrayExpression(array_expression) => todo!(),
                ConstantExpression::TupleExpression(tuple_expression) => todo!(),
                ConstantExpression::BinaryOp(binary_op) => todo!(),
                ConstantExpression::UnaryOp(unary_op) => todo!(),
            },
        }
    }
}

#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub struct FunctionAttributes;

#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "NAME: Serialize", deserialize = "NAME: Deserialize<'de>"))]
pub struct Function<NAME: Serialize> {
    attr: FunctionAttributes,
    parameters: ParameterList,
    return_type: Option<Type>,
    statements: Box<[FunctionStatement<NAME>]>,
}

/// Symbol definition
#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
#[serde(bound(serialize = "NAME: Serialize", deserialize = "NAME: Deserialize<'de>"))]
pub enum SymbolDef<NAME: Serialize> {
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
    Function(Function<NAME>),
    /// Constant.
    Constant(Constant),
    /// Builtin symbol.
    Builtin,
    /// Alias of a pub use statement.
    Alias,
    /// Use all available symbols in the module with the given name.
    Wildcard,
}
