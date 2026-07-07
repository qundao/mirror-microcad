// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol iterators

use crate::{Symbol, symbol::SymbolRef};

/// Iterator over children of a symbol.
pub struct Children<'tree> {
    symbol: SymbolRef<'tree>,
    // Store the bounds of the children to allow bidirectional movement
    head: usize,
    tail: usize,
}

impl<'tree> Children<'tree> {
    /// Create children iterator from symbol.
    pub fn new(symbol: SymbolRef<'tree>) -> Self {
        let len = symbol.children.items.len();
        Self {
            symbol,
            head: 0,
            tail: len,
        }
    }
}

impl<'tree> Iterator for Children<'tree> {
    type Item = SymbolRef<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head < self.tail {
            let hash = self.symbol.children.items[self.head];
            self.head += 1;
            self.symbol.tree.get(hash)
        } else {
            None
        }
    }
}

// Implement DoubleEndedIterator for .rev() support
impl<'tree> DoubleEndedIterator for Children<'tree> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.head < self.tail {
            self.tail -= 1;
            let hash = self.symbol.children.items[self.tail];
            self.symbol.tree.get(hash)
        } else {
            None
        }
    }
}

/// Iterator that recursively iterates over children of a symbol, including the symbol itself.
pub struct Descendants<'tree> {
    stack: Vec<SymbolRef<'tree>>,
}

impl<'tree> Descendants<'tree> {
    /// Create recursive children iterator from symbol (including symbol itself).
    pub fn new(symbol: SymbolRef<'tree>) -> Self {
        Self {
            stack: vec![symbol].into(),
        }
    }
}

impl<'tree> Iterator for Descendants<'tree> {
    type Item = SymbolRef<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(symbol) = self.stack.pop() {
            self.stack.extend(symbol.children().rev());

            Some(symbol)
        } else {
            None
        }
    }
}
