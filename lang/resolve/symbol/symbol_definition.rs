// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{builtin::*, rc::*, resolve::*, src_ref::*, syntax::*, value::*};

/// Symbol definition
#[derive(Clone)]
pub enum SymbolDef {
    /// Source file symbol.
    SourceFile(Rc<SourceFile>),
    /// Module symbol.
    Module(Rc<ModuleDefinition>),
    /// Part symbol.
    Workbench(Rc<WorkbenchDefinition>),
    /// Function symbol.
    Function(Rc<FunctionDefinition>),
    /// Assignment.
    Assignment(Rc<Assignment>),
    /// Builtin symbol.
    Builtin(Rc<Builtin>),
    /// Constant.
    Constant(Visibility, Identifier, Value),
    /// Argument value.
    Argument(Identifier, Value),
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
            Self::Workbench(w) => w.id.clone(),
            Self::Module(m) => m.id.clone(),
            Self::Function(f) => f.id.clone(),
            Self::SourceFile(s) => s.id(),
            Self::Builtin(m) => m.id(),
            Self::Assignment(a) => a.id.clone(),
            Self::Constant(_, id, _) | Self::Argument(id, _) | Self::Alias(_, id, _) => id.clone(),
            Self::UseAll(..) => Identifier::none(),
            #[cfg(test)]
            Self::Tester(id) => id.clone(),
        }
    }

    /// Return visibility of this symbol.
    pub fn visibility(&self) -> Visibility {
        match &self {
            Self::SourceFile(..) | Self::Builtin(..) => Visibility::Public,

            Self::Argument(..) => Visibility::Private,

            Self::Constant(visibility, ..) => visibility.clone(),
            Self::Module(md) => md.visibility.clone(),
            Self::Workbench(wd) => wd.visibility.clone(),
            Self::Function(fd) => fd.visibility.clone(),
            Self::Assignment(a) => a.visibility.clone(),

            Self::Alias(visibility, ..) | Self::UseAll(visibility, ..) => visibility.clone(),

            #[cfg(test)]
            Self::Tester(..) => Visibility::Public,
        }
    }

    pub(crate) fn kind_str(&self) -> String {
        match self {
            Self::Workbench(w) => format!("{}", w.kind),
            Self::Module(..) => "Module".to_string(),
            Self::Function(..) => "Function".to_string(),
            Self::SourceFile(..) => "SourceFile".to_string(),
            Self::Builtin(b) => format!("{}", b.kind),
            Self::Constant(..) => "Constant".to_string(),
            Self::Assignment(..) => "Assignment".to_string(),
            Self::Argument(..) => "Argument".to_string(),
            Self::Alias(..) => "Alias".to_string(),
            Self::UseAll(..) => "UseAll".to_string(),
            #[cfg(test)]
            Self::Tester(..) => "Tester".to_string(),
        }
    }

    pub(crate) fn source_hash(&self) -> u64 {
        match self {
            Self::SourceFile(sf) => sf.hash,
            Self::Module(md) => md.src_ref.source_hash(),
            Self::Workbench(wd) => wd.src_ref.source_hash(),
            Self::Function(fd) => fd.src_ref.source_hash(),
            Self::Builtin(_) => 0,
            Self::Assignment(a) => a.src_ref.source_hash(),
            Self::Constant(_, id, _) | Self::Argument(id, _) | Self::Alias(_, id, _) => {
                id.src_ref().source_hash()
            }
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
            Self::Workbench(..)
            | Self::Module(..)
            | Self::Function(..)
            | Self::SourceFile(..)
            | Self::Builtin(..) => write!(f, "({kind})"),
            Self::Constant(.., value) => write!(f, "({kind}) = {value}"),
            Self::Assignment(.., value) => write!(f, "({kind}) = {value}"),
            Self::Argument(.., value) => write!(f, "({kind}) = {value}"),
            Self::Alias(.., name) => write!(f, "({kind}) => {name}"),
            Self::UseAll(.., name) => write!(f, "({kind}) => {name}"),
            #[cfg(test)]
            Self::Tester(id) => write!(f, "(Tester) => {id}"),
        }
    }
}

impl std::fmt::Debug for SymbolDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = self.kind_str();
        match self {
            Self::Workbench(..)
            | Self::Module(..)
            | Self::Function(..)
            | Self::SourceFile(..)
            | Self::Builtin(..) => write!(f, "({kind})"),
            Self::Constant(.., value) => write!(f, "({kind}) = {value}"),
            Self::Assignment(.., expr) => write!(f, "({kind}) = {expr:?}"),
            Self::Argument(.., value) => write!(f, "({kind}) = {value}"),
            Self::Alias(.., name) => write!(f, "({kind}) => {name:?}"),
            Self::UseAll(.., name) => write!(f, "({kind}) => {name:?}"),
            #[cfg(test)]
            Self::Tester(id) => write!(f, "({kind}) => {id:?}"),
        }
    }
}

impl Doc for SymbolDef {
    fn doc(&self) -> Option<DocBlock> {
        match self {
            SymbolDef::SourceFile(sf) => sf.doc(),
            SymbolDef::Module(md) => md.doc(),
            SymbolDef::Workbench(wd) => wd.doc(),
            SymbolDef::Function(fd) => fd.doc(),
            _ => None,
        }
    }
}

impl Info for SymbolDef {
    fn info(&self) -> SymbolInfo {
        match self {
            SymbolDef::SourceFile(sf) => sf.into(),
            SymbolDef::Module(md) => md.into(),
            SymbolDef::Workbench(wd) => wd.into(),
            SymbolDef::Function(fd) => fd.into(),
            SymbolDef::Builtin(bi) => bi.into(),
            SymbolDef::Assignment(a) => a.into(),

            SymbolDef::Constant(visibility, id, value) => {
                SymbolInfo::new_constant(visibility, id, value)
            }
            SymbolDef::Argument(id, value) => SymbolInfo::new_arg(id, value),

            SymbolDef::Alias(..) => unimplemented!(),
            SymbolDef::UseAll(..) => unimplemented!(),

            #[cfg(test)]
            SymbolDef::Tester(_) => unreachable!(),
        }
    }
}
