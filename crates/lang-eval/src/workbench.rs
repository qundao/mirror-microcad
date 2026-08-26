// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation

use crate::{
    CallTrait, Eval, EvalContext, EvalError, EvalResult, context::WorkbenchGroupFrame, find_match,
};

use microcad_builtin::{BuiltinEvalContext, BuiltinItem};
use microcad_lang_base::{SrcReferrer, element::Visibility};
use microcad_package::{
    SymbolId,
    symbol::{self, ParameterList},
};

use microcad_lang_types::{
    ArgumentValue, ArgumentValueList, ModelTree, Value,
    model::{Element, ModelTreeBuilderMut, Properties, Property, PropertyType},
    tuple,
};

impl Eval<ModelTree> for symbol::workbench::Group {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ModelTree> {
        context.scope(WorkbenchGroupFrame::new(), |context| {
            self.statements
                .iter()
                .try_for_each(|stmt| stmt.eval(context))?;
            Ok(context.model_tree_builder_mut().build())
        })
    }
}

impl Eval<Value> for symbol::workbench::Group {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let model_tree: ModelTree = self.eval(context)?;
        Ok(model_tree.into())
    }
}

impl Eval<Value> for symbol::workbench::If {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let cond: Value = self.cond.eval(context)?;
        let cond: bool = cond.try_into()?;

        if cond {
            self.body.eval(context)
        } else if let Some(next_if) = &self.next_if {
            // Handle `else if ...` chain
            next_if.eval(context)
        } else if let Some(body) = &self.body_else {
            body.eval(context)
        } else {
            Ok(Value::None)
        }
    }
}

impl Eval<ModelTree> for symbol::workbench::Marker {
    fn eval(&self, _context: &mut EvalContext) -> EvalResult<ModelTree> {
        Ok(ModelTree::new(Element::InputPlaceholder))
    }
}

impl Eval<Value> for BuiltinItem {
    fn eval(&self, _context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            BuiltinItem::Constant(builtin_constant) => Ok(builtin_constant.value()),
            _ => todo!("Error handling: Builtin constant expected"),
        }
    }
}

impl Eval<Value> for symbol::SymbolDef {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            symbol::SymbolDef::Constant(constant) => match constant.value() {
                Some(value) => Ok(value.clone()),
                None => {
                    context.diag(EvalError::ConstantExpressionExpected {
                        src_ref: constant.expr.src_ref(),
                    });
                    Ok(Value::None)
                }
            },
            _ => todo!("Error handling: Constant symbol expected"),
        }
    }
}

impl Eval<Value> for symbol::SymbolId {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        use crate::context::Lookup;

        match &self {
            SymbolId::Builtin(builtin_id) => match context.builtins.get(*builtin_id) {
                Some(builtin) => Ok(builtin.eval(context)?),
                None => {
                    //context.diag(BuiltinError::NoBuiltin { full_name: (), id: *builtin_id })
                    Ok(Value::None)
                }
            },
            SymbolId::Local(local_id) => match context.look_up_local(local_id) {
                Some(value) => Ok(value.clone()),
                None => {
                    todo!("Error handling: Local '{local_id}' not found")
                }
            },
            SymbolId::Item(node_id) => context.eval_constant_symbol(*node_id),
            SymbolId::External { package_name, id } => {
                let symbol = context.look_up_external_symbol(package_name, *id).unwrap();
                symbol.eval(context)
            }
        }
    }
}

impl Eval<Value> for symbol::Path {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            symbol::Path::Resolved(symbol_id) => symbol_id.eval(context),
            symbol::Path::Unresolved(unresolved_path) => {
                context.diag(EvalError::UnresolvedPath {
                    path: unresolved_path.to_string(),
                    src_ref: unresolved_path.src_ref,
                });
                Ok(Value::None)
            }
            symbol::Path::HumanReadable { .. } => todo!(),
        }
    }
}

impl Eval<ArgumentValue> for symbol::workbench::Argument {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ArgumentValue> {
        Ok(match self {
            symbol::workbench::Argument::Unnamed(expr) => {
                let value: Value = expr.eval(context)?;
                ArgumentValue::new(value, None)
            }
            symbol::workbench::Argument::Named { name, expr, .. }
            | symbol::workbench::Argument::AutoNamed { name, expr } => {
                let value: Value = expr.eval(context)?;
                ArgumentValue::new(value, Some(name.clone()))
            }
        })
    }
}

impl Eval<ArgumentValueList> for symbol::workbench::ArgumentList {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ArgumentValueList> {
        let map: Vec<ArgumentValue> = self
            .args
            .iter()
            .map(|arg| arg.eval(context))
            .collect::<EvalResult<Vec<_>>>()?;

        Ok(ArgumentValueList {
            args: map,
            src_ref: self.src_ref,
        })
    }
}

impl Eval for symbol::workbench::Call {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self.path {
            symbol::Path::Resolved(symbol::SymbolId::Builtin(builtin_id)) => {
                let args = self.args.eval(context)?;

                match context.builtins.get(*builtin_id) {
                    Some(BuiltinItem::Function(f)) => {
                        let args = find_match(&args, &f.ty(), &tuple!())?;

                        Ok(f.call_isolated(args)?)
                    }
                    Some(BuiltinItem::Primitive(p)) => {
                        let args = find_match(&args, &(p.ty)(), &tuple!())?;
                        Ok((p.f)(args, &mut BuiltinEvalContext::default())?.into())
                    }
                    None => unimplemented!("Function not found: {builtin_id}"),
                    _ => todo!(),
                }
            }
            path => {
                context.diag(EvalError::SymbolCannotBeCalled {
                    path: path.to_string(),
                    src_ref: self.src_ref,
                });
                Ok(Value::None)
            }
        }
    }
}

impl Eval<Value> for symbol::WorkbenchExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            symbol::WorkbenchExpression::Invalid => unreachable!(),
            symbol::WorkbenchExpression::Constant(constant_value) => {
                Ok(constant_value.value().clone())
            }
            symbol::WorkbenchExpression::Path(path) => path.eval(context),
            symbol::WorkbenchExpression::Group(group) => group.eval(context),
            symbol::WorkbenchExpression::If(if_) => if_.eval(context),
            symbol::WorkbenchExpression::Call(call) => call.eval(context),
            symbol::WorkbenchExpression::Marker(marker) => Ok(marker.eval(context)?.into()),
            _ => unimplemented!(),
        }
    }
}

impl Eval<ModelTree> for symbol::WorkbenchExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ModelTree> {
        let value: Value = self.eval(context)?;
        Ok(value.into())
    }
}

impl Eval<()> for symbol::WorkbenchStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<()> {
        match &self.name {
            // If we have an id, this becomes a property in the current model
            Some(name) => {
                let property = Property {
                    name: name.clone(),
                    value: self.expression.eval(context)?,
                    src_ref: self.src_ref,
                    ty: match &self.visibility {
                        Visibility::Public => PropertyType::Output,
                        Visibility::Private => PropertyType::Hidden,
                    },
                };
                context
                    .model_tree_builder_mut()
                    .add_model_property(property);
            }
            // If we have no id, we have a child model.
            None => {
                let value: Value = self.expression.eval(context)?;
                // It might be that the value is none, because an if expression might not produce a model when its condition is not fulfilled
                if !value.is_none() {
                    context
                        .model_tree_builder_mut()
                        .add_model_child(ModelTree::from(value));
                }
            }
        }
        Ok(())
    }
}

/*
impl ir::WorkbenchDefinition {
    /// Try to evaluate a single call into a [`Model`].
    ///
    /// - `arguments`: Single argument tuple (will not be multiplied).
    /// - `init`: Initializer to call with given `arguments`.
    /// - `context`: Current evaluation context.
    fn eval_to_model<'a>(
        &'a self,
        call_src_ref: SrcRef,
        creator: Creator,
        init: Option<&'a ir::InitDefinition>,
        context: &mut EvalContext,
    ) -> EvalResult<Model> {
        let arguments = creator.arguments.clone();

        // copy all arguments which are part of the building plan into properties
        let (mut properties, non_properties): (Vec<_>, Vec<_>) = arguments
            .named_iter()
            .map(|(id, value)| (id.clone(), value.clone()))
            .partition(|(id, _)| self.parameters.contains_key(id));

        // create uninitialized values for all missing building plan properties
        let missing: Vec<_> = self
            .parameters
            .iter()
            .filter(|param| !properties.iter().any(|(id, _)| param.id_ref() == id))
            .map(|param| param.id())
            .collect();
        missing
            .into_iter()
            .for_each(|id| properties.push((id, Value::None)));

        // Create model
        let model = ModelBuilder::new(
            Element::Workpiece(Workpiece {
                kind: *self.kind,
                // copy all arguments which are part of the building plan to properties
                properties: properties.into_iter().collect(),
                creator: Hashed::new(creator),
            }),
            call_src_ref,
        )
        .attributes(self.attribute_list.eval(context)?)
        .build();

        context.scope(
            StackFrame::Workbench(model, self.id(), Default::default()),
            |context| {
                let model = context.get_model()?;

                // run init code
                if let Some(init) = init {
                    log::trace!(
                        "Initializing`{id:?}` {kind}",
                        id = self.id_ref(),
                        kind = self.kind
                    );
                    if let Err(err) = init.eval(non_properties.into_iter().collect(), context) {
                        context.error(&self.src_ref(), err)?;
                    }
                }

                // At this point, all properties must have a value
                log::trace!(
                    "Run body`{id:?}` {kind}",
                    id = self.id_ref(),
                    kind = self.kind
                );
                model.append_children(self.body.statements.eval(context)?);

                Ok(model)
            },
        )
    }
}

impl ir::WorkbenchDefinition {
    /// Evaluate the call of a workbench with given arguments.
    ///
    /// - `args`: Arguments which will be matched with the building plan and the initializers using parameter multiplicity.
    /// - `context`: Current evaluation context.
    ///
    /// Return evaluated nodes (multiple nodes might be created by parameter multiplicity).
    pub fn call(
        &self,
        call_src_ref: SrcRef,
        symbol: Symbol,
        arguments: &ArgumentValueList,
        context: &mut EvalContext,
    ) -> EvalResult<Model> {
        // prepare empty result model
        let mut models = Models::default();

        // match all initializations starting with the building plan
        let matches: Vec<_> = std::iter::once((
            None,
            self.parameters
                .eval(context)
                .and_then(|params| ArgumentMatch::find_multi_match(arguments, &params)),
        ))
        // chain the inits
        .chain(self.inits().map(|init| {
            (
                Some(init),
                init.parameters
                    .eval(context)
                    .and_then(|params| ArgumentMatch::find_multi_match(arguments, &params)),
            )
        }))
        // filter out non-matching
        .filter_map(|(i, m)| if let Ok(m) = m { Some((i, m)) } else { None })
        .collect();

        // find hightest priority matches
        let matches = Priority::high_to_low().iter().find_map(|priority| {
            let matches: Vec<_> = matches
                .iter()
                .filter(|(_, m)| m.priority == *priority)
                .collect();
            if matches.is_empty() {
                None
            } else {
                Some(matches)
            }
        });

        if let Some(mut matches) = matches {
            if matches.len() > 1 {
                let ambiguous = matches
                    .iter()
                    .map(|(init, _)| match init {
                        Some(init) => {
                            format!(
                                "{name}::{init}",
                                name = symbol.full_name(),
                                init = init.signature()
                            )
                        }
                        None => format!(
                            "{name}({params})",
                            name = symbol.full_name(),
                            params = self.parameters
                        ),
                    })
                    .collect::<Vec<_>>();

                context.error(
                    arguments,
                    EvalError::AmbiguousInitialization {
                        src_ref: call_src_ref,
                        name: self.id(),
                        actual_params: arguments.to_string(),
                        ambiguous_params: ambiguous,
                    },
                )?;
            } else if let Some(matched) = matches.pop() {
                // evaluate models for all multiplicity matches
                for arguments in matched.1.args.iter() {
                    models.push(self.eval_to_model(
                        call_src_ref,
                        Creator::new(symbol.clone(), arguments.clone()),
                        matched.0,
                        context,
                    )?);
                }
            }
        } else {
            context.error(
                arguments,
                EvalError::NoInitializationFound {
                    src_ref: call_src_ref,
                    name: self.id(),
                    actual_params: arguments.to_string(),
                    possible_params: self.possible_params(),
                },
            )?;
        }

        Ok(models.to_multiplicity(self.src_ref()))
    }
}
*/

pub trait InitExt {
    fn input_properties(
        &self,
        parameters: &ParameterList,
        context: &mut EvalContext,
    ) -> EvalResult<Properties>;
}

impl CallTrait<ModelTree> for symbol::Workbench {
    fn call(&self, _args: &ArgumentValueList, _context: &mut EvalContext) -> EvalResult<ModelTree> {
        // Find correct inits

        todo!();
        /*
        match crate::find_multi_match(args, &self.ty, &self.default_parameters) {
            Ok(args) => {
                for arg in args {
                    context.scope(WorkbenchFrame::new(args), |context| {
                        Ok(self.statements.eval(context)?)
                    })
                }
            }

            Err(_) => todo!(),
        }*/
    }
}
