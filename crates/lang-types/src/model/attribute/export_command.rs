// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Export attribute.

use derive_more::Display;
use microcad_lang_base::Name;
use serde::{Deserialize, Serialize};

/// Export attribute, e.g. `#[export: "output.svg"]`.
#[derive(Clone, Debug, Display, PartialEq, Hash, Serialize, Deserialize)]
#[display("path = {}, id = {}", path.display(), id)]
pub struct ExportCommand {
    /// Filename.
    pub path: std::path::PathBuf,
    /// Exporter id
    pub id: Name,
}
