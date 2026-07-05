// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve Context

use microcad_lang_base::{Diagnostics, Identifier};

use crate::ResolveResult;

pub struct ResolveStackFrame {
    symbol: Symbol,
    locals: Vec<Identifier>,
}

impl From<Symbol> for ResolveStackFrame {
    fn from(value: Symbol) -> Self {
        Self {
            symbol,
            locals: vec![],
        }
    }
}

pub struct ResolveStack(pub Vec<ResolveStackFrame>);

impl ResolveStack {
    pub fn new(root_symbols: impl Iter<Symbol>) -> Self {
        let root = ResolveStackFrame::from(Symbol::root().add_children(root_symbols));
        Self(vec![root])
    }

    pub fn push(&mut self, frame: Into<ResolveStackFrame>) {
        self.0.push(frame.into());
    }

    pub fn pop(&mut self) -> Option<ResolveStackFrame> {
        self.0.pop()
    }
}

pub trait ResolveFileModule {
    fn resolve_file(
        &mut self,
        module: &def::FileModule,
        context: &mut ResolveContext,
    ) -> ResolveResult<Symbol>;
}

/// Resolve Context
pub struct ResolveContext {
    pub stack: ResolveStack,

    /// File module loader
    pub file_module_resolver: Box<dyn ResolveFileModule>,

    /// Diagnostic handler.
    pub diagnostics: Diagnostics,
}

impl ResolveContext {
    pub fn new(
        root_symbols: impl Iter<Symbol>,
        file_module_resolver: dyn ResolveFileModule,
    ) -> Self {
        Self {
            stack: ResolveStack::new(root_symbols),
            file_module_resolver: Box::new(file_module_resolver),
            diagnostics: Diagnostics::default(),
        }
    }
}
