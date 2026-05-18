// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::HashMap;

use crate::prelude::*;

#[derive(Default)]
pub struct Session {
    pub documents: HashMap<Url, Document>,

    pub render_cache: Option<RcMut<RenderCache>>,

    pub config: Config,
}

impl Session {
    pub fn new(config: Config) -> Self {
        Self {
            documents: HashMap::default(),
            render_cache: Some(RcMut::new(RenderCache::new())),
            config,
        }
    }

    pub fn add_document(&mut self, _url: Url) -> &mut Document {
        todo!()
    }

    pub fn remove_document(&mut self, _url: Url) -> Document {
        todo!()
    }
}
