// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Creator of work pieces.

use crate::{resolve::*, value::Tuple};

/// A creator is the origin  
#[derive(Debug, Clone)]
pub struct Creator {
    /// Symbol.
    pub symbol: Symbol,
    /// Workpiece arguments.
    pub arguments: Tuple,
}

impl Creator {
    /// New creator.
    pub fn new(symbol: Symbol, arguments: Tuple) -> Self {
        Self { symbol, arguments }
    }
}

impl Info for Creator {
    fn info(&self) -> SymbolInfo {
        self.symbol.info()
    }
}

impl std::fmt::Display for Creator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{symbol}{arguments}",
            symbol = self.symbol.full_name(),
            arguments = self.arguments
        )
    }
}

impl std::hash::Hash for Creator {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.full_name().hash(state);
        self.arguments.hash(state);
    }
}
