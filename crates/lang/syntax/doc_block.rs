// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Documentation block syntax element

use crate::{src_ref::*, syntax::*};

/// Retrieve doc from symbol definition.
pub trait Doc {
    /// Return documentation
    fn doc(&self) -> Option<DocBlock>;
}

/// Block of documentation comments, starting with `/// `.
#[derive(Clone, Debug, Default)]
pub struct DocBlock(pub Refer<Vec<String>>);

impl DocBlock {
    /// Create new doc block for builtin.
    pub fn new_builtin(comment: &str) -> Self {
        Self(Refer::none(
            comment.lines().map(|s| s.to_string()).collect(),
        ))
    }

    /// Check if this doc block is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Merge two doc blocks, e.g. for merging inner and outer docs
    pub fn merge(a: &DocBlock, b: &DocBlock) -> DocBlock {
        match (a.is_empty(), b.is_empty()) {
            (true, true) => Self::default(),
            (true, false) => b.clone(),
            (false, true) => a.clone(),
            _ => {
                let merged =
                    a.0.iter()
                        .chain([String::default()].iter()) // Add an empty line
                        .chain(b.0.iter())
                        .cloned()
                        .collect::<Vec<_>>();
                Self(Refer::new(
                    merged,
                    SrcRef::merge(&a.src_ref(), &b.src_ref()),
                ))
            }
        }
    }

    /// Remove `///` comment marks and return them as string.
    pub fn fetch_text(&self) -> String {
        self.0
            .iter()
            .filter_map(|s| s.strip_prefix("/// ").or(s.strip_prefix("///")))
            .map(|s| s.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl SrcReferrer for DocBlock {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for DocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.value.join("\n"))
    }
}

impl TreeDisplay for DocBlock {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}DocBlock: '{}'",
            "",
            crate::shorten!(self.0.first().cloned().unwrap_or_default())
        )
    }
}
#[test]
fn doc_block_merge() {
    let doc_a = DocBlock(Refer::none(vec!["/// line 1".to_string()]));
    let doc_b = DocBlock(Refer::none(vec!["/// line 2".to_string()]));
    let empty = DocBlock::default();

    // Test: Merge with empty blocks
    assert!(DocBlock::merge(&empty, &empty).is_empty());

    let merge_with_empty_left = DocBlock::merge(&empty, &doc_a);
    assert_eq!(merge_with_empty_left.0.value, vec!["/// line 1"]);

    let merge_with_empty_right = DocBlock::merge(&doc_a, &empty);
    assert_eq!(merge_with_empty_right.0.value, vec!["/// line 1"]);

    // Test: Merge two populated blocks
    let merged = DocBlock::merge(&doc_a, &doc_b);

    // The implementation adds an empty line (String::default()) between blocks
    let expected = vec![
        "/// line 1".to_string(),
        "".to_string(),
        "/// line 2".to_string(),
    ];

    assert_eq!(merged.0.value, expected);
}

#[test]
fn doc_block_fetch_text() {
    // Test 1: Standard space-separated doc comments
    let doc1 = DocBlock(Refer::none(vec![
        "/// Line one".to_string(),
        "/// Line two ".to_string(), // Note the trailing space
    ]));
    assert_eq!(doc1.fetch_text(), "Line one\nLine two");

    // Test 2: Mixed prefixes (with and without space)
    let doc2 = DocBlock(Refer::none(vec![
        "///Space".to_string(),
        "///No space".to_string(),
    ]));
    assert_eq!(doc2.fetch_text(), "Space\nNo space");

    // Test 3: Lines that don't start with '///' should be ignored
    let doc3 = DocBlock(Refer::none(vec![
        "/// Valid".to_string(),
        "Invalid line".to_string(),
        "/// Also valid".to_string(),
    ]));
    assert_eq!(doc3.fetch_text(), "Valid\nAlso valid");

    // Test 4: Empty DocBlock
    let doc_empty = DocBlock::default();
    assert_eq!(doc_empty.fetch_text(), "");
}
