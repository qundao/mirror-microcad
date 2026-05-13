// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

use microcad_core::RenderResolution;
use microcad_lang::{
    render::{ProgressTx, RenderCache},
    value::Quantity,
};
use microcad_lang_base::RcMut;

use crate::document::{self, Result};

pub struct RenderParameters {
    pub resolution: RenderResolution,
    pub cache: Option<RcMut<RenderCache>>,
    pub progress_tx: Option<ProgressTx>,
}

impl RenderParameters {
    pub fn new(resolution: RenderResolution) -> Self {
        Self {
            resolution,
            cache: None,
            progress_tx: None,
        }
    }

    pub fn with_cache(self, cache: RcMut<RenderCache>) -> Self {
        Self {
            resolution: self.resolution,
            cache: Some(cache),
            progress_tx: self.progress_tx,
        }
    }
}

impl FromStr for RenderParameters {
    type Err = miette::Report;

    fn from_str(s: &str) -> document::Result<Self> {
        use microcad_lang::value::Value;

        match crate::value_from_str(s)? {
            Value::Quantity(q) => match q.quantity_type {
                microcad_lang::ty::QuantityType::Length => {
                    Ok(RenderParameters::new(RenderResolution::new(q.value)))
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

pub trait Render {
    fn render(&mut self, params: &RenderParameters) -> document::Result;
}
