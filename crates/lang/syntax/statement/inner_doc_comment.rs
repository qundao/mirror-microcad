// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Inner doc syntax element impl.

use microcad_lang_base::{Refer, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

/// Inner doc syntax element: `//!`.
///
/// A doc comment statement only contains one line of documentation.
#[derive(Clone, Debug, SrcReferrer)]
pub struct InnerDocComment(pub Refer<String>);

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
