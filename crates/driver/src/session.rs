// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::render::RenderCache;
use microcad_lang_base::HashMap;

use crate::{Config, Document, RcMut, Url};

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
            render_cache: None,
            config,
        }
    }

    pub fn init_render_cache(mut self) -> Self {
        self.render_cache = Some(RcMut::new(RenderCache::new()));
        self
    }

    pub fn install_std(&self) {
        #[cfg(not(debug_assertions))]
        microcad_std::StdLib::new(microcad_std::StdLib::default_path())
            .map_err(|err| miette::miette!("Could not load standard library: {err}"))?;
    }

    pub fn add_document(&mut self, url: Url) -> &mut Document {
        todo!()
    }

    pub fn remove_document(&mut self, url: Url) -> Document {
        todo!()
    }

    /// Return a path with default µcad extension given in the config.
    pub fn path_with_default_ext(&self, path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        let mut path = path.as_ref().to_path_buf();
        if path.extension().is_none() {
            path.set_extension(self.config.default_extension.clone());
        }
        path
    }
}
