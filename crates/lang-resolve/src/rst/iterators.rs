// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol iterators

use serde::Serialize;

use crate::{Symbol, rst::SymbolRef};

/// Iterator over children of a symbol.
pub struct Children<'tree, DEF: Serialize> {
    symbol: SymbolRef<'tree, DEF>,
    // Store the bounds of the children to allow bidirectional movement
    head: usize,
    tail: usize,
}

impl<'tree, DEF: Serialize> Children<'tree, DEF> {
    /// Create children iterator from symbol.
    pub fn new(symbol: SymbolRef<'tree, DEF>) -> Self {
        let len = symbol.children.items.len();
        Self {
            symbol,
            head: 0,
            tail: len,
        }
    }
}

impl<'tree, DEF: Serialize> Iterator for Children<'tree, DEF> {
    type Item = SymbolRef<'tree, DEF>;

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
impl<'tree, DEF: Serialize> DoubleEndedIterator for Children<'tree, DEF> {
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
pub struct Descendants<'tree, DEF: Serialize> {
    stack: Vec<SymbolRef<'tree, DEF>>,
}

impl<'tree, DEF: Serialize> Descendants<'tree, DEF> {
    /// Create recursive children iterator from symbol (including symbol itself).
    pub fn new(symbol: SymbolRef<'tree, DEF>) -> Self {
        Self {
            stack: vec![symbol].into(),
        }
    }
}

impl<'tree, DEF: Serialize> Iterator for Descendants<'tree, DEF> {
    type Item = SymbolRef<'tree, DEF>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(symbol) = self.stack.pop() {
            self.stack.extend(symbol.children().rev());

            Some(symbol)
        } else {
            None
        }
    }
}
