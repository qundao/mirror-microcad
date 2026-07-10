// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol definitions

mod attribute;

use derive_more::From;
use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_lower::{
    Identifiable,
    ir::{self, ConstantExpression},
};
use microcad_lang_types::{Type, Value};
use serde::{Deserialize, Serialize};

pub use microcad_lang_lower::ir::Visibility;

use crate::{
    Resolve, ResolveContext, ResolveResult, Symbol, SymbolRef,
    rst::{self, SymbolData, UnresolvedName},
};

#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub struct ParameterAttributes;

#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub attr: ParameterAttributes,
    id: Identifier,
    ty: Type,
    default_value: Value,
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

pub fn resolve_constant<'tree>(
    constant: &ConstantExpression<rst::UnresolvedName>,
    parent: rst::SymbolRef<'tree, UnresolvedSymbolDef>,
) -> ResolveResult<Value> {
    match constant {
        ConstantExpression::Invalid => todo!(),
        ConstantExpression::Literal(literal) => Ok(literal.value().clone()),
        ConstantExpression::Call(call) => todo!(),
        ConstantExpression::Name(rst::UnresolvedName(path)) => {
            use UnresolvedSymbolDef::*;
            let resolved = parent.resolve(path.clone());
            match resolved {
                Some(symbol) => match &symbol.def {
                    SourceFile(source_file) => todo!(),
                    InlineModule(inline_module) => todo!(),
                    FileModule => todo!(),
                    Workbench => todo!(),
                    Function(function) => todo!(),
                    Constant(constant) => resolve_constant(constant, symbol),
                    Builtin => todo!(),
                    Alias => todo!(),
                    Wildcard => todo!(),
                },
                None => todo!("Error handling"),
            }
        }
        ConstantExpression::FormatString(format_string) => todo!(),
        ConstantExpression::ArrayExpression(array_expression) => todo!(),
        ConstantExpression::TupleExpression(tuple_expression) => todo!(),
        ConstantExpression::BinaryOp(binary_op) => todo!(),
        ConstantExpression::UnaryOp(unary_op) => todo!(),
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
pub enum UnresolvedSymbolDef {
    /// Source file symbol.
    SourceFile(SourceFile),
    /// Inline Module symbol: `mod foo {}`
    InlineModule(InlineModule),
    /// File Module Symbol: `mod foo;`
    FileModule,
    /// Workbench symbol.
    Workbench,
    /// Function symbol.
    Function(Function<UnresolvedName>),
    /// Constant.
    Constant(ConstantExpression<rst::UnresolvedName>),
    /// Builtin symbol.
    Builtin,
    /// Alias of a pub use statement.
    Alias,
    /// Use all available symbols in the module with the given name.
    Wildcard,
}

/// Symbol definition
#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum ResolvedSymbolDef {
    /// Source file symbol.
    SourceFile(SourceFile),
    /// Inline Module symbol: `mod foo {}`
    InlineModule(InlineModule),
    /// File Module Symbol: `mod foo;`
    FileModule,
    /// Workbench symbol.
    Workbench,
    /// Function symbol.
    Function(Function<rst::ResolvedName>),
    /// Constant.
    Constant(Value),
    /// Builtin symbol.
    Builtin,
    /// Alias of a pub use statement.
    Alias,
    /// Use all available symbols in the module with the given name.
    Wildcard,
}

pub fn inline_module(ir: &ir::InlineModule) -> rst::SymbolTree<UnresolvedSymbolDef> {
    Symbol::new(ir.into(), UnresolvedSymbolDef::InlineModule(InlineModule)).into()
}

pub fn resolve_symbol<'tree>(
    symbol_ref: SymbolRef<'tree, UnresolvedSymbolDef>,
) -> ResolveResult<crate::Symbol<ResolvedSymbolDef>> {
    Ok(crate::Symbol {
        def: match &symbol_ref.def {
            UnresolvedSymbolDef::SourceFile(source_file) => {
                ResolvedSymbolDef::SourceFile(SourceFile {})
            }
            UnresolvedSymbolDef::InlineModule(inline_module) => {
                ResolvedSymbolDef::InlineModule(InlineModule)
            }
            UnresolvedSymbolDef::FileModule => todo!(),
            UnresolvedSymbolDef::Workbench => todo!(),
            UnresolvedSymbolDef::Function(function) => todo!(),
            UnresolvedSymbolDef::Constant(constant_expression) => {
                ResolvedSymbolDef::Constant(resolve_constant(constant_expression, symbol_ref)?)
            }
            UnresolvedSymbolDef::Builtin => todo!(),
            UnresolvedSymbolDef::Alias => todo!(),
            UnresolvedSymbolDef::Wildcard => todo!(),
        },
        data: symbol_ref.data.clone(),
        parent: symbol_ref.parent,
        children: symbol_ref.children.clone(),
    })
}
