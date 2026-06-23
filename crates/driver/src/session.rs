// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{commands::Compile, prelude as mu};

/// A session with a VFS for documents.
pub struct Session {
    /// The cache of documents (virtual FS)
    pub documents: mu::HashMap<mu::Url, mu::Document>,
    /// Render cache
    pub render_cache: mu::RcMut<mu::RenderCache>,
    /// Configuration
    pub config: mu::DriverConfig,
}

impl Session {
    /// Create new session.
    pub fn new(config: mu::DriverConfig) -> mu::Result<Self> {
        mu::install_std()?;

        Ok(Self {
            documents: mu::HashMap::default(),
            render_cache: mu::RcMut::new(mu::RenderCache::new()),
            config,
        })
    }

    /// Load a document and try to compile it.
    pub fn load_document(&mut self, url: mu::Url) -> mu::Result<Model> {
        self.documents
            .insert(url.clone(), mu::Document::load(url.clone())?);
        self.compile_document(&url)
    }

    pub fn compile_document(&mut self, url: &mu::Url) -> mu::Result<Model> {
        match self.documents.get_mut(url) {
            Some(document) => document.compile(commands::CompileParameters {
                resolve: commands::compile::ResolveParameters {
                    search_paths: self.config.search_paths.clone(),
                },
            }),
            None => Err(miette::miette!("Document `{url}` not found!")),
        }
    }

    /// Remove document. The document is removed when it is physically removed from disk.
    pub fn remove_document(&mut self, url: &mu::Url) -> Option<mu::Document> {
        self.documents.remove(url)
    }

    pub fn get_document(&self, url: &mu::Url) -> Option<&mu::Document> {
        self.documents.get(url)
    }

    pub fn format_document(&mut self, url: &mu::Url) -> mu::Result<Vec<TextEdit>> {}
}
