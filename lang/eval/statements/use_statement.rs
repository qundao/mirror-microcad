// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

/// Trait to use symbols on the [Stack].
pub trait UseLocally {
    /// Find a symbol in the symbol table and copy it to the locals.
    ///
    /// Might load any related external file if not already loaded.
    ///
    /// # Arguments
    /// - `name`: Name of the symbol to search for
    /// - `id`: if given overwrites the ID from qualified name (use as)
    fn use_symbol(&mut self, name: &QualifiedName, id: Option<Identifier>) -> EvalResult<Symbol>;

    /// Find a symbol and copy all it's children to the locals.
    ///
    /// Might load any related external file if not already loaded.
    ///
    /// # Arguments
    /// - `name`: Name of the symbol to search for
    fn use_symbols_of(&mut self, name: &QualifiedName) -> EvalResult<Symbol>;
}

impl Eval<()> for UseStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<()> {
        self.grant(context)?;

        if !context.is_module() {
            log::trace!("Evaluating use statement: {self}");
            match &self.decl {
                UseDeclaration::Use(name) => {
                    if let Err(err) = context.use_symbol(name, None) {
                        context.error(name, err)?;
                    }
                }
                UseDeclaration::UseAll(name) => {
                    if let Err(err) = context.use_symbols_of(name) {
                        context.error(name, err)?
                    }
                }
                UseDeclaration::UseAlias(name, alias) => {
                    if let Err(err) = context.use_symbol(name, Some(alias.clone())) {
                        context.error(name, err)?;
                    }
                }
            }
        }
        Ok(())
    }
}
