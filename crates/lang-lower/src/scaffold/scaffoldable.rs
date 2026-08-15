// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::ir;
use crate::scaffold::Scaffold;

/// This traits returns an iterator over all items that will eventually produce a Symbol.
pub trait Scaffoldables {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold>;
}

impl<T> Scaffoldables for Box<[T]>
where
    T: Scaffold,
{
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.iter().map(|i| i as &dyn Scaffold)
    }
}

impl Scaffoldables for ir::desugared::Aliases {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.explicit_aliases
            .scaffoldables()
            .chain(self.wildcards.scaffoldables())
    }
}

impl Scaffoldables for ir::desugared::FunctionItems {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.aliases
            .scaffoldables()
            .chain(self.constants.scaffoldables())
    }
}

impl Scaffoldables for ir::desugared::WorkbenchItems {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.aliases
            .scaffoldables()
            .chain(self.constants.scaffoldables())
            .chain(self.functions.scaffoldables())
    }
}

impl Scaffoldables for ir::desugared::InlineModuleItems {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.aliases
            .scaffoldables()
            .chain(self.constants.scaffoldables())
            .chain(self.modules.scaffoldables())
            .chain(self.functions.scaffoldables())
            .chain(self.workbenches.scaffoldables())
    }
}

impl Scaffoldables for ir::desugared::SourceItems {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.file_modules
            .scaffoldables()
            .chain(self.aliases.scaffoldables())
            .chain(self.constants.scaffoldables())
            .chain(self.inline_modules.scaffoldables())
            .chain(self.functions.scaffoldables())
            .chain(self.workbenches.scaffoldables())
    }
}
