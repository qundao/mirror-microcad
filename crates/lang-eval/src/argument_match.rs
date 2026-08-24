// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument match trait

use derive_more::Display;
use microcad_lang_base::Identifier;
use microcad_lang_types::{ArgumentValueList, Arguments, FunctionType, Tuple, Type, Value};

use crate::{EvalError, EvalResult};

/// Match priorities
///
/// Argument matching in µcad is complex and comes in several priority layers.
#[derive(Clone, Copy, Display, Debug, PartialEq, PartialOrd)]
pub enum Priority {
    /// Matched empty parameter list
    Empty,
    /// Matched by exact identifier.
    Id,
    /// Matched by shortened identifier.
    Short,
    /// Matched by exact type.
    Type,
    /// Matched by automatic type conversion (e.g. Integer -> Scalar).
    TypeAuto,
    /// Matched by parameter default.
    Default,
    /// No priority (invalid).
    None,
}

/// Match an `ArgumentValueList` against a `FunctionType` signature and return resolved `Arguments`.
pub fn find_match(
    arguments: &ArgumentValueList,
    fn_ty: &FunctionType,
    default_parameters: &Tuple,
) -> EvalResult<Arguments> {
    // Handle variadic functions without fixed parameters
    let params_vec = match &fn_ty.parameters {
        Some(params) => params.0.iter().collect::<Vec<_>>(),
        None => {
            let named = arguments
                .iter()
                .enumerate()
                .map(|(i, arg)| {
                    let id = arg
                        .id
                        .clone()
                        .unwrap_or_else(|| Identifier::no_ref(&format!("_{i}")));
                    (id, arg.value.clone())
                })
                .collect();

            return Ok(Tuple {
                positional: vec![],
                named,
            }
            .into());
        }
    };

    let num_params = params_vec.len();
    let mut bound_values: Vec<Option<Value>> = vec![None; num_params];

    // -----------------------------------------------------------------
    // Step 1: Bind caller arguments to parameter slots
    // -----------------------------------------------------------------
    for arg in arguments.iter() {
        let mut target_idx = None;

        // 1a. Named match
        if let Some(ref arg_id) = arg.id {
            target_idx = params_vec
                .iter()
                .position(|(param_id, _)| param_id == arg_id);
        }

        // 1b. Positional match (first unbound slot)
        if target_idx.is_none() {
            target_idx = bound_values.iter().position(|slot| slot.is_none());
        }

        if let Some(idx) = target_idx {
            if bound_values[idx].is_some() {
                return Err(EvalError::DuplicateArgument {
                    id: params_vec[idx].0.clone(),
                }
                .into());
            }
            bound_values[idx] = Some(arg.value.clone());
        } else {
            return Err(EvalError::TooManyArguments {
                given: arguments.len(),
                expected: num_params,
            }
            .into());
        }
    }

    // -----------------------------------------------------------------
    // Step 2: Resolve missing values via default_parameters
    // -----------------------------------------------------------------
    let mut named = Vec::with_capacity(num_params);

    for (i, (param_id, _param_ty)) in params_vec.iter().enumerate() {
        let value = match bound_values[i].take() {
            Some(val) => val,
            None => {
                if let Some((_, default_val)) = default_parameters
                    .named
                    .iter()
                    .find(|(id, _)| id == param_id)
                {
                    default_val.clone()
                } else {
                    return Err(EvalError::MissingRequiredArgument {
                        id: (*param_id).clone(),
                    }
                    .into());
                }
            }
        };

        named.push(((*param_id).clone(), value));
    }

    Ok(Tuple {
        positional: vec![],
        named,
    }
    .into())
}

pub fn find_multi_match(
    arguments: &ArgumentValueList,
    fn_ty: &FunctionType,
    default_parameters: &Tuple,
) -> EvalResult<Vec<Arguments>> {
    // Step 1: Bind base arguments and resolve default values
    let base_args = find_match(arguments, fn_ty, default_parameters)?;

    // Step 2: Extract parameter metadata
    let params_map = match &fn_ty.parameters {
        Some(params) => &params.0,
        None => return Ok(vec![base_args]),
    };

    // Step 3: Identify which arguments need vectorization
    let mut resolved_lists: Vec<(Identifier, Vec<Value>)> =
        Vec::with_capacity(base_args.0.named.len());

    for (id, val) in base_args.0.named {
        let expected_ty = params_map
            .iter()
            .find(|(p_id, _)| p_id == &id)
            .map(|(_, ty)| ty);

        if let (Some(expected), Value::List(elements)) = (expected_ty, &val) {
            if expected.is_list_of(expected) || !expected.is_list_of(&Type::Any) {
                resolved_lists.push((id, elements.to_vec()));
            } else {
                resolved_lists.push((id, vec![val.clone()]));
            }
        } else {
            resolved_lists.push((id, vec![val.clone()]));
        }
    }

    // Step 4: Compute Cartesian product across all argument value lists
    let mut combinations: Vec<Vec<(Identifier, Value)>> = vec![vec![]];

    for (id, values) in resolved_lists {
        let mut next_combinations = Vec::new();
        for current_combination in combinations {
            for val in &values {
                let mut extended = current_combination.clone();
                extended.push((id.clone(), val.clone()));
                next_combinations.push(extended);
            }
        }
        combinations = next_combinations;
    }

    // Step 5: Convert combinations into Arguments
    let result = combinations
        .into_iter()
        .map(|named| {
            Tuple {
                positional: vec![],
                named,
            }
            .into()
        })
        .collect();

    Ok(result)
}
