// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::RenderResolution;
use microcad_lang::render::{ProgressTx, RenderCache};
use microcad_lang_base::RcMut;

use crate::document;

pub struct RenderParameters {
    pub resolution: RenderResolution,
    pub cache: Option<RcMut<RenderCache>>,
    pub progress_tx: Option<ProgressTx>,
}

impl RenderParameters {
    pub fn new(resolution: String) -> Self {
        todo!()
    }

    pub fn with_cache(self, cache: RcMut<RenderCache>) -> Self {
        Self {
            resolution: self.resolution,
            cache: Some(cache),
            progress_tx: self.progress_tx,
        }
    }
}

pub trait Render {
    fn render(&self, params: &RenderParameters) -> document::Result;
}
