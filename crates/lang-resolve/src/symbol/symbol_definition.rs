// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::From;
use microcad_lang_base::SrcReferrer;
use std::rc::Rc;

use crate::{builtin::*, symbol::defs};

/// Symbol definition
#[derive(Clone, Default, From)]
pub enum SymbolDef {
    /// Project's root symbol.
    #[default]
    Root,
    /// Source file symbol.
    SourceFile(Rc<defs::Source>),
    /// Module symbol.
    Module(Rc<defs::ModuleDefinition>),
    /// Part symbol.
    Workbench(Rc<defs::WorkbenchDefinition>),
    /// Function symbol.
    Function(Rc<defs::FunctionDefinition>),
    /// Assignment.
    Constant(defs::Constant),
    /// Builtin symbol.
    Builtin(Builtin),
    /// Alias of a pub use statement.
    Alias(Visibility, Identifier, QualifiedName),
    /// Use all available symbols in the module with the given name.
    UseAll(Visibility, QualifiedName),

    /// Just a dummy for testing
    #[cfg(test)]
    Tester(Identifier),
}

impl SymbolDef {
    /// Returns ID of this definition.
    pub fn id(&self) -> Identifier {
        match &self {
            Self::Root => Identifier::none(),
            Self::Workbench(w) => w.id(),
            Self::Module(m) => m.id(),
            Self::Function(f) => f.id(),
            Self::SourceFile(s) => s.id(),
            Self::Builtin(m) => m.id(),
            Self::Constant(c) => c.id(),
            Self::Alias(_, id, _) => id.clone(),
            Self::UseAll(..) => Identifier::none(),
            #[cfg(test)]
            Self::Tester(id) => id.clone(),
        }
    }

    /// Return visibility of this symbol.
    pub fn visibility(&self) -> Visibility {
        match &self {
            Self::Root => Visibility::Private,
            Self::SourceFile(..) | Self::Builtin(..) => Visibility::Public,

            Self::Module(md) => md.visibility.clone(),
            Self::Workbench(wd) => wd.visibility.clone(),
            Self::Function(fd) => fd.visibility.clone(),
            Self::Constant(c) => c.visibility.clone(),

            Self::Alias(visibility, ..) | Self::UseAll(visibility, ..) => visibility.clone(),

            #[cfg(test)]
            Self::Tester(..) => Visibility::Public,
        }
    }

    pub(crate) fn kind_str(&self) -> &'static str {
        match self {
            Self::Root => "root",
            Self::Workbench(w) => w.kind.as_str(),
            Self::Module(..) => "module",
            Self::Function(..) => "function",
            Self::SourceFile(..) => "source file",
            Self::Builtin(..) => "built-in",
            Self::Constant(..) => "constant",
            Self::Alias(..) => "alias",
            Self::UseAll(..) => "use-all",
            #[cfg(test)]
            Self::Tester(..) => "tester",
        }
    }

    pub(crate) fn source_hash(&self) -> u64 {
        match self {
            Self::Root => 0,
            Self::SourceFile(sf) => sf.source_hash(),
            Self::Module(md) => md.src_ref().source_hash(),
            Self::Workbench(wd) => wd.src_ref().source_hash(),
            Self::Function(fd) => fd.src_ref().source_hash(),
            Self::Builtin(_) => 0,
            Self::Constant(c) => c.src_ref.source_hash(),
            Self::Alias(_, id, _) => id.src_ref().source_hash(),
            Self::UseAll(_, name) => name.src_ref().source_hash(),
            #[cfg(test)]
            Self::Tester(..) => 0,
        }
    }
}

impl std::fmt::Display for SymbolDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = self.kind_str();
        match self {
            Self::Root => write!(f, "<ROOT>"),
            Self::Workbench(..)
            | Self::Module(..)
            | Self::Function(..)
            | Self::SourceFile(..)
            | Self::Builtin(..) => write!(f, "({kind})"),
            Self::Constant(c) => write!(f, "{c}"),
            Self::Alias(.., name) => write!(f, "({kind}) => {name}"),
            Self::UseAll(.., name) => write!(f, "({kind}) => {name}"),
            #[cfg(test)]
            Self::Tester(id) => write!(f, "(Tester) => {id}"),
        }
    }
}
