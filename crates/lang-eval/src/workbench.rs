// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench evaluation

use crate::{
    CallTrait, Eval, EvalContext, EvalError, EvalResult,
    context::{
        BuiltinItemFrame, ContextScope, WorkbenchGroupFrame, WorkbenchInitFrame, WorkpieceFrame,
    },
};

use microcad_builtin::BuiltinItem;
use microcad_lang_base::{DisplayWithCtx, PushDiag, SrcReferrer, element::Visibility};
use microcad_lang_resolve::{SymbolId, library::symbol};

use microcad_lang_types::{
    ArgumentValue, ArgumentValueList, ModelTree, Value,
    model::{Element, ModelTreeBuilderMut, Properties, Property, PropertyType, Workpiece},
};

impl Eval for symbol::workbench::Group {
    fn eval(&self, context: &mut EvalContext) -> EvalResult {
        context.scope(WorkbenchGroupFrame::new(), |context| {
            self.statements
                .iter()
                .try_for_each(|stmt| stmt.eval(context))?;
            Ok(context.model_tree_builder_mut().build().into())
        })
    }
}

impl Eval for symbol::workbench::WorkbenchIf {
    fn eval(&self, context: &mut EvalContext) -> EvalResult {
        let cond: Value = self.cond.eval(context)?;
        let cond: bool = match cond.try_into() {
            Ok(cond) => cond,
            Err(err) => {
                return context.catch(EvalError::IfConditionIsNotBool {
                    condition_src_ref: self.cond.src_ref(),
                    src_ref: self.src_ref,
                    err,
                });
            }
        };

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

impl Eval for BuiltinItem {
    fn eval(&self, _context: &mut EvalContext) -> EvalResult {
        match self {
            BuiltinItem::Constant(builtin_constant) => Ok(builtin_constant.value()),
            _ => todo!("Error handling: Builtin constant expected"),
        }
    }
}

impl Eval for symbol::SymbolDef {
    fn eval(&self, context: &mut EvalContext) -> EvalResult {
        match self {
            symbol::SymbolDef::Constant(constant) => match constant.value() {
                Some(value) => Ok(value.clone()),
                None => context.catch(EvalError::ConstantExpressionExpected {
                    src_ref: constant.expr.src_ref(),
                }),
            },
            _ => todo!("Error handling: Constant symbol expected"),
        }
    }
}

impl Eval for symbol::SymbolId {
    fn eval(&self, context: &mut EvalContext) -> EvalResult {
        use crate::context::ContextScope;

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
            SymbolId::External { .. } => todo!(),
        }
    }
}

impl Eval for symbol::Path {
    fn eval(&self, context: &mut EvalContext) -> EvalResult {
        match self {
            symbol::Path::Resolved(symbol_id)
            | symbol::Path::HumanReadable { id: symbol_id, .. } => symbol_id.eval(context),
            symbol::Path::Unresolved(unresolved_path) => context.catch(EvalError::UnresolvedPath {
                path: unresolved_path.to_string(),
                src_ref: unresolved_path.src_ref,
            }),
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

impl Eval for symbol::workbench::WorkbenchCall {
    fn eval(&self, context: &mut EvalContext) -> EvalResult {
        match &self.path {
            symbol::Path::Resolved(symbol::SymbolId::Builtin(builtin_id)) => {
                let args = self.args.eval(context)?;
                let item = context.builtins.get(*builtin_id);
                let src_ref = context.current_symbol_src_ref();
                match item {
                    Some(item) => context.scope(BuiltinItemFrame::new(src_ref, item), |ctx| {
                        item.call(&args, ctx)
                    }),
                    None => context.catch(EvalError::BuiltinNotFound {
                        name: builtin_id.to_string_with_ctx(context),
                        src_ref: context.current_symbol_src_ref(),
                    }),
                }
            }
            path => context.catch(EvalError::SymbolCannotBeCalled {
                path: path.to_string(),
                src_ref: self.src_ref,
            }),
        }
    }
}

impl Eval for symbol::WorkbenchExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult {
        match &self {
            symbol::WorkbenchExpression::Invalid => unreachable!(),
            symbol::WorkbenchExpression::Value(constant_value) => {
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

/// Evaluate an [`InitStatement`] into an input property.
///
/// This does not throw an error if the property is not as workbench parameter.
impl Eval<Property> for symbol::workbench::InitStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Property> {
        let value: Value = self.expression.eval(context)?;
        Ok(Property::input(self.name.clone(), value).with_src_ref(self.src_ref))
    }
}

impl CallTrait<ModelTree> for symbol::Workbench {
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<ModelTree> {
        fn eval_to_model(
            workbench: &symbol::Workbench,
            input_properties: Properties,
            context: &mut EvalContext,
        ) -> EvalResult<ModelTree> {
            context.scope(
                WorkpieceFrame::new(Workpiece::new(workbench.signature.kind)),
                |context| -> EvalResult<ModelTree> {
                    context
                        .model_tree_builder_mut()
                        .add_model_properties(input_properties.into_iter().map(|(_, prop)| prop));
                    workbench
                        .statements
                        .iter()
                        .try_for_each(|stmt| stmt.eval(context))?;
                    Ok(context.model_tree_builder_mut().build())
                },
            )
        }

        use crate::ArgumentMatch;

        let matching_inits: Vec<_> = self
            .signature
            .inits
            .iter()
            .filter(|iter| iter.is_matching(args))
            .collect();

        match matching_inits.len() {
            0 => Err(Box::new(EvalError::NoInitializationFound {
                src_ref: self.signature.parameters.src_ref,
                path: context.current_symbol_name().unwrap_or_default(),
                arguments: args.to_string(),
                inits: self
                    .signature
                    .inits
                    .iter()
                    .map(|init| init.to_string())
                    .collect(),
            })),
            1 => {
                let init = matching_inits.first().unwrap();
                let mut models = Vec::new();

                match init.argument_multi_match(args) {
                    Ok(arguments) => {
                        for args in arguments {
                            let properties = context.scope(
                                WorkbenchInitFrame::new(args),
                                |context| -> EvalResult<Properties> {
                                    let mut properties = Properties::new();
                                    init.statements.iter().try_for_each(
                                        |stmt| -> EvalResult<()> {
                                            let property: Property = stmt.eval(context)?;
                                            properties.set_property(property);
                                            Ok(())
                                        },
                                    )?;
                                    Ok(properties)
                                },
                            )?;

                            models.push(eval_to_model(self, properties, context)?);
                        }
                    }
                    Err(_) => todo!(),
                }

                Ok(ModelTree::to_multiplicity(models))
            }
            _n => Err(Box::new(EvalError::AmbiguousInitialization {
                src_ref: self.signature.parameters.src_ref,
                path: context.current_symbol_name().unwrap_or_default(),
                arguments: args.to_string(),
                inits: matching_inits.iter().map(|init| init.to_string()).collect(),
            })),
        }
    }
}
