// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Check wether a statement is legally placed.

mod context;
mod scope;

use crate::src_ref::{SrcRef, SrcReferrer};
use crate::{resolve::*, syntax::*};
pub(crate) use context::*;
pub(crate) use scope::*;

pub(super) trait Grant {
    /// Checks if definition is allowed to occur within the given parent symbol.
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()>;

    /// Return scope information.
    fn scope(&self) -> Scope;
}

impl Grant for SourceFile {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        context.scope(Source, |context| {
            self.statements.iter().try_for_each(|statement| {
                statement.grant(context)?;
                Ok(())
            })
        })
    }

    fn scope(&self) -> Scope {
        Scope::Source
    }
}

impl Grant for ModuleDefinition {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        match &context.parent() {
            Source | Module(..) => Ok(()),
            parent => context.error(self, StatementNotSupportedError::new(&self.scope(), parent)),
        }?;
        if let Some(body) = &self.body {
            context.scope(Module(self.keyword_ref.clone()), |context| {
                body.grant(context)
            })?;
        }
        Ok(())
    }

    fn scope(&self) -> Scope {
        Scope::Module(self.keyword_ref.clone())
    }
}

impl Grant for StatementList {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        match &context.parent() {
            Source | Module(..) | Workbench(..) | Function(..) => Ok(()),
            parent => context.error(self, StatementNotSupportedError::new(&self.scope(), parent)),
        }?;
        self.iter().try_for_each(|statement| {
            statement.grant(context)?;
            Ok(())
        })
    }

    fn scope(&self) -> Scope {
        Scope::StatementList(self.src_ref())
    }
}

impl Grant for Statement {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Statement::*;
        match self {
            If(statement) => statement.grant(context),
            Init(statement) => statement.grant(context),
            Return(statement) => statement.grant(context),
            Assignment(statement) => statement.grant(context),
            Module(statement) => statement.grant(context),
            Workbench(statement) => statement.grant(context),
            Function(statement) => statement.grant(context),
            Use(statement) => statement.grant(context),
            Expression(statement) => statement.grant(context),
            _ => {
                // the error handling for the other statements are already handled
                Ok(())
            }
        }
    }

    fn scope(&self) -> Scope {
        use Statement::*;
        match self {
            If(statement) => statement.scope(),
            Init(statement) => statement.scope(),
            Return(statement) => statement.scope(),
            Workbench(statement) => statement.scope(),
            Module(statement) => statement.scope(),
            Function(statement) => statement.scope(),
            Use(statement) => statement.scope(),
            Assignment(statement) => statement.scope(),
            _ => unreachable!(),
        }
    }
}

impl Grant for WorkbenchDefinition {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        match &context.parent() {
            Source | Module(..) => Ok(()),
            parent => context.error(self, StatementNotSupportedError::new(&self.scope(), parent)),
        }?;
        context.scope(Workbench(self.keyword_ref.clone()), |context| {
            self.body.grant(context)
        })
    }

    fn scope(&self) -> Scope {
        Scope::Workbench(self.keyword_ref.clone())
    }
}

impl Grant for FunctionDefinition {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        let parent = &context.parent();
        match parent {
            Source | Module(..) => Ok(()),
            Workbench(..) => {
                use Visibility::*;
                match self.visibility {
                    Private | PrivateUse(..) => Ok(()),
                    Public | Deleted => {
                        context.error(self, StatementNotSupportedError::new(&self.scope(), parent))
                    }
                }
            }
            parent => context.error(self, StatementNotSupportedError::new(&self.scope(), parent)),
        }?;
        context.scope(Function(self.keyword_ref.clone()), |context| {
            self.body.grant(context)
        })
    }

    fn scope(&self) -> Scope {
        Scope::Function(self.keyword_ref.clone())
    }
}

impl Grant for InitDefinition {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        match &context.parent() {
            Workbench(..) => Ok(()),
            parent => context.error(self, StatementNotSupportedError::new(&self.scope(), parent)),
        }?;
        context.scope(Init(self.keyword_ref.clone()), |context| {
            self.body.grant(context)
        })
    }

    fn scope(&self) -> Scope {
        Scope::Init(self.keyword_ref.clone())
    }
}

impl Grant for ReturnStatement {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        let parent = context.parent();
        match &parent {
            Function(..) => Ok(()),
            If(..) => {
                if context.find(|k| matches!(k, Scope::Function(..))) {
                    Ok(())
                } else {
                    context.error(
                        self,
                        StatementNotSupportedError::new(&self.scope(), &parent),
                    )
                }
            }
            parent => context.error(self, StatementNotSupportedError::new(&self.scope(), parent)),
        }
    }

    fn scope(&self) -> Scope {
        Scope::Return(self.src_ref())
    }
}

impl Grant for IfStatement {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        match &context.parent() {
            Source | Workbench(..) | Function(..) | If(..) | ExpressionStatement(..) => Ok(()),
            parent => context.error(self, StatementNotSupportedError::new(&self.scope(), parent)),
        }?;
        context.scope(If(self.src_ref()), |context| self.body.grant(context))?;
        if let Some(body_else) = &self.body_else {
            context.scope(If(self.src_ref()), |context| body_else.grant(context))?;
        }
        if let Some(next_if) = &self.next_if {
            context.scope(If(self.src_ref()), |context| next_if.grant(context))?;
        }
        Ok(())
    }

    fn scope(&self) -> Scope {
        Scope::If(self.src_ref())
    }
}

impl Grant for AssignmentStatement {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use {Qualifier::*, Scope::*, Visibility::*};
        let parent = context.parent();
        let grant = match (&self.assignment.visibility, self.assignment.qualifier()) {
            (Private | PrivateUse(_), Value) => {
                matches!(
                    parent,
                    Source | Function(..) | Workbench(..) | Init(..) | ExpressionStatement(..)
                )
            }
            (Public, _) => matches!(parent, Source | Module(..)),
            (_, Const) => matches!(parent, Source | Module(..) | Workbench(..)),
            (_, Prop) => matches!(parent, Workbench(..)),
            (Deleted, _) => false,
        };

        if !grant {
            context.error(
                self,
                StatementNotSupportedError::new(&self.scope(), &parent),
            )?;
        }
        Ok(())
    }

    fn scope(&self) -> Scope {
        Scope::Assignment(
            self.src_ref(),
            self.assignment.visibility.clone(),
            self.assignment.qualifier(),
        )
    }
}

impl Grant for Body {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        let parent = &context.parent();
        match parent {
            Source | Module(..) | Function(..) | Init(..) | If(..) | ExpressionStatement(..) => {
                Ok(())
            }
            Workbench(..) => {
                if let Err(err) = self.check_workbench_body(parent, context) {
                    context.error(self, err)?;
                }
                Ok(())
            }
            parent => context.error(self, StatementNotSupportedError::new(&self.scope(), parent)),
        }?;

        self.iter()
            .try_for_each(|statement| statement.grant(context))
    }

    fn scope(&self) -> Scope {
        Scope::Body(self.src_ref(), None)
    }
}

impl Body {
    fn check_workbench_body(
        &self,
        parent: &Scope,
        context: &mut GrantContext,
    ) -> ResolveResult<()> {
        fn find_init((pos, stmt): (usize, &Statement)) -> Option<(usize, &InitDefinition)> {
            match stmt {
                Statement::Init(init) => Some((pos, init.as_ref())),
                _ => None,
            }
        }
        if let (Some((first_init_pos, first_init)), Some((last_init_pos, last_init))) = (
            self.iter().enumerate().find_map(find_init),
            self.iter().enumerate().rev().find_map(find_init),
        ) {
            use ResolveError::*;
            let code_before_err = |stmt: &Statement| StatementNotAllowedPriorInitializers {
                initializer: first_init.keyword_ref.clone(),
                statement: stmt.src_ref(),
                workbench: parent.src_ref(),
                scope: parent.to_str(),
            };
            let code_between_err = |stmt: &Statement| CodeBetweenInitializers {
                initializers: SrcRef::merge(&first_init.keyword_ref, &last_init.keyword_ref),
                statement: stmt.src_ref(),
                workbench: parent.src_ref(),
                scope: parent.to_str(),
            };

            for (n, stmt) in self.iter().enumerate() {
                use {Qualifier::*, Statement::*, Visibility::*};
                match stmt {
                    // ignore inits
                    Init(_) => (),

                    // RULE: Illegal statements in workbenches
                    Module(_) | Workbench(_) | Return(_) => {
                        context.error(stmt, IllegalWorkbenchStatement)?;
                    }

                    // RULE: Ony use or assignments before initializers
                    Use(_) => {
                        if n > first_init_pos && n < last_init_pos {
                            context.error(stmt, code_between_err(stmt))?;
                        }
                    }

                    // Some assignments are post init statements
                    Assignment(a_stmt) => match a_stmt.assignment.qualifier() {
                        Const => {
                            if matches!(a_stmt.assignment.visibility, Public) {
                                context.error(a_stmt, IllegalWorkbenchStatement)?;
                            }
                            if n > first_init_pos && n < last_init_pos {
                                context.error(a_stmt, code_between_err(stmt))?;
                            }
                        }
                        Value => {
                            if n < last_init_pos {
                                if n > first_init_pos {
                                    context.error(a_stmt, code_between_err(stmt))?;
                                } else {
                                    context.error(a_stmt, code_before_err(stmt))?;
                                }
                            }
                        }
                        Prop => {
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
                    If(_) | InnerAttribute(_) | InnerDocComment(_) | Expression(_)
                    | Function(_) => {
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
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        let parent = &context.parent();
        let grant = match parent {
            Source | Module(..) => true,
            Workbench(..) | Function(..) | ExpressionStatement(..) | Body(..) | Init(..) => {
                match self.visibility {
                    Visibility::Private | Visibility::PrivateUse(_) => true,
                    Visibility::Public => false,
                    Visibility::Deleted => unreachable!(),
                }
            }
            _ => false,
        };

        if !grant {
            context.error(self, StatementNotSupportedError::new(&self.scope(), parent))?;
        }
        Ok(())
    }

    fn scope(&self) -> Scope {
        Scope::Use(self.src_ref(), self.visibility.clone())
    }
}

impl Grant for ExpressionStatement {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Scope::*;
        let parent = &context.parent();
        if !matches!(
            parent,
            Source
                | Workbench(..)
                | Function(..)
                | If(..)
                | StatementList(..)
                | Init(..)
                | ExpressionStatement(..)
        ) {
            context.error(self, StatementNotSupportedError::new(&self.scope(), parent))?;
        }
        context.scope(Scope::ExpressionStatement(self.src_ref()), |context| {
            self.expression.grant(context)
        })
    }

    fn scope(&self) -> Scope {
        Scope::ExpressionStatement(self.src_ref())
    }
}

impl Grant for Expression {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use FormatStringInner::*;

        {
            let parent = &context.parent();
            use Scope::*;
            match parent {
                Workbench(..)
                | Function(..)
                | If(..)
                | StatementList(..)
                | ExpressionStatement(..)
                | Expression(..) => (),
                _ => context.error(self, StatementNotSupportedError::new(&self.scope(), parent))?,
            }
        }

        use Expression::*;
        match self {
            Invalid | Expression::Literal(..) | QualifiedName(..) | Marker(..) => Ok(()),
            FormatString(fs) => {
                fs.0.iter()
                    .filter_map(|fs| {
                        if let FormatExpression(fs) = fs {
                            Some(&fs.expression)
                        } else {
                            None
                        }
                    })
                    .try_for_each(|exp| exp.grant(context))
            }
            ArrayExpression(ae) => {
                if let ArrayExpressionInner::List(expressions) = &ae.inner {
                    expressions.iter().try_for_each(|exp| exp.grant(context))
                } else {
                    Ok(())
                }
            }
            TupleExpression(te) => te
                .args
                .iter()
                .map(|arg| &arg.expression)
                .try_for_each(|exp| exp.grant(context)),
            Body(body) => body.grant(context),
            If(is) => is.grant(context),
            Call(call) => call
                .argument_list
                .iter()
                .map(|arg| &arg.expression)
                .try_for_each(|exp| exp.grant(context)),

            BinaryOp {
                lhs,
                op: _,
                rhs,
                src_ref: _,
            } => {
                lhs.grant(context)?;
                rhs.grant(context)
            }
            UnaryOp {
                op: _,
                rhs,
                src_ref: _,
            } => rhs.grant(context),
            ArrayElementAccess(exp, exp1, _) => {
                exp.grant(context)?;
                exp1.grant(context)
            }
            PropertyAccess(exp, ..) | AttributeAccess(exp, ..) => exp.grant(context),
            MethodCall(exp, mc, ..) => {
                exp.grant(context)?;
                mc.argument_list
                    .iter()
                    .map(|arg| &arg.expression)
                    .try_for_each(|exp| exp.grant(context))
            }
        }
    }

    fn scope(&self) -> Scope {
        Scope::Expression(self.src_ref())
    }
}
