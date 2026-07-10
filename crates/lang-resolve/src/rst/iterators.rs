// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol iterators

use serde::Serialize;

use crate::{Symbol, rst::SymbolRef};

/// Iterator over children of a symbol.
pub struct Children<'rst, DATA: Serialize> {
    symbol: SymbolRef<'rst, DATA>,
    // Store the bounds of the children to allow bidirectional movement
    head: usize,
    tail: usize,
}

impl<'rst, DATA: Serialize> Children<'rst, DATA> {
    /// Create children iterator from symbol.
    pub fn new(symbol: SymbolRef<'rst, DATA>) -> Self {
        let len = symbol.children.items.len();
        Self {
            symbol,
            head: 0,
            tail: len,
        }
    }
}

impl<'rst, DATA: Serialize> Iterator for Children<'rst, DATA> {
    type Item = SymbolRef<'rst, DATA>;

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
impl<'rst, DATA: Serialize> DoubleEndedIterator for Children<'rst, DATA> {
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
pub struct Descendants<'rst, DATA: Serialize> {
    stack: Vec<SymbolRef<'rst, DATA>>,
}

impl<'rst, DATA: Serialize> Descendants<'rst, DATA> {
    /// Create recursive children iterator from symbol (including symbol itself).
    pub fn new(symbol: SymbolRef<'rst, DATA>) -> Self {
        Self {
            stack: vec![symbol].into(),
        }
    }
}

impl<'rst, DATA: Serialize + Clone> Iterator for Descendants<'rst, DATA> {
    type Item = SymbolRef<'rst, DATA>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(symbol) = self.stack.pop() {
            self.stack.extend(symbol.children().rev());

            Some(symbol)
        } else {
            None
        }
    }
}
