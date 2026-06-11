// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source code references.
//!
//! All errors which occur while *parsing* or *evaluating* µcad code need to be reported and
//! therefore need to address a place in the code where they did appear.
//! A bunch of structs from this module provide this functionality:
//!
//! - [`SrcRef`] boxes a [`SrcRefInner`] which itself includes all necessary reference
//!   information like *line*/*column* and a hash to identify the source file.
//! - [`Refer`] encapsulates any syntax element and puts a [`SrcRef`] beside it.
//! - [`SrcReferrer`] is a trait which provides unified access to the [`SrcRef`]
//!   (e.g. implemented by [`Refer`]).

mod line_col;
mod line_index;
mod refer;
mod src_referrer;

pub use line_col::*;
pub use line_index::*;
pub use refer::*;
pub use src_referrer::*;

use crate::HashId;

use miette::SourceSpan;
use serde::Serialize;

/// Span for tokens or AST nodes, a range of byte offsets from the start of the source
pub type Span = std::ops::Range<usize>;

/// Reference into a source file.
///
/// *Hint*: Source file is not part of `SrcRef` and must be provided from outside
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct SrcRef {
    /// Start position in bytes
    pub start: usize,

    /// End position in bytes
    pub end: usize,

    /// Line and column
    pub at: LineCol,
    /// Hash of the source code file to map `SrcRef` -> `SourceFile`
    ///
    /// A `source_hash == 0` is considered as `None`.
    pub source_hash: HashId,
}

impl SrcRef {
    /// Create new `SrcRef`
    /// - `range`: Position in file
    /// - `line`: Line number within file
    /// - `col`: Column number within file
    pub fn new(range: std::ops::Range<usize>, at: LineCol, source_hash: HashId) -> Self {
        Self {
            start: range.start,
            end: range.end,
            at,
            source_hash,
        }
    }

    pub fn none() -> Self {
        Self::default()
    }

    /// Return a span for the source reference as expected by miette
    pub fn as_miette_span(&self) -> Option<SourceSpan> {
        if self.is_some() {
            Some(SourceSpan::new(self.start.into(), self.len()))
        } else {
            None
        }
    }

    pub fn is_none(&self) -> bool {
        self.source_hash == 0
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Check if two source refs are overlapping.
    ///
    /// This means they must have the same non-zero source file hash and its ranges must overlap.
    pub fn is_overlapping(&self, other: &Self) -> bool {
        self.is_some() && other.is_some() && (self.start < other.end) && (other.start < self.end)
    }

    /// Return a reference with a given line offset.
    pub fn with_line_offset(&self, line_offset: u32) -> Self {
        let mut s = self.clone();
        s.at.line += line_offset;
        s
    }

    /// Return range between `start..end`
    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }

    /// return length of `SrcRef`
    pub fn len(&self) -> usize {
        self.range().len()
    }

    /// return true if code base is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// return source file hash
    /// - `0` if not `SrcRefInner` is none
    /// - `u64` if `SrcRefInner` is some
    ///
    /// This is used to map `SrcRef` -> `SourceFile`
    pub fn source_hash(&self) -> u64 {
        self.source_hash
    }

    /// Return slice to code base.
    pub fn source_slice<'a>(&self, src: &'a str) -> &'a str {
        assert!(self.is_some());
        &src[self.range().to_owned()]
    }

    /// Merge two `SrcRef` into a single one.
    ///
    /// `SrcRef::none()` is returned if:
    /// - ranges not in correct order (warning in log),
    /// - references are not in the same file (warning in log),
    /// - or `lhs` and `rhs` are both `None`.
    pub fn merge(lhs: &impl SrcReferrer, rhs: &impl SrcReferrer) -> SrcRef {
        let lhs = lhs.src_ref();
        let rhs = rhs.src_ref();

        match (lhs.is_some(), rhs.is_some()) {
            (true, true) => {
                if lhs.source_hash == rhs.source_hash {
                    let source_hash = lhs.source_hash;

                    if lhs.range() == rhs.range() {
                        lhs
                    } else if lhs.end > rhs.start || lhs.start > rhs.end {
                        log::warn!(
                            "ranges not in correct order: {lhs} vs {rhs} @ {source_hash}",
                            lhs = lhs.at,
                            rhs = rhs.at
                        );
                        SrcRef::none()
                    } else {
                        assert!(lhs.end <= rhs.end);
                        assert!(lhs.start <= rhs.start);

                        SrcRef {
                            start: lhs.start,
                            end: rhs.end,
                            at: lhs.at,
                            source_hash,
                        }
                    }
                } else {
                    log::warn!("references are not in the same file");
                    SrcRef::none()
                }
            }
            (true, false) => lhs.clone(),
            (false, true) => rhs.clone(),
            (false, false) => SrcRef::none(),
        }
    }

    /// Merge all given source references to one
    ///
    /// All  given source references must have the same hash otherwise panics!
    pub fn merge_all<S: SrcReferrer>(referrers: impl Iterator<Item = S>) -> SrcRef {
        let mut result = SrcRef::none();
        for referrer in referrers {
            let src_ref = referrer.src_ref();
            if src_ref.is_some() {
                if result.is_some() {
                    if result.source_hash != src_ref.source_hash {
                        panic!("can only merge source references of the same file");
                    }
                    if src_ref.start < result.start {
                        result.start = src_ref.start;
                        result.at = src_ref.at;
                    }
                    result.end = std::cmp::max(src_ref.end, result.end);
                } else {
                    result = src_ref;
                }
            }
        }
        result
    }

    /// Return line and column in source code or `None` if not available.
    pub fn at(&self) -> Option<LineCol> {
        if self.is_some() {
            Some(self.at.clone())
        } else {
            None
        }
    }

    /// Get the line of the start of the referenced source, if any
    pub fn line(&self) -> Option<u32> {
        if self.is_some() {
            Some(self.at.line)
        } else {
            None
        }
    }

    /// Get the column of the start of the referenced source, if any
    pub fn col(&self) -> Option<u32> {
        if self.is_some() {
            Some(self.at.col)
        } else {
            None
        }
    }
}

impl From<SrcRef> for SourceSpan {
    fn from(value: SrcRef) -> Self {
        value
            .as_miette_span()
            .unwrap_or(SourceSpan::new(0.into(), 0))
    }
}

impl std::fmt::Display for SrcRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.is_some() {
            true => write!(f, "{}", self.at),
            false => write!(f, "<NO REF>"),
        }
    }
}

impl std::fmt::Debug for SrcRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.is_some() {
            true => write!(
                f,
                "{} ({}..{}) in {:#x}",
                self.at, self.start, self.end, self.source_hash
            ),
            false => write!(f, "<NO REF>"),
        }
    }
}

impl PartialEq for SrcRef {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl PartialOrd for SrcRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SrcRef {}

impl Ord for SrcRef {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

unsafe impl bytemuck::Zeroable for SrcRef {}
unsafe impl bytemuck::Pod for SrcRef {}

impl Serialize for SrcRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            // Text formats (RON, JSON) get the ultra-compact string
            let compact = match self.is_some() {
                true => format!(
                    "{}:{} @ {:x} ({}..{})",
                    self.line().unwrap(),
                    self.col().unwrap(),
                    self.source_hash,
                    self.start,
                    self.end
                ),
                false => String::from("None"),
            };
            serializer.serialize_str(&compact)
        } else {
            // Binary formats get zero-copy raw bytes via bytemuck
            let bytes: &[u8] = bytemuck::bytes_of(self);
            serializer.serialize_bytes(bytes)
        }
    }
}

#[test]
fn merge_all() {
    use std::ops::Range;
    assert_eq!(
        SrcRef::merge_all(
            [
                SrcRef::new(Range { start: 5, end: 8 }, LineCol { line: 1, col: 6 }, 123),
                SrcRef::new(
                    Range { start: 8, end: 10 },
                    LineCol { line: 2, col: 1 },
                    123
                ),
                SrcRef::new(
                    Range { start: 12, end: 16 },
                    LineCol { line: 3, col: 1 },
                    123
                ),
                SrcRef::new(
                    Range { start: 0, end: 10 },
                    LineCol { line: 1, col: 1 },
                    123
                ),
            ]
            .iter(),
        ),
        SrcRef::new(
            Range { start: 0, end: 16 },
            LineCol { line: 1, col: 1 },
            123
        ),
    );
}

#[test]
fn test_src_ref() {
    use microcad_core::hash::{ComputedHash, Hashed};
    let input = Hashed::new("geo3d::Cube(size_x = 3.0, size_y = 3.0, size_z = 3.0);");

    let cube = 7..11;
    let size_y = 26..32;
    let cube = SrcRef::new(cube, LineCol { line: 1, col: 0 }, input.computed_hash());
    let size_y = SrcRef::new(size_y, LineCol { line: 1, col: 0 }, input.computed_hash());

    assert_eq!(cube.source_slice(input.value()), "Cube");
    assert_eq!(size_y.source_slice(input.value()), "size_y");
}
