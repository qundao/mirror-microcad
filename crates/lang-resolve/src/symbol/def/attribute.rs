// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Id, Identifier};
use microcad_lang_types::Value;

#[derive(Debug)]
pub struct Metadata(pub Vec<(Identifier, Value)>);

#[derive(Debug)]
#[non_exhaustive]
pub enum Command {
    Export {
        filename: std::path::PathBuf,
        exporter_id: Id,
    },
}

#[derive(Debug)]
pub struct PropertyAttributes;

pub use microcad_lang_lower::hir::DocBlock;
