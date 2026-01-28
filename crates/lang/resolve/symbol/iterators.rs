// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbol iterators

use crate::resolve::*;

/// Iterator over children of a symbol.
pub struct Children {
    symbol: Symbol,
    index: usize,
}

impl Children {
    /// Create children iterator from symbol.
    pub fn new(symbol: Symbol) -> Self {
        Self { symbol, index: 0 }
    }
}

impl Iterator for Children {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        let symbol = self.symbol.inner.borrow();
        let child = symbol
            .children
            .get_index(self.index)
            .map(|(_, child)| child);
        self.index += 1;
        child.cloned()
    }
}

/// Iterator that recursively iterates over children of a symbol.
pub struct RecurseChildren {
    stack: Symbols,
}

impl RecurseChildren {
    /// Create recursive children iterator from symbol (inluding symbol itself).
    pub(crate) fn new(symbol: Symbol) -> Self {
        Self {
            stack: vec![symbol].into(),
        }
    }
}

impl Iterator for RecurseChildren {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(symbol) = self.stack.pop() {
            self.stack
                .extend(symbol.inner.borrow().children.values().rev().cloned());

            Some(symbol)
        } else {
            None
        }
    }
}

#[test]
fn test_recurse_children() {
    use crate::rc::*;

    let mut root = Symbol::new(
        SymbolDef::SourceFile(Rc::new(SourceFile::new(
            None,
            StatementList::default(),
            String::new(),
            0,
        ))),
        None,
    );

    let mut foo = Symbol::new(
        SymbolDef::Tester(Identifier::no_ref("foo")),
        Some(root.clone()),
    );
    {
        let mut baz = Symbol::new(
            SymbolDef::Tester(Identifier::no_ref("baz")),
            Some(foo.clone()),
        );
        {
            let bam = Symbol::new(
                SymbolDef::Tester(Identifier::no_ref("bam")),
                Some(baz.clone()),
            );
            baz.add_symbol(bam).expect("test error");
        }

        foo.add_symbol(baz).expect("test error");
    }
    root.add_symbol(foo).expect("test error");

    let bar = Symbol::new(
        SymbolDef::Tester(Identifier::no_ref("bar")),
        Some(root.clone()),
    );
    root.add_symbol(bar).expect("test error");

    let s = root
        .riter()
        .map(|symbol| format!("{}", symbol.id()))
        .collect::<Vec<_>>()
        .join(" ");

    assert_eq!(s, "<NO ID> foo baz bam bar");
}
