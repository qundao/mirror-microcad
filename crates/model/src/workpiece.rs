// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Work piece element

use microcad_lang_base::{Hashed, element::WorkbenchKind};

use crate::creator::Creator;

/// A workpiece is an element produced by a workbench.
#[derive(Debug, Clone)]
pub struct Workpiece {
    /// Workpiece kind: `op`, `sketch`, `part`.
    pub kind: WorkbenchKind,
    /// Workpiece properties.
    pub creator: Hashed<Creator>,
}
