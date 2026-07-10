// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol definitions

mod attribute;

use derive_more::From;
use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_lower::ir;
use microcad_lang_types::{Type, Value};
use serde::{Deserialize, Serialize};

pub use microcad_lang_lower::ir::Visibility;

use crate::{ResolveResult, Symbol, SymbolRef, rst};

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
    constant: &rst::def::Constant,
    parent: rst::SymbolRef<'tree, UnresolvedSymbolDef>,
) -> ResolveResult<Value> {
    use ir::ConstantExpression::*;
    match &constant.expr {
        Invalid => todo!(),
        Literal(literal) => Ok(literal.value().clone()),
        Call(_) => todo!(),
        Name(name) => {
            use UnresolvedSymbolDef::*;
            let path: rst::SymbolPath = name.into();
            let resolved = parent.resolve(path.clone());
            match resolved {
                Some(symbol) => match &symbol.def {
                    SourceFile(_) => todo!(),
                    InlineModule => todo!(),
                    FileModule => todo!(),
                    Workbench => todo!(),
                    Function(_) => todo!(),
                    Constant(constant) => resolve_constant(constant, symbol),
                    Builtin => todo!(),
                    Alias => todo!(),
                    Wildcard => todo!(),
                },
                None => todo!("Error handling"),
            }
        }
        FormatString(_) => todo!(),
        ArrayExpression(_) => todo!(),
        TupleExpression(_) => todo!(),
        BinaryOp(_) => todo!(),
        UnaryOp(_) => todo!(),
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

#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub struct Constant {
    pub ty: Option<ir::TypeAnnotation>,
    pub expr: ir::ConstantExpression,
}

/// Symbol definition
#[derive(Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum UnresolvedSymbolDef {
    /// Source file symbol.
    SourceFile(SourceFile),
    /// Inline Module symbol: `mod foo {}`
    InlineModule,
    /// File Module Symbol: `mod foo;`
    FileModule,
    /// Workbench symbol.
    Workbench,
    /// Function symbol.
    Function(Function<rst::UnresolvedName>),
    /// Constant.
    Constant(Constant),
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

pub fn constant(ir: &ir::Constant) -> rst::SymbolTree<UnresolvedSymbolDef> {
    Symbol::new(
        ir.into(),
        UnresolvedSymbolDef::Constant(Constant {
            ty: ir.ty.clone(),
            expr: ir.expr.clone(),
        }),
    )
    .into()
}

pub fn inline_module(ir: &ir::InlineModule) -> rst::SymbolTree<UnresolvedSymbolDef> {
    let mut builder = rst::Builder::new(Symbol::new(ir.into(), UnresolvedSymbolDef::InlineModule));

    for ir in &ir.items.modules {
        builder.add(inline_module(ir));
    }

    for ir in &ir.items.constants {
        builder.add(constant(ir));
    }

    builder.tree
}

pub fn resolve_symbol<'tree>(
    symbol_ref: SymbolRef<'tree, UnresolvedSymbolDef>,
) -> ResolveResult<crate::Symbol<ResolvedSymbolDef>> {
    Ok(crate::Symbol {
        def: match &symbol_ref.def {
            UnresolvedSymbolDef::SourceFile(_) => ResolvedSymbolDef::SourceFile(SourceFile {}),
            UnresolvedSymbolDef::InlineModule => ResolvedSymbolDef::InlineModule(InlineModule),
            UnresolvedSymbolDef::FileModule => todo!(),
            UnresolvedSymbolDef::Workbench => todo!(),
            UnresolvedSymbolDef::Function(_) => todo!(),
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
