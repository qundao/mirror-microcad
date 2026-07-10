// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, SrcRef, SrcReferrer};
use microcad_lang_lower::{Ir, ir};
use miette::Diagnostic;
use thiserror::Error;

use crate::{
    Symbol, SymbolRef,
    rst::{
        self, SymbolData,
        def::{SourceFile, UnresolvedSymbolDef},
    },
};

/// Resolve error.
#[derive(Debug, Error, Diagnostic)]
pub enum ResolveError {}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;

/// Resolve Context
pub struct ResolveContext {
    pub builder: rst::Builder,

    //pub file_module_resolver: Box<dyn ResolveFileModule>,
    /// Diagnostic handler.
    pub diagnostics: Diagnostics,
}

impl ResolveContext {
    pub fn new(root: impl Into<Symbol<UnresolvedSymbolDef>>) -> Self {
        Self {
            builder: rst::Builder::new(root.into()),
            diagnostics: Diagnostics::default(),
        }
    }

    pub fn diag<E>(&mut self, err: E)
    where
        E: Into<miette::Report> + SrcReferrer,
    {
        self.diagnostics.push(err)
    }

    pub fn top<'tree>(&'tree self) -> SymbolRef<'tree, UnresolvedSymbolDef> {
        self.builder
            .tree
            .get(*self.builder.stack.last().unwrap())
            .unwrap()
    }
}

impl Resolve for rst::def::InlineModule {
    fn resolve(&self, context: &mut ResolveContext) -> ResolveResult<rst::Rst> {
        todo!()
    }
}

/// Trait to resolve an IR node into a symbol.
pub trait Resolve<T = rst::Rst> {
    fn resolve(&self, context: &mut ResolveContext) -> ResolveResult<T>;
}

impl From<&Ir> for Symbol<UnresolvedSymbolDef> {
    fn from(ir: &Ir) -> Self {
        Symbol {
            data: SymbolData {
                id: "root".into(),            // TODO Fetch name
                doc: ir::DocBlock::default(), // TODO
                visibility: ir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_ref: SrcRef::none(),
            },
            def: UnresolvedSymbolDef::SourceFile(SourceFile {}),
            parent: None,
            children: Default::default(),
        }
    }
}

impl From<&ir::InlineModule> for SymbolData {
    fn from(ir: &ir::InlineModule) -> Self {
        Self {
            id: ir.id.id().clone(),
            doc: ir.outer_attr.doc.clone(),
            visibility: ir.visibility.clone(),
            src_ref: ir.src_ref,
            keyword_ref: ir.keyword_ref,
        }
    }
}
