// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Check wether a statement is legally placed.

use crate::{resolve::*, syntax::*};

pub(super) trait Grant {
    /// Checks if definition is allowed to occur within the given parent symbol.
    fn grant(&self, _parent: &Symbol, _context: &mut ResolveContext) -> DiagResult<&Self> {
        Ok(self)
    }

    fn kind(&self) -> &'static str;

    fn allowed_parents(&self) -> &'static [&'static str];
}

impl Grant for ModuleDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) => Ok(()),
            _ => context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            ),
        })?;
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        "module definition"
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        &["source root", "module"]
    }
}

impl Grant for StatementList {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..)
            | SymbolDef::Module(..)
            | SymbolDef::Workbench(..)
            | SymbolDef::Function(..) => Ok(()),
            _ => context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            ),
        })?;
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        "statement list"
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        &["source root", "module definition", "workbench definition", "function definition"]
    }
}

impl Grant for Statement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        match self {
            Statement::If(statement) => {
                statement.grant(parent, context)?;
            },
            Statement::Init(statement) => {
                statement.grant(parent, context)?;
            },
            Statement::Return(statement) => {
                statement.grant(parent, context)?;
            },
            _ => {
                // the error handling for the other statements are already handled
            }
        }
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        "statement"
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        unreachable!()
    }
}

impl Grant for WorkbenchDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) => Ok(()),
            _ => context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            ),
        })?;
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        self.kind.as_str()
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        &["source root", "module definition"]
    }
}

impl Grant for FunctionDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) | SymbolDef::Workbench(..) => Ok(()),
            _ => context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            ),
        })?;
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        "function definition"
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        &["source root", "module definition", "workbench definition"]
    }
}

impl Grant for InitDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::Workbench(..) => Ok(()),
            _ => context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            ),
        })?;
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        "init definition"
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        &["workbench definition"]
    }
}

impl Grant for ReturnStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::Function(..) => Ok(()),
            _ => context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            ),
        })?;
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        "return statement"
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        &["function definition"]
    }
}

impl Grant for IfStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..)
            | SymbolDef::Workbench(..)
            | SymbolDef::Function(..) => Ok(()),
            _ => context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            ),
        })?;
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        "if statement"
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        &["source root", "module definition", "workbench definition", "function definition"]
    }
}

impl Grant for AssignmentStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        let grant = parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) => {
                match self.assignment.qualifier() {
                    Qualifier::Value => matches!(self.assignment.visibility, Visibility::Private),
                    Qualifier::Const => true,
                    Qualifier::Prop => false,
                }
            }
            SymbolDef::Workbench(..) => matches!(self.assignment.visibility, Visibility::Private),
            SymbolDef::Function(..) => match self.assignment.qualifier() {
                Qualifier::Value => {
                    matches!(self.assignment.visibility, Visibility::Private)
                }
                Qualifier::Prop | Qualifier::Const => false,
            },
            _ => false,
        });

        if !grant {
            context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            )?;
        }
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        if matches!(self.assignment.visibility, Visibility::Private) {
            match self.assignment.qualifier() {
                Qualifier::Value => "assigment",
                Qualifier::Const => "constant assigment",
                Qualifier::Prop => "property assigment",
            }
        } else {
            match self.assignment.qualifier() {
                Qualifier::Value => "public assigment",
                Qualifier::Const => "public constant assigment",
                Qualifier::Prop => "public property assigment",
            }
        }
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        if matches!(self.assignment.visibility, Visibility::Private) {
            match self.assignment.qualifier() {
                Qualifier::Value => &["source root", "module definition", "workbench definition", "function definition"],
                Qualifier::Const => &["source root", "module definition", "workbench definition"],
                Qualifier::Prop => &["workbench definition"],
            }
        } else {
            match self.assignment.qualifier() {
                Qualifier::Value => &[],
                Qualifier::Const => &["source root", "module definition"],
                Qualifier::Prop => &[],
            }
        }
    }
}

impl Grant for Body {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) | SymbolDef::Function(..) => Ok(()),
            SymbolDef::Workbench(..) => {
                if let Err(err) = self.check_statements(context) {
                    context.error(self, err)?;
                }
                Ok(())
            }
            _ => context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            ),
        })?;
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        "code body"
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        &["source root", "module definition", "workbench definition", "function definition"]
    }
}

impl Body {
    fn check_statements(&self, context: &mut ResolveContext) -> ResolveResult<()> {
        if let (Some(first_init), Some(last_init)) = (
            self.iter()
                .position(|stmt| matches!(stmt, Statement::Init(_))),
            self.iter()
                .rposition(|stmt| matches!(stmt, Statement::Init(_))),
        ) {
            for (n, stmt) in self.iter().enumerate() {
                match stmt {
                    // ignore inits
                    Statement::Init(_) => (),

                    // RULE: Illegal statements in workbenches
                    Statement::Module(_) | Statement::Workbench(_) | Statement::Return(_) => {
                        context.error(stmt, ResolveError::IllegalWorkbenchStatement)?;
                    }

                    // RULE: Ony use or assignments before initializers
                    Statement::Use(_) => {
                        if n > first_init && n < last_init {
                            context.error(stmt, ResolveError::CodeBetweenInitializers)?;
                        }
                    }

                    // Some assignments are post init statements
                    Statement::Assignment(a_stmt) => match a_stmt.assignment.qualifier() {
                        Qualifier::Const => {
                            if matches!(a_stmt.assignment.visibility, Visibility::Public) {
                                context.error(a_stmt, ResolveError::IllegalWorkbenchStatement)?;
                            }
                            if n > first_init && n < last_init {
                                context.error(a_stmt, ResolveError::CodeBetweenInitializers)?;
                            }
                        }
                        Qualifier::Value => {
                            if n < last_init {
                                if n > first_init {
                                    context.error(a_stmt, ResolveError::CodeBetweenInitializers)?;
                                }
                                context.error(
                                    a_stmt,
                                    ResolveError::StatementNotAllowedPriorInitializers,
                                )?;
                            }
                        }
                        Qualifier::Prop => {
                            if n < last_init {
                                if n > first_init {
                                    context.error(a_stmt, ResolveError::CodeBetweenInitializers)?;
                                }
                                context.error(
                                    a_stmt,
                                    ResolveError::StatementNotAllowedPriorInitializers,
                                )?;
                            }
                        }
                    },

                    // Post init statements
                    Statement::If(_)
                    | Statement::InnerAttribute(_)
                    | Statement::Expression(_)
                    | Statement::Function(_) => {
                        // RULE: No code between initializers
                        if n < last_init {
                            if n > first_init {
                                context.error(stmt, ResolveError::CodeBetweenInitializers)?;
                            }
                            context
                                .error(stmt, ResolveError::StatementNotAllowedPriorInitializers)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Grant for UseStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        let grant = parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) => true,
            SymbolDef::Workbench(..) | SymbolDef::Function(..) => match self.visibility {
                Visibility::Private | Visibility::PrivateUse(_) => true,
                Visibility::Public => false,
                Visibility::Deleted => unreachable!(),
            },
            _ => false,
        });

        if !grant {
            context.error(
                self,
                StatementNotSupportedError::new(self, parent),
            )?;
        }
        Ok(self)
    }

    fn kind(&self) -> &'static str {
        "use statement"
    }

    fn allowed_parents(&self) -> &'static [&'static str] {
        match self.visibility {
            Visibility::Private | Visibility::PrivateUse(_) => &["source root", "module definition", "workbench definition", "function definition"],
            _ => &["source root", "module definition"],
        }
    }
}
