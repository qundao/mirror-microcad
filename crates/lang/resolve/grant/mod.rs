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
}

impl Grant for SourceFile {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        context.scope(Scope(Source, SrcRef(None)), |context| {
            self.statements.grant(context)
        })
    }
}

impl Grant for ModuleDefinition {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(Module, self.keyword_ref.clone());
        let parent = &context.parent();
        match parent.ty() {
            Source | Module => Ok(()),
            _ => context.error(self, StatementNotSupportedError::new(&scope, parent)),
        }?;
        if let Some(body) = &self.body {
            context.scope(scope, |context| body.grant(context))?;
        }
        Ok(())
    }
}

impl Grant for StatementList {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(StatementList, self.src_ref());
        let parent = &context.parent();
        match parent.ty() {
            Source | Module | Workbench | Function | Init | If | ExpressionStatement => Ok(()),
            _ => context.error(self, StatementNotSupportedError::new(&scope, parent)),
        }?;
        self.iter()
            .try_for_each(|statement| statement.grant(context))
    }
}

impl Grant for Statement {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use Statement::*;
        match self {
            If(statement) => statement.grant(context),
            Init(statement) => statement.grant(context),
            Return(statement) => statement.grant(context),
            Value(statement) => statement.grant(context),
            Const(statement) => statement.grant(context),
            Prop(statement) => statement.grant(context),
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
}

impl Grant for WorkbenchDefinition {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(Workbench, self.keyword_ref.clone());
        let parent = &context.parent();
        match parent.ty() {
            Source | Module => Ok(()),
            _ => context.error(self, StatementNotSupportedError::new(&scope, parent)),
        }?;
        context.scope(scope, |context| self.body.grant(context))
    }
}

impl Grant for FunctionDefinition {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(Function, self.keyword_ref.clone());
        let parent = &context.parent();
        match parent.ty() {
            Source | Module => Ok(()),
            Workbench => {
                use Visibility::*;
                match self.visibility {
                    Private | PrivateUse(..) => Ok(()),
                    Public | Deleted => {
                        context.error(self, StatementNotSupportedError::new(&scope, parent))
                    }
                }
            }
            _ => context.error(self, StatementNotSupportedError::new(&scope, parent)),
        }?;
        context.scope(scope, |context| self.body.grant(context))
    }
}

impl Grant for InitDefinition {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(Init, self.keyword_ref.clone());
        let parent = &context.parent();
        match parent.ty() {
            Workbench => Ok(()),
            _ => context.error(self, StatementNotSupportedError::new(&scope, parent)),
        }?;
        context.scope(scope, |context| self.body.grant(context))
    }
}

impl Grant for ReturnStatement {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(Return, self.src_ref());
        let parent = &context.parent();
        match parent.ty() {
            Function => Ok(()),
            If => {
                if context.find(|scope| matches!(scope.ty(), Function)) {
                    Ok(())
                } else {
                    context.error(self, StatementNotSupportedError::new(&scope, parent))
                }
            }
            _ => context.error(self, StatementNotSupportedError::new(&scope, parent)),
        }
    }
}

impl Grant for IfStatement {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(If, self.src_ref());
        let parent = &context.parent();
        match &context.parent().ty() {
            Source | Workbench | Function | If | ExpressionStatement => Ok(()),
            _ => context.error(self, StatementNotSupportedError::new(&scope, parent)),
        }?;
        context.scope(Scope(If, self.body.src_ref()), |context| {
            self.body.grant(context)
        })?;
        if let Some(body_else) = &self.body_else {
            context.scope(Scope(If, body_else.src_ref()), |context| {
                body_else.grant(context)
            })?;
        }
        if let Some(next_if) = &self.next_if {
            context.scope(Scope(If, next_if.src_ref()), |context| {
                next_if.grant(context)
            })?;
        }
        Ok(())
    }
}

impl Grant for ValueAssignment {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(ValueAssignment, self.src_ref());
        let parent = context.parent();
        let grant = matches!(
            parent.ty(),
            Source | Function | Workbench | Init | ExpressionStatement
        );
        if !grant {
            context.error(self, StatementNotSupportedError::new(&scope, &parent))?;
        }
        Ok(())
    }
}

impl Grant for ConstAssignment {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use {ScopeType::*, Visibility::*};
        let scope = Scope(ValueAssignment, self.src_ref());
        let parent = context.parent();
        let grant = match &self.visibility {
            Private | PrivateUse(..) => matches!(parent.ty(), Source | Module | Workbench),
            Public => matches!(parent.ty(), Source | Module),
            Deleted => unreachable!(),
        };
        if !grant {
            context.error(self, StatementNotSupportedError::new(&scope, &parent))?;
        }
        Ok(())
    }
}

impl Grant for PropAssignment {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(PropAssignment, self.src_ref());
        let parent = context.parent();
        let grant = matches!(parent.ty(), Workbench);
        if !grant {
            context.error(self, StatementNotSupportedError::new(&scope, &parent))?;
        }
        Ok(())
    }
}

impl<T: Grant> Grant for AssignmentStatement<T> {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        self.assignment.grant(context)
    }
}

impl Grant for Body {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(Body, self.src_ref());
        let parent = &context.parent();
        match parent.ty() {
            Source | Module | Function | Init | If | ExpressionStatement => Ok(()),
            Workbench => {
                if let Err(err) = self.check_workbench_body(parent, context) {
                    context.error(self, err)?;
                }
                Ok(())
            }
            _ => context.error(self, StatementNotSupportedError::new(&scope, parent)),
        }?;

        self.statements.grant(context)
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
                use {Statement::*, Visibility::*};
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
                    Statement::Const(a_stmt) => {
                        if matches!(a_stmt.visibility, Public) {
                            context.error(a_stmt, IllegalWorkbenchStatement)?;
                        }
                        if n > first_init_pos && n < last_init_pos {
                            context.error(a_stmt, code_between_err(stmt))?;
                        }
                    }
                    Statement::Value(a_stmt) => {
                        if n < last_init_pos {
                            if n > first_init_pos {
                                context.error(a_stmt, code_between_err(stmt))?;
                            } else {
                                context.error(a_stmt, code_before_err(stmt))?;
                            }
                        }
                    }
                    Statement::Prop(a_stmt) => {
                        if n < last_init_pos {
                            if n > first_init_pos {
                                context.error(a_stmt, code_between_err(stmt))?;
                            } else {
                                context.error(a_stmt, code_before_err(stmt))?;
                            }
                        }
                    }

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
        use {ScopeType::*, Visibility::*};
        let scope = Scope(
            match self.visibility {
                Private | PrivateUse(..) => Use,
                Public => PubUse,
                _ => unreachable!(),
            },
            self.src_ref(),
        );
        let parent = &context.parent();
        let grant = match parent.ty() {
            Source | Module => true,
            Workbench | Function | ExpressionStatement | Body | Init => match scope.0 {
                Use => true,
                PubUse => false,
                _ => unreachable!(),
            },
            _ => false,
        };

        if !grant {
            context.error(self, StatementNotSupportedError::new(&scope, parent))?;
        }
        Ok(())
    }
}

impl Grant for ExpressionStatement {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use ScopeType::*;
        let scope = Scope(ExpressionStatement, self.src_ref());
        let parent = &context.parent();
        if !matches!(
            parent.ty(),
            Source | Workbench | Function | If | StatementList | Init | ExpressionStatement
        ) {
            context.error(self, StatementNotSupportedError::new(&scope, parent))?;
        }
        context.scope(scope, |context| self.expression.grant(context))
    }
}

impl Grant for Expression {
    fn grant(&self, context: &mut GrantContext) -> DiagResult<()> {
        use FormatStringInner::*;

        {
            use ScopeType::*;
            let parent = &context.parent();
            let scope = Scope(Expression, self.src_ref());
            match parent.ty() {
                Workbench | Function | If | StatementList | ExpressionStatement | Expression => (),
                _ => context.error(self, StatementNotSupportedError::new(&scope, parent))?,
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
}
