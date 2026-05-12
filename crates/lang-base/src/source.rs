// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{GetSourceLocInfoByHash, HashId, Hashed, ResourceLocation, SourceLocInfo};
use microcad_core::hash::ComputedHash;
use url::Url;

/// An unparsed source code with a location
#[derive(Debug, Clone)]
pub struct Source {
    /// The source url
    pub url: Url,
    /// Line offset
    pub line_offset: u32,
    /// The original code
    pub code: Hashed<String>,
}

impl Source {
    pub fn new(url: Url, line_offset: u32, code: String) -> Self {
        Self {
            url,
            line_offset,
            code: Hashed::new(code),
        }
    }
}

impl GetSourceLocInfoByHash for Source {
    fn get_source_loc_info_by_hash(&'_ self, hash: HashId) -> Option<SourceLocInfo<'_>> {
        if hash == self.code.computed_hash() {
            Some(SourceLocInfo {
                code: &self.code,
                url: self.url.clone(),
                line_offset: self.line_offset,
            })
        } else {
            None
        }
    }
}

impl<'a> ResourceLocation for Source {
    fn url(&self) -> &Url {
        &self.url
    }
}
