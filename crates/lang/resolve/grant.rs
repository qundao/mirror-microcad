// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Check wether a statement is legally placed.

use miette::SourceSpan;
use crate::src_ref::SrcReferrer;
use crate::{resolve::*, syntax::*};

fn capitalize_first(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub(super) trait Grant {
    /// Checks if definition is allowed to occur within the given parent symbol.
    fn grant(&self, _parent: &Symbol, _context: &mut ResolveContext) -> DiagResult<&Self> {
        Ok(self)
    }
}

impl Grant for ModuleDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported(
                    "Module definition".into(),
                    capitalize_first(def.kind_str()),
                ),
            ),
        })?;
        Ok(self)
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
                ResolveError::StatementNotSupported(
                    "Statement list".to_string(),
                    parent.kind_str(),
                ),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for Statement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match (def, &self) {
            (
                SymbolDef::SourceFile(..),
                Statement::Assignment(..)
                | Statement::Expression(..)
                | Statement::Function(..)
                | Statement::If(..)
                | Statement::Module(..)
                | Statement::Use(..)
                | Statement::Workbench(..),
            )
            | (
                SymbolDef::Module(..),
                Statement::Assignment(..)
                | Statement::Expression(..)
                | Statement::Function(..)
                | Statement::Module(..)
                | Statement::Use(..)
                | Statement::Workbench(..),
            )
            | (
                SymbolDef::Workbench(..),
                Statement::Assignment(..)
                | Statement::Expression(..)
                | Statement::Function(..)
                | Statement::If(..)
                | Statement::Init(..)
                | Statement::Use(..),
            )
            | (
                SymbolDef::Function(..),
                Statement::Assignment(..)
                | Statement::If(..)
                | Statement::Return(..)
                | Statement::Use(..)
                | Statement::Expression(..),
            ) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported("Statement".to_string(), parent.kind_str()),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for WorkbenchDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported(self.kind.to_string(), parent.kind_str()),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for FunctionDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) | SymbolDef::Workbench(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported(
                    "Function definition".to_string(),
                    parent.kind_str(),
                ),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for InitDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::Workbench(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported(
                    "Init definition".to_string(),
                    parent.kind_str(),
                ),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for ReturnStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::Function(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported(
                    "Return statement".to_string(),
                    parent.kind_str(),
                ),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for IfStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..)
            | SymbolDef::Module(..)
            | SymbolDef::Workbench(..)
            | SymbolDef::Function(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported("If statement".to_string(), parent.kind_str()),
            ),
        })?;
        Ok(self)
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
                ResolveError::StatementNotSupported(
                    "Assignment statement".to_string(),
                    parent.kind_str(),
                ),
            )?;
        }
        Ok(self)
    }
}

impl Grant for Body {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDef::SourceFile(..) | SymbolDef::Module(..) | SymbolDef::Function(..) => Ok(()),
            SymbolDef::Workbench(..) => {
                if let Err(err) = self.check_statements(parent, context) {
                    context.error(self, err)?;
                }
                Ok(())
            }
            _ => context.error(
                self,
                ResolveError::StatementNotSupported("Code body".to_string(), parent.kind_str()),
            ),
        })?;
        Ok(self)
    }
}

impl Body {
    fn check_statements(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<()> {
        if let (Some(first_init_pos), Some(last_init_pos)) = (
            self.iter()
                .position(|stmt| matches!(stmt, Statement::Init(_))),
            self.iter()
                .rposition(|stmt| matches!(stmt, Statement::Init(_))),
        ) {
            let first_init = &self.statements[first_init_pos];
            let last_init = &self.statements[last_init_pos];

            let code_before_err =
                |stmt: &Statement| ResolveError::StatementNotAllowedPriorInitializers {
                    initializer: first_init.src_ref().into(),
                    statement: stmt.src_ref().into(),
                    workbench: parent.src_ref().into(),
                    kind: parent.kind_str(),
                };
            let code_between_err = |stmt: &Statement| {
                let start_span = SourceSpan::from(first_init.src_ref());
                let end_span = SourceSpan::from(last_init.src_ref());
                let initializers_start = start_span.offset();
                let initializers_end = end_span.offset() + end_span.len();
                ResolveError::CodeBetweenInitializers {
                    initializers: SourceSpan::new(initializers_start.into(), initializers_end - initializers_start),
                    statement: stmt.src_ref().into(),
                    workbench: parent.src_ref().into(),
                    kind: parent.kind_str(),
                }
            };

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
                        if n > first_init_pos && n < last_init_pos {
                            context.error(stmt, code_between_err(stmt))?;
                        }
                    }

                    // Some assignments are post init statements
                    Statement::Assignment(a_stmt) => match a_stmt.assignment.qualifier() {
                        Qualifier::Const => {
                            if matches!(a_stmt.assignment.visibility, Visibility::Public) {
                                context.error(a_stmt, ResolveError::IllegalWorkbenchStatement)?;
                            }
                            if n > first_init_pos && n < last_init_pos {
                                context.error(a_stmt, code_between_err(stmt))?;
                            }
                        }
                        Qualifier::Value => {
                            if n < last_init_pos {
                                if n > first_init_pos {
                                    context.error(a_stmt, code_between_err(stmt))?;
                                } else {
                                    context.error(a_stmt, code_before_err(stmt))?;
                                }
                            }
                        }
                        Qualifier::Prop => {
                            if n < last_init_pos {
                                if n > first_init_pos {
                                    context.error(a_stmt, code_between_err(stmt))?;
                                } else {
                                    context.error(a_stmt, code_before_err(stmt))?;
                                }
                            }
                        }
                    },

                    // Post init statements
                    Statement::If(_)
                    | Statement::InnerAttribute(_)
                    | Statement::Expression(_)
                    | Statement::Function(_) => {
                        // RULE: No code between initializers
                        if n < last_init_pos {
                            if n > first_init_pos {
                                context.error(stmt, code_between_err(stmt))?;
                            } else {
                                context.error(stmt, code_before_err(stmt))?;
                            }
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
                ResolveError::StatementNotSupported("Use statement".to_string(), parent.kind_str()),
            )?;
        }
        Ok(self)
    }
}
