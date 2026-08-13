// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad package API

pub mod manifest;
pub mod symbol;

pub use manifest::{Manifest, ManifestError};

pub use symbol::{
    SymbolArena, SymbolDef, SymbolId, SymbolNode, SymbolNodeExt, SymbolNodeMut, SymbolNodeRef,
};
