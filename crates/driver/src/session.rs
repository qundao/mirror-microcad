// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{commands::Compile, prelude::*};

/// A session with a VFS for documents.
pub struct Session {
    /// The cache of documents (virtual FS)
    pub documents: HashMap<Url, Document>,
    /// Render cache
    pub render_cache: RcMut<RenderCache>,
    /// Configuration
    pub config: DriverConfig,
}

impl Session {
    /// Create new session.
    pub fn new(config: DriverConfig) -> Self {
        Self {
            documents: HashMap::default(),
            render_cache: RcMut::new(RenderCache::new()),
            config,
        }
    }

    /// Load a document and try to compile it.
    pub fn load_document(&mut self, url: Url) -> Result<Model> {
        self.documents
            .insert(url.clone(), document::Document::load(url.clone())?);
        self.compile_document(&url)
    }

    pub fn compile_document(&mut self, url: &Url) -> Result<Model> {
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
    pub fn remove_document(&mut self, url: &Url) -> Option<Document> {
        self.documents.remove(url)
    }
}
