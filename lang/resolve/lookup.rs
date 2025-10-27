// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{resolve::*, syntax::*};

/// Kind of symbol to look up.
#[derive(Clone, Copy)]
pub enum LookupTarget {
    /// Lookup for any symbol
    Any,

    /// Lookup for methods only
    Method,
    /// Lookup for functions only
    Function,
    /// Lookup for modules only
    Module,
    /// Lookup for constants and const expressions only
    Value,
    /// Lookup for use-all and aliases only
    Link,
}

impl LookupTarget {
    pub(super) fn matches(&self, symbol: &Symbol) -> bool {
        symbol.with_def(|def| -> bool {
            match &def {
                SymbolDef::SourceFile(..) | SymbolDef::Module(..) => {
                    matches!(self, Self::Any | Self::Module)
                }
                SymbolDef::Workbench(wd) => match *wd.kind {
                    WorkbenchKind::Part | WorkbenchKind::Sketch => {
                        matches!(self, Self::Any | Self::Function)
                    }
                    WorkbenchKind::Operation => matches!(self, Self::Any | Self::Method),
                },
                SymbolDef::Function(..) => {
                    matches!(self, Self::Any | Self::Function)
                }
                SymbolDef::Builtin(b) => match &b.kind {
                    crate::builtin::BuiltinKind::Function => {
                        matches!(self, Self::Any | Self::Function)
                    }
                    crate::builtin::BuiltinKind::Workbench(bwk) => match bwk {
                        crate::builtin::BuiltinWorkbenchKind::Primitive2D
                        | crate::builtin::BuiltinWorkbenchKind::Primitive3D => {
                            matches!(self, Self::Any | Self::Function)
                        }
                        crate::builtin::BuiltinWorkbenchKind::Transform
                        | crate::builtin::BuiltinWorkbenchKind::Operation => {
                            matches!(self, Self::Any | Self::Method)
                        }
                    },
                },
                SymbolDef::Constant(..)
                | SymbolDef::ConstExpression(..)
                | SymbolDef::Argument(..) => {
                    matches!(self, Self::Any | Self::Value)
                }
                SymbolDef::Alias(..) | SymbolDef::UseAll(..) => {
                    matches!(self, Self::Any | Self::Link)
                }
                #[cfg(test)]
                SymbolDef::Tester(..) => todo!(),
            }
        })
    }
}

/// Trait to lookup symbols by *qualified name*.
pub trait Lookup<E: std::error::Error = ResolveError> {
    /// Search a *symbol* by it's *qualified name*.
    /// # Arguments
    /// - `name`: *Qualified name* to search for.
    /// - `kind`: What to search for
    fn lookup(&self, name: &QualifiedName, kind: LookupTarget) -> Result<Symbol, E>;

    /// Return an ambiguity error.
    fn ambiguity_error(ambiguous: QualifiedName, others: QualifiedNames) -> E;

    /// Search a *symbol* by it's *qualified name* **and** within the given *symbol*.
    ///
    /// # Arguments
    /// - `name`: *Qualified name* to search for.
    /// - `within`: Searches within this *symbol* too.
    /// - `kind`: What to search for
    /// # Return
    /// If both are found and one is an *alias* returns the other one.
    fn lookup_within(
        &self,
        name: &QualifiedName,
        within: &Symbol,
        kind: LookupTarget,
    ) -> Result<Symbol, E> {
        log::trace!(
            "{lookup} for symbol '{name:?}' within '{within}'",
            within = within.full_name(),
            lookup = crate::mark!(LOOKUP)
        );
        match (self.lookup(name, kind), within.search(name, true)) {
            // found both
            (Ok(global), Ok(relative)) => {
                // check if one is an alias of the other
                match (global.is_alias(), relative.is_alias()) {
                    (true, false) => Ok(relative),
                    (false, true) => Ok(global),
                    (true, true) => unreachable!("found two aliases"),
                    (false, false) => {
                        if relative == global {
                            Ok(global)
                        } else {
                            Err(Self::ambiguity_error(
                                relative.full_name(),
                                [global.full_name()].into_iter().collect(),
                            ))
                        }
                    }
                }
            }
            // found one
            (Ok(symbol), Err(_)) | (Err(_), Ok(symbol)) => {
                log::trace!(
                    "{found} symbol '{name:?}' within '{within}'",
                    within = within.full_name(),
                    found = crate::mark!(FOUND_INTERIM)
                );
                Ok(symbol)
            }
            // found nothing
            (Err(err), Err(_)) => {
                log::trace!(
                    "{not_found} symbol '{name:?}' within '{within}'",
                    within = within.full_name(),
                    not_found = crate::mark!(NOT_FOUND_INTERIM)
                );
                Err(err)
            }
        }
    }

    /// Search a *symbol* by it's *qualified name* **and** within a given *symbol*
    ///
    /// # Arguments
    /// - `name`: *qualified name* to search for
    /// - `within`: If some, searches within this *symbol* too.
    /// - `kind`: What to search for
    /// # Return
    /// If both are found and one is an *alias* returns the other one.
    fn lookup_within_opt(
        &self,
        name: &QualifiedName,
        within: &Option<Symbol>,
        kind: LookupTarget,
    ) -> Result<Symbol, E> {
        if let Some(within) = within {
            self.lookup_within(name, within, kind)
        } else {
            self.lookup(name, kind)
        }
    }

    /// Search a *symbol* by it's *qualified name* **and** within a *symbol* given by name.
    ///
    /// If both are found
    /// # Arguments
    /// - `name`: *qualified name* to search for.
    /// - `within`: Searches in the *symbol* with this name too.
    /// - `kind`: What to search for
    fn lookup_within_name(
        &self,
        name: &QualifiedName,
        within: &QualifiedName,
        kind: LookupTarget,
    ) -> Result<Symbol, E> {
        self.lookup_within(name, &self.lookup(within, kind)?, kind)
    }

    /// Returns an error if name starts with `super::`.
    fn deny_super(&self, name: &QualifiedName) -> ResolveResult<()> {
        if name.count_super() > 0 {
            log::trace!(
                "{not_found} '{name:?}' is not canonical",
                not_found = crate::mark!(NOT_FOUND_INTERIM),
            );
            return Err(ResolveError::SymbolNotFound(name.clone()));
        }
        Ok(())
    }
}
