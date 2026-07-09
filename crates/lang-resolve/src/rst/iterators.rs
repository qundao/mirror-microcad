// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol iterators

use crate::{Symbol, rst::SymbolRef};

/// Iterator over children of a symbol.
pub struct Children<'rst> {
    symbol: SymbolRef<'rst>,
    // Store the bounds of the children to allow bidirectional movement
    head: usize,
    tail: usize,
}

impl<'rst> Children<'rst> {
    /// Create children iterator from symbol.
    pub fn new(symbol: SymbolRef<'rst>) -> Self {
        let len = symbol.children.items.len();
        Self {
            symbol,
            head: 0,
            tail: len,
        }
    }
}

impl<'rst> Iterator for Children<'rst> {
    type Item = SymbolRef<'rst>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head < self.tail {
            let hash = self.symbol.children.items[self.head];
            self.head += 1;
            self.symbol.rst.get(hash)
        } else {
            None
        }
    }
}

// Implement DoubleEndedIterator for .rev() support
impl<'rst> DoubleEndedIterator for Children<'rst> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.head < self.tail {
            self.tail -= 1;
            let hash = self.symbol.children.items[self.tail];
            self.symbol.rst.get(hash)
        } else {
            None
        }
    }
}

/// Iterator that recursively iterates over children of a symbol, including the symbol itself.
pub struct Descendants<'rst> {
    stack: Vec<SymbolRef<'rst>>,
}

impl<'rst> Descendants<'rst> {
    /// Create recursive children iterator from symbol (including symbol itself).
    pub fn new(symbol: SymbolRef<'rst>) -> Self {
        Self {
            stack: vec![symbol].into(),
        }
    }
}

impl<'rst> Iterator for Descendants<'rst> {
    type Item = SymbolRef<'rst>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(symbol) = self.stack.pop() {
            self.stack.extend(symbol.children().rev());

            Some(symbol)
        } else {
            None
        }
    }
}
