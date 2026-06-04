// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbolizing of syntax definitions.

use std::rc::Rc;

use microcad_lang_base::PushDiag;

use crate::{
    lower::ir,
    resolve::*,
    symbol::{Symbol, SymbolDef, SymbolMap},
};

pub(super) trait Symbolize<T = Option<Symbol>> {
    /// Create symbol from definition.
    fn symbolize(&self, _parent: &Symbol, _context: &mut ResolveContext) -> ResolveResult<T> {
        unreachable!()
    }
}

impl ir::Source {
    /// Create symbol from definition.
    pub fn symbolize(
        &self,
        visibility: ir::Visibility,
        context: &mut ResolveContext,
    ) -> ResolveResult<Symbol> {
        let symbol = Symbol::new_with_visibility(
            visibility,
            SymbolDef::SourceFile(self.clone().into()),
            None,
        );
        symbol.set_children(self.statements.symbolize(&symbol, context)?);
        log::trace!("Granting {}", self.name);
        self.grant(&mut GrantContext::new(context))?;
        Ok(symbol)
    }
}

impl Symbolize<Symbol> for ir::ModuleDefinition {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<Symbol> {
        use crate::Identifiable;

        let symbol = if let Some(body) = &self.body {
            let symbol = Symbol::new(SymbolDef::Module(self.clone().into()), Some(parent.clone()));
            symbol.set_children(body.symbolize(&symbol, context)?);
            symbol
        } else if let Some(parent_path) = parent.source_path() {
            let mut symbol =
                context.symbolize_file(self.visibility.clone(), parent_path, self.id_ref())?;
            symbol.set_parent(parent.clone());
            symbol
        } else {
            todo!("no top-level source file")
        };
        Ok(symbol)
    }
}

impl Symbolize<SymbolMap> for ir::StatementList {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        let mut symbols = SymbolMap::default();
        // Iterate over all statement fetch definitions
        for statement in &self.0 {
            if let Some((id, symbol)) = statement.symbolize(parent, context)? {
                if let Some(alt) = symbols.insert(id.clone(), symbol) {
                    context.error(
                        &id,
                        ResolveError::AmbiguousId {
                            first: alt.id(),
                            ambiguous: id.clone(),
                        },
                    )?;
                }
            }
        }
        Ok(symbols)
    }
}

impl Symbolize<Option<(ir::Identifier, Symbol)>> for ir::Statement {
    fn symbolize(
        &self,
        parent: &Symbol,
        context: &mut ResolveContext,
    ) -> ResolveResult<Option<(ir::Identifier, Symbol)>> {
        use crate::Identifiable;
        use ir::Statement::*;
        match self {
            Workbench(wd) => Ok(Some((wd.id(), wd.symbolize(parent, context)?))),
            Module(md) => Ok(Some((md.id(), md.symbolize(parent, context)?))),
            Function(fd) => Ok(Some((fd.id(), fd.symbolize(parent, context)?))),
            Use(us) => us.symbolize(parent, context),
            LocalAssignment(a) => Ok(a
                .symbolize(parent, context)?
                .map(|symbol| (a.assignment.id(), symbol))),
            // Not producing any symbols
            Init(_) | Return(_) | If(_) | InnerAttribute(_) | InnerDocComment(_)
            | Expression(_) => Ok(None),
        }
    }
}

impl Symbolize<Symbol> for Rc<ir::WorkbenchDefinition> {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(SymbolDef::Workbench(self.clone()), Some(parent.clone()));
        symbol.set_children(self.body.symbolize(&symbol, context)?);
        Ok(symbol)
    }
}

impl Symbolize<Symbol> for Rc<ir::FunctionDefinition> {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(SymbolDef::Function(self.clone()), Some(parent.clone()));
        symbol.set_children(self.body.symbolize(&symbol, context)?);
        Ok(symbol)
    }
}

impl Symbolize<Symbol> for ir::Constant {
    fn symbolize(&self, parent: &Symbol, _context: &mut ResolveContext) -> ResolveResult<Symbol> {
        Ok(Symbol::new(
            SymbolDef::Constant(self.clone()),
            Some(parent.clone()),
        ))
    }
}

impl Symbolize<SymbolMap> for ir::Body {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        self.statements.symbolize(parent, context)
    }
}

impl Symbolize<Option<(ir::Identifier, Symbol)>> for ir::UseStatement {
    fn symbolize(
        &self,
        parent: &Symbol,
        _: &mut ResolveContext,
    ) -> ResolveResult<Option<(ir::Identifier, Symbol)>> {
        match &self.decl {
            ir::UseDeclaration::Use(name) => {
                let identifier = name.last().expect("Identifier");
                Ok(Some((
                    ir::Identifier::unique(),
                    Symbol::new(
                        SymbolDef::Alias(self.visibility.clone(), identifier.clone(), name.clone()),
                        Some(parent.clone()),
                    ),
                )))
            }
            ir::UseDeclaration::UseAll(name) => Ok(Some((
                ir::Identifier::unique(),
                Symbol::new(
                    SymbolDef::UseAll(self.visibility.clone(), name.clone()),
                    Some(parent.clone()),
                ),
            ))),
            ir::UseDeclaration::UseAs(name, alias) => Ok(Some((
                ir::Identifier::unique(),
                Symbol::new(
                    SymbolDef::Alias(self.visibility.clone(), alias.clone(), name.clone()),
                    Some(parent.clone()),
                ),
            ))),
        }
    }
}
