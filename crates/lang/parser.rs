// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Code Parser

#![allow(missing_docs)]

/// include grammar from file
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

use microcad_syntax::Span;
use crate::{parse::*, src_ref::*};

#[derive(Debug, Clone)]
pub struct Pair<'i>(pest::iterators::Pair<'i, Rule>, u64);

impl<'i> Pair<'i> {
    pub fn new(pest_pair: pest::iterators::Pair<'i, Rule>, source_hash: u64) -> Self {
        Self(pest_pair, source_hash)
    }

    pub fn source_hash(&self) -> u64 {
        self.1
    }

    pub fn set_source_hash(&mut self, hash: u64) {
        self.1 = hash
    }

    pub fn pest_pair(&self) -> &pest::iterators::Pair<'i, Rule> {
        &self.0
    }

    pub fn inner(&'i self) -> impl Iterator<Item = Self> {
        self.0.clone().into_inner().map(|p| Self(p, self.1))
    }

    /// Find an inner pair by rule
    pub fn find<T: Parse>(&'i self, rule: Rule) -> Option<T> {
        match self
            .inner()
            .find(|pair| pair.as_rule() == rule)
            .map(T::parse)
        {
            Some(Err(_)) | None => None,
            Some(Ok(x)) => Some(x),
        }
    }
}

impl SrcReferrer for Pair<'_> {
    fn src_ref(&self) -> SrcRef {
        let pair = &self.0;
        let (line, col) = pair.line_col();
        SrcRef::new(
            pair.as_span().start()..pair.as_span().end(),
            line,
            col,
            self.1,
        )
    }
}

impl<'i> std::ops::Deref for Pair<'i> {
    type Target = pest::iterators::Pair<'i, Rule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Pairs<'i> = pest::iterators::Pairs<'i, Rule>;

#[derive(Clone)]
pub struct LineIndex {
    /// Offset (bytes) the beginning of each line, zero-based
    line_offsets: Vec<usize>,
}

impl LineIndex {
    pub fn new(text: &str) -> LineIndex {
        let mut line_offsets: Vec<usize> = vec![0];

        let mut offset = 0;

        for c in text.chars() {
            offset += c.len_utf8();
            if c == '\n' {
                line_offsets.push(offset);
            }
        }

        LineIndex { line_offsets }
    }

    /// Returns (line, col) of pos.
    ///
    /// The pos is a byte offset, start from 0, e.g. "ab" is 2, "你好" is 6
    pub fn line_col(&self, input: &str, pos: usize) -> (usize, usize) {
        let line = self.line_offsets.partition_point(|&it| it <= pos) - 1;
        let first_offset = self.line_offsets[line];

        // Get line str from original input, then we can get column offset
        let line_str = &input[first_offset..pos];
        let col = line_str.chars().count();

        (line + 1, col + 1)
    }
}

pub struct ParseContext<'source> {
    pub source: &'source str,
    pub source_file_hash: u64,
    line_index: LineIndex,
}

impl<'source> ParseContext<'source> {
    pub fn new(source: &'source str) -> Self {
        let source_file_hash = {
            use std::hash::{Hash, Hasher};
            let mut hasher = rustc_hash::FxHasher::default();
            source.hash(&mut hasher);
            hasher.finish()
        };
        ParseContext {
            source,
            source_file_hash,
            line_index: LineIndex::new(source),
        }
    }

    pub fn src_ref(&self, span: &Span) -> SrcRef {
        let (line, col) = self.line_index.line_col(self.source, span.start);
        SrcRef::new(span.clone(), line, col, self.source_file_hash)
    }
}

pub trait FromAst: Sized {
    type AstNode;

    fn from_ast(node: &Self::AstNode, context: &ParseContext) -> Result<Self, ParseError>;
}

pub trait Parse: Sized {
    fn parse(pair: Pair) -> ParseResult<Self>;
}

impl Parser {
    /// Helper function to parse a vector of pairs into a vector of T
    ///
    /// # Arguments
    ///
    /// - `pairs`: The pairs to parse
    /// - `f`: The function to parse the pair into `T`
    ///
    /// Returns a vector of `T`
    pub fn vec<'a, T>(pair: Pair<'a>, f: impl Fn(Pair<'a>) -> ParseResult<T>) -> ParseResult<Vec<T>>
    where
        T: Clone,
    {
        pair.0.into_inner().map(|p| f(Pair(p, pair.1))).collect()
    }

    /// Parse a rule for type `T`
    pub fn parse_rule<T>(rule: Rule, input: &str, src_hash: u64) -> ParseResult<T>
    where
        T: Parse + Clone,
    {
        use pest::Parser as _;

        match Parser::parse(rule, input.trim()) {
            Ok(mut pairs) => match pairs.next() {
                Some(pair) => Ok(T::parse(Pair(pair, src_hash))?),
                None => Err(ParseError::RuleNotFoundError(Box::new(rule))),
            },
            Err(err) => Err(ParseError::Parser(err.into())),
        }
    }

    pub fn ensure_rule(pair: &Pair, expected: Rule) {
        let rule = pair.as_rule();
        assert_eq!(rule, expected, "Unexpected rule: {rule:?}");
    }

    pub fn ensure_rules(pair: &Pair, rules: &[Rule]) {
        for rule in rules {
            if *rule == pair.as_rule() {
                return;
            }
        }

        panic!(
            "Unexpected rules: expected {rules:?}, got {rule:?}",
            rule = pair.as_rule()
        );
    }
}
