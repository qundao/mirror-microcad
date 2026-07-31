// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree iterators

use microcad_lang_base::HashId;

use crate::{ModelRef, element::ElementKind};

/// Children iterator struct.
pub struct Children<'tree> {
    model: ModelRef<'tree>,
    head: usize,
    tail: usize,
}

impl<'tree> Children<'tree> {
    /// Create children iterator from symbol.
    pub fn new(model: ModelRef<'tree>) -> Self {
        let len = model.children.items.len();
        Self {
            model,
            head: 0,
            tail: len,
        }
    }
}

impl<'tree> Iterator for Children<'tree> {
    type Item = ModelRef<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head < self.tail {
            let hash = self.model.children.items[self.head];
            self.head += 1;
            self.model.tree().get(hash)
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
            let hash = self.model.children.items[self.tail];
            self.model.tree().get(hash)
        } else {
            None
        }
    }
}

/// Iterator over all descendants.
pub struct UnnamedDescendants<'tree> {
    stack: Vec<ModelRef<'tree>>,
}

impl<'tree> UnnamedDescendants<'tree> {
    /// Create new descendants iterator
    pub fn new(model: ModelRef<'tree>) -> Self {
        Self {
            stack: vec![model].into(),
        }
    }
}

impl<'tree> Iterator for UnnamedDescendants<'tree> {
    type Item = ModelRef<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(model) = self.stack.pop() {
            self.stack
                .extend(model.children().filter(|model| !model.has_name()).rev());

            Some(model)
        } else {
            None
        }
    }
}

/// Iterator over all descendants of multiplicities.
pub struct UnnamedMultiplicityDescendants<'tree> {
    stack: Vec<ModelRef<'tree>>,
}

impl<'tree> UnnamedMultiplicityDescendants<'tree> {
    /// Create new descendants iterator
    pub fn new(model: ModelRef<'tree>) -> Self {
        Self {
            stack: vec![model].into(),
        }
    }
}

impl<'tree> Iterator for UnnamedMultiplicityDescendants<'tree> {
    type Item = ModelRef<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(model) = self.stack.pop() {
            if matches!(model.element().kind(), ElementKind::Multiplicity) {
                // Expand but don't yield this node itself
                self.stack
                    .extend(model.children().filter(|model| !model.has_name()).rev());
                continue;
            }
            // Return only non-multiplicity elements
            return Some(model.clone());
        }
        None
    }
}

/// Iterator over all parents of a [`Model`].
pub struct Parents<'tree> {
    model: Option<ModelRef<'tree>>,
}

impl<'tree> Parents<'tree> {
    /// New parents iterator
    pub fn new(model: ModelRef<'tree>) -> Self {
        Self { model: Some(model) }
    }
}

impl<'tree> Iterator for Parents<'tree> {
    type Item = ModelRef<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.model {
            Some(model) => {
                let parent = model.parent();
                self.model = parent;
                self.model.clone()
            }
            None => None,
        }
    }
}

/// Iterator over all ancestors (this model and its parents)
pub struct Ancestors<'tree> {
    model: Option<ModelRef<'tree>>,
}

impl<'tree> Ancestors<'tree> {
    /// New parents iterator
    pub fn new(model: ModelRef<'tree>) -> Self {
        Self { model: Some(model) }
    }
}

impl<'tree> Iterator for Ancestors<'tree> {
    type Item = ModelRef<'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        let model = match &self.model {
            Some(model) => model.clone(),
            None => return None,
        };

        self.model = model.parent();
        Some(model.clone())
    }
}

/// Iterator over all descendants.
pub struct SourceFileDescendants<'tree> {
    _stack: Vec<ModelRef<'tree>>,
    _source_hash_id: HashId,
}

/*
impl<'tree> SourceFileDescendants<'tree> {
    /// Create a new source file descendants.
    pub fn new(model: ModelRef<'tree>) -> Self {
        let source_hash = model.element().source_hash();

        Self {
            stack: root
                .borrow()
                .children
                .filter_by_source_hash(source_hash)
                .iter()
                .rev()
                .cloned()
                .collect(),
            source_hash,
        }
    }
}

impl Iterator for SourceFileDescendants {
    type Item = Model;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(model) = self.stack.pop() {
            let children = model
                .borrow()
                .children
                .filter_by_source_hash(self.source_hash);
            for child in children.iter().rev() {
                self.stack.push(child.clone());
            }

            Some(model)
        } else {
            None
        }
    }
}
*/
