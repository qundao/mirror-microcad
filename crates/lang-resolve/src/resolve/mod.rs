// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Diagnostics, SrcReferrer};
use miette::Diagnostic;
use thiserror::Error;

use crate::rst;

/// Resolve error.
#[derive(Debug, Error, Diagnostic)]
pub enum ResolveError {}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;

/// Resolve Context
pub struct ResolveContext {
    //pub stack: ResolveStack,
    /// File module loader
    //pub file_module_resolver: Box<dyn ResolveFileModule>,

    /// Diagnostic handler.
    pub diagnostics: Diagnostics,
}

impl ResolveContext {
    pub fn new(//root_symbols: impl Iter<Symbol>,
        //file_module_resolver: dyn ResolveFileModule,
    ) -> Self {
        Self {
            //stack: ResolveStack::new(root_symbols),
            //file_module_resolver: Box::new(file_module_resolver),
            diagnostics: Diagnostics::default(),
        }
    }

    pub fn diag<E>(&mut self, err: E)
    where
        E: Into<miette::Report> + SrcReferrer,
    {
        self.diagnostics.push(err)
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
