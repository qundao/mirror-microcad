// Copyright © 2024-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation

use crate::{eval::*, model::*, render::Hashed, syntax::*};

impl WorkbenchDefinition {
    /// Try to evaluate a single call into a [`Model`].
    ///
    /// - `arguments`: Single argument tuple (will not be multiplied).
    /// - `init`: Initializer to call with given `arguments`.
    /// - `context`: Current evaluation context.
    fn eval_to_model<'a>(
        &'a self,
        call_src_ref: SrcRef,
        creator: Creator,
        init: Option<&'a InitDefinition>,
        context: &mut EvalContext,
    ) -> EvalResult<Model> {
        log::debug!(
            "Evaluating model of `{id:?}` {kind}",
            id = self.id,
            kind = self.kind
        );

        let arguments = creator.arguments.clone();

        // copy all arguments which are part of the building plan into properties
        let (mut properties, non_properties): (Vec<_>, Vec<_>) = arguments
            .named_iter()
            .map(|(id, value)| (id.clone(), value.clone()))
            .partition(|(id, _)| self.plan.contains_key(id));

        // create uninitialized values for all missing building plan properties
        let missing: Vec<_> = self
            .plan
            .iter()
            .filter(|param| !properties.iter().any(|(id, _)| param.id == *id))
            .map(|param| param.id.clone())
            .collect();
        missing
            .into_iter()
            .for_each(|id| properties.push((id, Value::None)));

        log::trace!("Properties: {properties:?}");
        log::trace!("Non-Properties: {non_properties:?}");

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
            StackFrame::Workbench(model, self.id.clone(), Default::default()),
            |context| {
                let model = context.get_model()?;

                // run init code
                if let Some(init) = init {
                    log::trace!(
                        "Initializing`{id:?}` {kind}",
                        id = self.id,
                        kind = self.kind
                    );
                    if let Err(err) = init.eval(non_properties.into_iter().collect(), context) {
                        context.error(&self.src_ref(), err)?;
                    }
                }

                // At this point, all properties must have a value
                log::trace!("Run body`{id:?}` {kind}", id = self.id, kind = self.kind);
                model.append_children(self.body.statements.eval(context)?);

                // We have to deduce the output type of this model, otherwise the model is incomplete.
                {
                    let model_ = model.borrow();
                    match &*model_.element {
                        Element::Workpiece(workpiece) => {
                            let output_type = model.deduce_output_type();

                            let result = workpiece.check_output_type(output_type);
                            match result {
                                Ok(()) => {}
                                Err(EvalError::WorkbenchNoOutput(..)) => {
                                    context.warning(&self.src_ref(), result.expect_err("Error"))?;
                                }
                                result => {
                                    context.error(&self.src_ref(), result.expect_err("Error"))?;
                                }
                            }
                        }
                        _ => panic!("A workbench must produce a workpiece."),
                    }
                }

                Ok(model)
            },
        )
    }
}

impl WorkbenchDefinition {
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
        log::debug!(
            "{call} workbench {kind} {id:?}({arguments:?})",
            call = crate::mark!(CALL),
            id = self.id,
            kind = self.kind
        );

        // prepare empty result model
        let mut models = Models::default();

        // match all initializations starting with the building plan
        let matches: Vec<_> = std::iter::once((
            None,
            self.plan
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
        // debug inspection of all matches/non-matches
        .inspect(|(i, m)| {
            let result = match m {
                Ok(m) => format!(
                    "{match_} [{priority:>10}]",
                    priority = m.priority,
                    match_ = crate::mark!(MATCH)
                ),
                Err(_) => crate::mark!(NO_MATCH),
            };
            if let Some(i) = i {
                log::debug!("{result} {}::init({})", symbol.full_name(), i.parameters)
            } else {
                log::debug!("{result} {}({})", symbol.full_name(), self.plan)
            }
        })
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
                                "{name}::init({params})",
                                name = symbol.full_name(),
                                params = init.parameters
                            )
                        }
                        None => format!(
                            "{name:?}({params})",
                            name = symbol.full_name(),
                            params = self.plan
                        ),
                    })
                    .collect::<Vec<_>>();
                log::debug!(
                    "{match_} Ambiguous initialization: {name}({arguments})\nCould be one of:\n{ambiguous}",
                    name = symbol.full_name(),
                    ambiguous = ambiguous.join("\n"),
                    match_ = crate::mark!(AMBIGUOUS)
                );
                context.error(
                    arguments,
                    EvalError::AmbiguousInitialization {
                        src_ref: call_src_ref,
                        name: self.id.clone(),
                        actual_params: arguments.to_string(),
                        ambiguous_params: ambiguous,
                    },
                )?;
            } else if let Some(matched) = matches.pop() {
                let what = if matched.0.is_none() {
                    "Building plan"
                } else {
                    "Initializer"
                };
                log::debug!(
                    "{match_} {what}: {}",
                    matched
                        .1
                        .args
                        .iter()
                        .map(|m| format!("{m:?}"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    match_ = crate::mark!(MATCH!)
                );

                // evaluate models for all multiplicity matches
                for arguments in matched.1.args.iter() {
                    models.push(self.eval_to_model(
                        call_src_ref.clone(),
                        Creator::new(symbol.clone(), arguments.clone()),
                        matched.0,
                        context,
                    )?);
                }
            }
        } else {
            log::debug!(
                "{match_} Neither the building plan nor any initializer matches arguments",
                match_ = crate::mark!(NO_MATCH!)
            );
            context.error(
                arguments,
                EvalError::NoInitializationFound {
                    src_ref: call_src_ref,
                    name: self.id.clone(),
                    actual_params: arguments.to_string(),
                    possible_params: self.possible_params(),
                },
            )?;
        }

        Ok(models.to_multiplicity(self.src_ref()))
    }
}
