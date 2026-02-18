// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Inner doc syntax element impl.

use crate::{src_ref::*, tree_display::*};

/// Inner doc syntax element: `//!`.
///
/// A doc comment statement only contains one line of documentation.
#[derive(Clone, Debug)]
pub struct InnerDocComment(pub Refer<String>);

impl SrcReferrer for InnerDocComment {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for InnerDocComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/// {}", self.0.value)
    }
}

impl TreeDisplay for InnerDocComment {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}Doc: {}", "", self.0)
    }
}
