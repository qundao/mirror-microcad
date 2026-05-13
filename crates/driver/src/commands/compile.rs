// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Result;

use microcad_core::RenderResolution;
use microcad_lang::render::{ProgressTx, RenderCache};
use microcad_lang_base::RcMut;

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
    fn resolve(&mut self, params: impl Into<ResolveParameters>) -> Result;
}

/// Resolve the IR into a symbol tree.
pub trait Eval {
    fn eval(&mut self) -> Result;
}

#[derive(Default, Clone)]
pub struct RenderParameters {
    pub resolution: RenderResolution,
    pub cache: Option<RcMut<RenderCache>>,
    pub progress_tx: Option<ProgressTx>,
}

impl RenderParameters {
    pub fn with_cache(self, cache: RcMut<RenderCache>) -> Self {
        Self {
            resolution: self.resolution,
            cache: Some(cache),
            progress_tx: self.progress_tx,
        }
    }
}

impl From<RenderResolution> for RenderParameters {
    fn from(resolution: RenderResolution) -> Self {
        Self {
            resolution,
            cache: None,
            progress_tx: None,
        }
    }
}

impl std::str::FromStr for RenderParameters {
    type Err = miette::Report;

    fn from_str(s: &str) -> Result<Self> {
        use microcad_lang::value::Value;

        match crate::value_from_str(s)? {
            Value::Quantity(q) => match q.quantity_type {
                microcad_lang::ty::QuantityType::Length => {
                    Ok(RenderParameters::from(RenderResolution::new(q.value)))
                }
                _ => Err(miette::miette!(
                    "Cannot convert quantity `{q}` into `RenderParameters`"
                )),
            },
            value => Err(miette::miette!(
                "Cannot convert value `{value}` into `RenderParameters`"
            )),
        }
    }
}

/// Render a model.
pub trait Render {
    fn render(&mut self, params: impl Into<RenderParameters>) -> Result;
}

/// Compile parameters
#[derive(Default, Clone)]
pub struct CompileParameters {
    /// Resolve parameters.
    pub resolve: ResolveParameters,
    /// Render parameters.
    pub render: RenderParameters,
}

/// Trait for compilation toolchain.
pub trait Compile: Parse + Lower + Resolve + Eval + Render {
    /// Compile a document.
    fn compile(&mut self, parameters: impl Into<CompileParameters>) -> Result {
        let parameters = parameters.into();
        self.parse()?;
        self.lower()?;
        self.resolve(parameters.resolve)?;
        self.eval()?;
        self.render(parameters.render)
    }
}
