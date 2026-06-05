// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::prelude::*;

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

    /// Creates a new render cache for this render parameter set.
    pub fn with_empty_cache(self) -> Self {
        self.with_cache(RcMut::new(RenderCache::default()))
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
    fn render(&mut self, params: impl Into<RenderParameters>) -> Result<Model>;
}
