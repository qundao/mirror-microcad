// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Library visitors

use microcad_lang_lower::ir::Meta;
pub use microcad_lang_lower::ir::visitor::{
    ConstantVisitor, ConstantVisitorMut, FnVisitor, FnVisitorMut, LeafVisitor, LeafVisitorMut,
    SourceVisitor, SourceVisitorMut, WorkbenchExpressionVisitor, WorkbenchExpressionVisitorMut,
    WorkbenchVisitor, WorkbenchVisitorMut,
};

use crate::{
    Library, SymbolDef, SymbolNodeRef,
    library::{Symbol, symbol},
};

/// Visitor for a Library tree.
pub trait Visitor: SourceVisitor + FnVisitor + WorkbenchVisitor + ConstantVisitor {
    fn visit(&mut self, library: &Library) {
        library
            .root()
            .descendants()
            .for_each(|node| self.visit_node(node));
    }

    fn visit_node<'a>(&mut self, node: SymbolNodeRef<'a>) {
        self.visit_symbol(node.get());
    }

    fn visit_symbol(&mut self, symbol: &Symbol) {
        self.visit_meta(&symbol.meta);
        self.visit_def(&symbol.def);
    }

    fn visit_file_path(&mut self, _path: &std::path::Path) {}

    fn visit_meta(&mut self, _meta: &Meta) {}
    fn visit_inline_module(&mut self, _inline_module: &symbol::InlineModule) {}
    fn visit_file_module(&mut self, file_module: &symbol::FileModule) {
        match file_module {
            symbol::FileModule::NotLoaded => {}
            symbol::FileModule::Loaded { path, source } => {
                self.visit_file_path(path);
                self.visit_source(source);
            }
        }
    }

    fn visit_alias(&mut self, alias: &symbol::Alias) {
        self.visit_path(&alias.path);
    }
    fn visit_wildcard(&mut self, wildcard: &symbol::Wildcard) {
        self.visit_path(&wildcard.path);
    }

    fn visit_def(&mut self, def: &SymbolDef) {
        use SymbolDef::*;
        match def {
            Root(library_root) => match &library_root.entry {
                Some(entry) => self.visit_source(entry),
                None => {}
            },
            Source(source) => {
                self.visit_source(source);
            }
            InlineModule(inline_module) => self.visit_inline_module(inline_module),
            FileModule(file_module) => self.visit_file_module(file_module),
            Workbench(workbench) => self.visit_workbench(workbench),
            Function(function) => self.visit_fn(function),
            Constant(constant) => self.visit_constant(constant),
            Alias(alias) => self.visit_alias(alias),
            Wildcard(wildcard) => self.visit_wildcard(wildcard),
        }
    }
}
