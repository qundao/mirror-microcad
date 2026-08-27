// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation

use crate::{
    CallTrait, Eval, EvalContext, EvalError, EvalResult,
    context::{WorkbenchGroupFrame, WorkbenchInitFrame, WorkpieceFrame},
};

use microcad_builtin::{BuiltinEvalContext, BuiltinItem};
use microcad_lang_base::{SrcReferrer, element::Visibility};
use microcad_package::{SymbolId, symbol};

use microcad_lang_types::{
    ArgumentValue, ArgumentValueList, ModelTree, Value,
    model::{Element, ModelTreeBuilderMut, Properties, Property, PropertyType, Workpiece},
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
        use crate::ArgumentMatch;
        match &self.path {
            symbol::Path::Resolved(symbol::SymbolId::Builtin(builtin_id)) => {
                let args = self.args.eval(context)?;

                match context.builtins.get(*builtin_id) {
                    Some(BuiltinItem::Function(f)) => {
                        let args = f.argument_match(&args)?;

                        Ok(f.call_isolated(args)?)
                    }
                    Some(BuiltinItem::Primitive(p)) => {
                        let multi_args = p.argument_multi_match(&args)?;
                        let mut models = Vec::new();
                        for args in multi_args {
                            models.push(ModelTree::from((p.f)(
                                args,
                                &mut BuiltinEvalContext::default(),
                            )?));
                        }

                        Ok(ModelTree::to_multiplicity(models).into())
                    }
                    Some(BuiltinItem::Operation(op)) => {
                        let multi_args = op.argument_multi_match(&args)?;
                        let mut models = Vec::new();
                        for args in multi_args {
                            models.push(ModelTree::from((op.f)(
                                args,
                                &mut BuiltinEvalContext::default(),
                            )?));
                        }

                        Ok(ModelTree::to_multiplicity(models).into())
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
            0 => todo!("Error handling: No matching init"),
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

                            models.push(eval_to_model(&self, properties, context)?);
                        }
                    }
                    Err(_) => todo!(),
                }

                Ok(ModelTree::to_multiplicity(models))
            }
            _n => {
                todo!("Ambiguous initializer");
            }
        }
    }
}
