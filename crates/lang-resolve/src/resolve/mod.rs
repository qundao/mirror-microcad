// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod scaffold;

mod case_check;
mod type_check;

use microcad_lang_base::{Diagnostics, HashId, SrcRef, SrcReferrer, element::Case};
use microcad_lang_lower::ir;
use miette::Diagnostic;
use thiserror::Error;

use crate::{rst, scaffold::ScaffoldError};

/// Resolve error.
#[derive(Debug, Error, Diagnostic)]
pub enum ResolveError {
    #[error("{0}")]
    ScaffoldError(#[from] ScaffoldError),

    #[error("Wrong case")]
    #[diagnostic(severity = "warning")]
    WrongCase {
        expected: Case,
        actual: Case,
        #[label]
        src_ref: SrcRef,
    },
    #[error("Type mismatch: {specified} != {actual}")]
    TypeMismatch {
        specified: rst::Type,
        #[label("Specified type")]
        specified_src_ref: SrcRef,
        actual: rst::Type,
        #[label("Actual type")]
        actual_src_ref: SrcRef,
    },
    #[error("No source with hash: {0}")]
    NoSourceWithHash(HashId),
}

impl SrcReferrer for ResolveError {
    fn src_ref(&self) -> SrcRef {
        match self {
            ResolveError::ScaffoldError(err) => err.src_ref(),
            ResolveError::WrongCase { src_ref, .. } => *src_ref,
            ResolveError::TypeMismatch {
                specified_src_ref, ..
            } => *specified_src_ref,
            ResolveError::NoSourceWithHash(_) => SrcRef::none(),
        }
    }
}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;

/// Resolve Context
pub struct ResolveContext {
    //pub file_module_resolver: Box<dyn ResolveFileModule>,
    /// Diagnostic handler.
    pub diagnostics: Diagnostics,
}

impl ResolveContext {
    pub fn new() -> Self {
        Self {
            diagnostics: Diagnostics::default(),
        }
    }

    pub fn diag<E>(&mut self, err: E)
    where
        E: Into<miette::Report> + SrcReferrer,
    {
        self.diagnostics.push(err)
    }

    /*pub fn top<'tree>(&'tree self) -> SymbolRef<'tree, UnresolvedSymbolDef> {
        self.builder
            .tree
            .get(*self.builder.stack.last().unwrap())
            .unwrap()
    }*/
}

/// Trait to resolve an IR node into a symbol.
pub trait Resolve<T = rst::Rst> {
    fn resolve(&self, context: &mut ResolveContext) -> ResolveResult<T>;
}

impl From<ir::QualifiedName> for crate::tree::SymbolPath {
    fn from(name: ir::QualifiedName) -> Self {
        name.value
            .clone()
            .iter()
            .map(|id| id.id().clone())
            .collect::<Vec<_>>()
            .into()
    }
}

/*


pub fn resolve_constant<'tree>(
    constant: &rst::def::Constant,
    parent: rst::SymbolRef<'tree, UnresolvedSymbolDef>,
) -> ResolveResult<Value> {
    use ir::ConstantExpression::*;
    match &constant.expr {
        Invalid => todo!(),
        Literal(literal) => Ok(literal.value().clone()),
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
            UnresolvedSymbolDef::Constant(constant) => {
                ResolvedSymbolDef::Constant(resolve_constant(constant, symbol_ref)?)
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

*/
