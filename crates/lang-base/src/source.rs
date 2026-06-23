// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{GetSourceLocInfoByHash, HashId, Hashed, ResourceLocation, SourceLocInfo};
use microcad_core::hash::ComputedHash;
use serde::Serialize;
use url::Url;

/// Unparsed source code with a location.
///
/// The unique location of µcad file is specified via:
/// * `url: Url`: A URL pointing to a source code
/// * `line_offset: u32`: A line offset inside file
///
/// Additionally, a unique hash of the source code computed.
#[derive(Debug, Clone, Serialize)]
pub struct Source {
    /// The source url
    pub url: Url,
    /// Line offset
    pub line_offset: u32,
    /// The original hashed code
    pub code: Hashed<String>,
}

impl Source {
    /// Create a new source.
    pub fn new(url: Url, line_offset: u32, code: String) -> Self {
        Self {
            url,
            line_offset,
            code: Hashed::new(code),
        }
    }

    pub fn code(&self) -> &str {
        self.code.value()
    }

    pub fn set_code(&mut self, code: String) {
        self.code = Hashed::new(code);
    }

    pub fn compare(&self, other: &Self) -> Vec<TextEdit> {
        
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

impl ResourceLocation for Source {
    fn url(&self) -> &Url {
        &self.url
    }
}
