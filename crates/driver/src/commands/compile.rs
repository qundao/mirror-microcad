// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::prelude::*;

/// Parse a document from source.
pub trait Parse {
    fn parse(&mut self) -> Result;
}

/// Lower the AST of a Document into an intermediate representation (IR).
pub trait Lower {
    fn lower(&mut self) -> Result;
}

/// Parameters for resolve
#[derive(Clone)]
pub struct ResolveParameters {
    pub search_paths: Vec<std::path::PathBuf>,
}

impl Default for ResolveParameters {
    fn default() -> Self {
        Self {
            search_paths: microcad_builtin::dirs::default_search_paths(),
        }
    }
}

/// Resolve the IR into a symbol tree.
pub trait Resolve {
    fn resolve(&mut self, params: impl Into<ResolveParameters>) -> Result<Symbol>;
}

/// Resolve the IR into a symbol tree.
pub trait Eval {
    fn eval(&mut self) -> Result<Model>;
}

/// Compile parameters
#[derive(Default, Clone)]
pub struct CompileParameters {
    /// Resolve parameters.
    pub resolve: ResolveParameters,
}

/// Trait for compilation toolchain.
pub trait Compile: Parse + Lower + Resolve + Eval {
    /// Compile a document.
    fn compile(&mut self, parameters: impl Into<CompileParameters>) -> Result<Model> {
        let parameters = parameters.into();
        self.parse()?;
        self.lower()?;
        self.resolve(parameters.resolve)?;
        self.eval()
    }
}
