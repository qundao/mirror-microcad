// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Documentation block syntax element

use crate::{src_ref::*, syntax::*};

trait Doc {
    fn doc(&self) -> &DocBlock;
    fn split_doc(&self) -> Option<(String, Option<String>)> {
        let mut parts = self.doc().lines.split(|line| line.trim().is_empty());

        let summary = parts.next().unwrap_or(&[]).join("\n");
        let summary = summary.trim();
        let details = parts.next().map(|lines| lines.join("\n"));

        if summary.is_empty() {
            None
        } else {
            Some((summary.to_string(), details))
        }
    }
}

/// Block of documentation comments, starting with `/// `.
#[derive(Clone, Default)]
pub struct DocBlock {
    /// Doc comment lines.
    pub lines: Vec<String>,
    /// Source reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for DocBlock {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for DocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.lines
            .iter()
            .try_for_each(|doc| writeln!(f, "/// {doc}"))
    }
}

impl TreeDisplay for DocBlock {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}DocBlock:", "")?;
        depth.indent();
        self.lines
            .iter()
            .try_for_each(|doc| writeln!(f, "{:depth$}/// {doc}", ""))
    }
}
