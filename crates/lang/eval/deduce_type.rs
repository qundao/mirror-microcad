use std::ops::{Div, Mul};

use crate::{builtin::Builtin, eval::*, model::*, src_ref::*, syntax::*, ty::*, value::*};

/// Interface to deduce the result type of a statement
pub trait DeduceResult {
    /// Deduce type of self.
    /// - `params`: Parameters and locals from outside.
    /// - `context`: Resolve context to fetch symbols
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type>;
}

impl DeduceResult for Symbol {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        log::trace!("deducing Symbol {}", self.id());
        self.with_def(|def| match def {
            SymbolDef::Workbench(wd) => wd.deduce_result(context),
            SymbolDef::Function(fd) => fd.deduce_result(context),
            SymbolDef::Builtin(bi) => bi.deduce_result(context),
            SymbolDef::SourceFile(sf) => sf.deduce_result(context),
            SymbolDef::Constant(.., v) | SymbolDef::Argument(.., v) => Ok(v.ty()),
            SymbolDef::Root => {
                for (id, symbol) in self.children() {
                    if Type::Invalid != symbol.deduce_result(context)? {
                        /*                        context.error(
                            &id,
                            EvalError::UnexpectedRootResult {
                                statement: symbol.src_ref(),
                            },
                        )?
                        */
                    }
                }
                Ok(Type::Invalid)
            }
            SymbolDef::Module(..)
            | SymbolDef::Alias(..)
            | SymbolDef::UseAll(..)
            | SymbolDef::Assignment(..) => Ok(Type::Invalid),

            #[cfg(test)]
            SymbolDef::Tester(..) => todo!(),
        })
        .inspect(|ty| log::trace!("Type = {ty}"))
    }
}

impl DeduceResult for SourceFile {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        context.open(StackFrame::Source(self.id(), Default::default()));
        let result = self.statements.deduce_result(context);
        context.close();
        result
    }
}

impl DeduceResult for Body {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        self.statements
            .deduce_result(context)
            .inspect(|ty| log::trace!("deduced Body: {ty}"))
    }
}

impl DeduceResult for StatementList {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        let mut result = Type::Invalid;
        let mut result_src_ref = None;
        for statement in &self.statements {
            match statement.deduce_result(context)? {
                r @ Type::Return(..) => {
                    if let Some(result_src_ref) = result_src_ref.clone() {
                        context.error(
                            statement,
                            EvalError::UnexpectedResult {
                                result: result_src_ref,
                                statement: statement.src_ref(),
                            },
                        )?;
                        break;
                    }

                    result = r;
                    result_src_ref = Some(statement.src_ref())
                },
                _ => (),
            }
        }

        if let Some(tail) = self.tail.as_deref() {
            if let Some(result_src_ref) = result_src_ref {
                context.error(
                    tail,
                    EvalError::UnexpectedResult {
                        result: result_src_ref,
                        statement: tail.src_ref(),
                    },
                )?;
            }
            result = tail.deduce_result(context)?;
            result_src_ref = Some(tail.src_ref())
        }

        Ok(result).inspect(|ty| log::trace!("deduced StatementList: {ty}"))
    }
}

impl DeduceResult for Statement {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        match self {
            Statement::Module(m) => m.deduce_result(context),
            Statement::Expression(expr) => expr.deduce_result(context),
            Statement::Workbench(wd) => wd.deduce_result(context),
            Statement::Function(f) => f.deduce_result(context),
            Statement::Init(id) => id.deduce_result(context),
            Statement::Return(rs) => rs.deduce_result(context),
            Statement::If(is) => is.deduce_result(context),
            Statement::Use(u) => u.deduce_result(context),
            Statement::Assignment(a) => a.deduce_result(context),
            Statement::InnerAttribute(..) | Statement::InnerDocComment(..) => Ok(Type::Invalid),
        }
        .inspect(|ty| log::trace!("deduced Statement: {ty}"))
    }
}

impl DeduceResult for ModuleDefinition {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        context.open(StackFrame::Module(self.id(), Default::default()));
        if let Some(body) = &self.body {
            match body.deduce_result(context)? {
                Type::Invalid => (),
                _ => todo!("module with result"),
            }
        }
        context.close();
        Ok(Type::Invalid)
    }
}

impl DeduceResult for ExpressionStatement {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        self.expression.deduce_result(context)
    }
}

impl DeduceResult for Expression {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        match &self {
            Expression::Invalid => Ok(Type::Invalid),
            Expression::Literal(l) => Ok(l.value().ty()),
            Expression::FormatString(..) => Ok(Type::String),
            Expression::ArrayExpression(ae) => ae.deduce_result(context),
            Expression::TupleExpression(te) => te.deduce_result(context),
            Expression::Body(b) => b.deduce_result(context),
            Expression::If(is) => is.deduce_result(context),
            Expression::Call(c) => c.deduce_result(context),
            Expression::QualifiedName(qn) => qn.deduce_result(context),
            Expression::Marker(m) => m.deduce_result(context),
            Expression::BinaryOp { lhs, op, rhs, .. } => match op.as_str() {
                "+" | "-" => lhs.deduce_result(context),
                "/" => Ok(lhs.deduce_result(context)?.div(rhs.deduce_result(context)?)),
                "*" => Ok(lhs.deduce_result(context)?.mul(rhs.deduce_result(context)?)),
                "<" | ">" | "≤" | "≥" | "&" | "|" => Ok(Type::Bool),
                _ => unreachable!(),
            },
            Expression::UnaryOp { rhs, .. } => rhs.deduce_result(context),
            Expression::ArrayElementAccess(expression, ..) => {
                if let Type::Array(ty) = expression.deduce_result(context)? {
                    Ok(*ty)
                } else {
                    unreachable!("no array type")
                }
            }
            Expression::PropertyAccess(expression, identifier, src_ref) => todo!(),
            Expression::AttributeAccess(expression, identifier, src_ref) => todo!(),
            Expression::MethodCall(_, mc, _) => mc.deduce_result(context),
        }
        .inspect(|ty| log::trace!("deduced Expression: {ty}"))
    }
}

impl DeduceResult for ArrayExpression {
    fn deduce_result(&self, _: &mut EvalContext) -> EvalResult<Type> {
        Ok(Type::Array(self.unit.ty().into()))
            .inspect(|ty| log::trace!("deduced ArrayExpression: {ty}"))
    }
}

impl DeduceResult for TupleExpression {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        Ok(Type::Tuple(Box::new(
            self.args
                .iter()
                .map(|arg| match arg.expression.deduce_result(context) {
                    Ok(ty) => Ok((arg.id.clone().unwrap_or(Identifier::none()), ty)),
                    Err(err) => Err(err),
                })
                .collect::<EvalResult<_>>()?,
        )))
        .inspect(|ty| log::trace!("deduced TupleExpression: {ty}"))
    }
}

fn parameter_list_to_symbol_map(
    params: &ParameterList,
    context: &mut EvalContext,
) -> EvalResult<SymbolMap> {
    Ok(SymbolMap(
        params
            .iter()
            .map(|param| {
                Ok((
                    param.id.clone(),
                    Symbol::new(
                        SymbolDef::Argument(
                            param.id.clone(),
                            Value::default_from_type(&param.deduce_result(context)?),
                        ),
                        None,
                    ),
                ))
            })
            .collect::<EvalResult<_>>()?,
    ))
}
fn parameter_list_to_properties(
    params: &ParameterList,
    context: &mut EvalContext,
) -> EvalResult<Properties> {
    params
        .iter()
        .map(|param| {
            Ok((
                param.id.clone(),
                Value::default_from_type(&param.deduce_result(context)?),
            ))
        })
        .collect::<EvalResult<_>>()
}

impl DeduceResult for WorkbenchDefinition {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        let properties = parameter_list_to_properties(&self.plan, context)?;
        let model = ModelBuilder::new(
            Element::Properties(self.kind.value, properties),
            self.src_ref(),
        )
        .attributes(self.attribute_list.eval(context)?)
        .build();

        context.open(StackFrame::Workbench(
            model,
            self.id.clone(),
            Default::default(),
        ));
        let result = self
            .body
            .deduce_result(context)
            .inspect(|ty| log::trace!("deduced WorkbenchDefinition '{}': {ty}", self.id));
        context.close();
        result
    }
}

impl DeduceResult for FunctionDefinition {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        let args = parameter_list_to_symbol_map(&self.signature.parameters, context)?;
        context.open(StackFrame::Function(self.id.clone(), args));
        let result = match self.body.deduce_result(context) {
            Ok(ty_body) => {
                let ty_sig = self.return_type();
                if ty_body != ty_sig {
                    context.error(
                        self,
                        EvalError::UnexpectedResultType {
                            ty_sig,
                            src_sig: self.signature.src_ref(),
                            ty_body,
                            src_body: self.body.src_ref(),
                        },
                    )?
                }
                Ok(Type::Invalid)
            }
            Err(err) => todo!("{err}"),
        };
        context.close();
        result.inspect(|ty| log::trace!("deduced FunctionDefinition '{}': {ty}", self.id))
    }
}

impl DeduceResult for InitDefinition {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        let map = parameter_list_to_symbol_map(&self.parameters, context)?;
        context.open(StackFrame::Init(map));
        let result = self.body.deduce_result(context);
        context.close();
        result.inspect(|ty| log::trace!("deduced InitDefinition: {ty}"))
    }
}

impl DeduceResult for ReturnStatement {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        if let Some(result) = &self.result {
            result
                .deduce_result(context)
                .map(|ty| Type::Return(ty.into()))
        } else {
            Ok(Type::Invalid).inspect(|ty| log::trace!("deduced ReturnStatement: {ty}"))
        }
    }
}

impl DeduceResult for IfStatement {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        let ty_body = self.body.deduce_result(context)?;
        if let Some(body_else) = &self.body_else {
            let ty_else = body_else.deduce_result(context)?;
            log::trace!("deduced IfStatement(else): {ty_else}");
            if ty_body != ty_else {
                context.error(
                    body_else,
                    EvalError::DifferentResults {
                        ty_body: ty_body.clone(),
                        src_body: self.body.src_ref(),
                        ty_else,
                        src_else: body_else.src_ref(),
                    },
                )?;
            }
        }
        if let Some(next_if) = &self.next_if {
            let ty_next_if = next_if.deduce_result(context)?;
            log::trace!("deduced IfStatement(next if): {ty_next_if}");
            if ty_body != ty_next_if {
                context.error(
                    next_if.as_ref(),
                    EvalError::DifferentResults {
                        ty_body: ty_body.clone(),
                        src_body: self.body.src_ref(),
                        ty_else: ty_next_if,
                        src_else: next_if.src_ref(),
                    },
                )?;
            }
        }
        Ok(ty_body).inspect(|ty| log::trace!("deduced IfStatement: {ty}"))
    }
}

impl Symbol {
    pub fn ty(&self, context: &mut EvalContext) -> EvalResult<Type> {
        self.with_def(|def| match def {
            SymbolDef::Root
            | SymbolDef::SourceFile(..)
            | SymbolDef::Module(..)
            | SymbolDef::UseAll(..) => Ok(Type::Invalid),
            SymbolDef::Workbench(..) => Ok(Type::Model),
            SymbolDef::Function(fd) => Ok(fd.return_type()),
            SymbolDef::Assignment(a) => a.expression.deduce_result(context),
            SymbolDef::Builtin(b) => Ok((*b.r)(&b.parameters)?),
            SymbolDef::Constant(.., value) => Ok(value.ty()),
            SymbolDef::Argument(.., value) => Ok(value.ty()),
            SymbolDef::Alias(.., qn) => qn.deduce_result(context),
            #[cfg(test)]
            SymbolDef::Tester(_) => unreachable!(),
        })
    }
}

impl DeduceResult for Call {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        let symbol = match context.lookup(&self.name, LookupTarget::Function) {
            Ok(symbol) => symbol,
            _ => {
                return Ok(Type::Invalid);
            }
        };

        // evaluate arguments
        let args: ArgumentValueList = if symbol.is_target_mode() {
            // for assert_valid() and assert_invalid()
            Eval::<ArgumentValueListRaw>::eval(&self.argument_list, context)
                .unwrap_or_default()
                .into()
        } else {
            self.argument_list.eval(context).unwrap_or_default()
        };

        symbol
            .with_def(|def| match def {
                SymbolDef::Function(fd) => {
                    match ArgumentMatch::find_multi_match(
                        &args,
                        &fd.signature.parameters.eval(context).unwrap_or_default(),
                    ) {
                        Ok(m) => Ok(m.multi_type(fd.return_type())),
                        _ => Ok(Type::Invalid),
                    }
                }
                SymbolDef::Workbench(_) => Ok(Type::Model),
                SymbolDef::Builtin(bi) => {
                    match ArgumentMatch::find_multi_match(&args, &bi.parameters) {
                        Ok(m) => Ok(m.multi_type((*bi.r)(&bi.parameters)?)),
                        _ => Ok(Type::Invalid),
                    }
                }
                _ => unreachable!("{self:?}"),
            })
            .inspect(|ty| log::trace!("deduced Call: {ty}"))
    }
}

impl DeduceResult for QualifiedName {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        match context.lookup(self, LookupTarget::Any) {
            Ok(symbol) => symbol.ty(context).inspect(|ty| log::trace!("Type = {ty}")),
            Err(_) => Ok(Type::Invalid),
        }
        .inspect(|ty| log::trace!("deduced QualifiedName: {ty}"))
    }
}

impl DeduceResult for Marker {
    fn deduce_result(&self, _: &mut EvalContext) -> EvalResult<Type> {
        Ok(Type::Model).inspect(|ty| log::trace!("deduced Marker: {ty}"))
    }
}

impl DeduceResult for Builtin {
    fn deduce_result(&self, _: &mut EvalContext) -> EvalResult<Type> {
        match &self.kind {
            crate::builtin::BuiltinKind::Function => Ok((*self.r)(&self.parameters)?),
            crate::builtin::BuiltinKind::Workbench(..) => Ok(Type::Model),
        }
        .inspect(|ty| log::trace!("deduced Builtin {}: {ty}", self.id()))
    }
}

impl DeduceResult for AssignmentStatement {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        let ty = self.assignment.expression.deduce_result(context)?;
        context.set_local_value(self.assignment.id.clone(), Value::Type(ty))?;
        Ok(Type::Invalid).inspect(|ty| log::trace!("deduced AssignmentStatement: {ty}"))
    }
}

impl DeduceResult for UseStatement {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        if !context.is_module() {
            let _ = match &self.decl {
                UseDeclaration::Use(name) => context.use_symbol(name, None),
                UseDeclaration::UseAll(name) => context.use_symbols_of(name),
                UseDeclaration::UseAs(name, id) => context.use_symbol(name, Some(id.clone())),
            };
        }
        Ok(Type::Invalid).inspect(|ty| log::trace!("deduced UseStatement: {ty}"))
    }
}

impl DeduceResult for Parameter {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        if let Some(specified_type) = &self.specified_type {
            Ok(specified_type.ty())
        } else {
            let default_value = match (&self.specified_type, &self.default_value) {
                (Some(specified_type), Some(default_value)) => {
                    let value: Value = default_value.eval(context)?;
                    if specified_type.ty() != value.ty() {
                        context.error(
                            &self.src_ref,
                            EvalError::TypeMismatch {
                                id: self.id.clone(),
                                expected: specified_type.ty(),
                                found: value.ty(),
                            },
                        )?;
                        Value::None
                    } else {
                        value
                    }
                }
                (None, Some(default_value)) => default_value.eval(context)?,
                _ => Value::None,
            };

            Ok(default_value.ty()).inspect(|ty| log::trace!("deduced Parameter: {ty}"))
        }
    }
}

impl DeduceResult for MethodCall {
    fn deduce_result(&self, context: &mut EvalContext) -> EvalResult<Type> {
        match context.lookup(&self.name, LookupTarget::Method) {
            Ok(method) => method.deduce_result(context),
            Err(_) => Ok(Type::Invalid),
        }
    }
}
