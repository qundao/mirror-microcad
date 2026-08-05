// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument match trait

use derive_more::Display;
use microcad_lang_base::{Identifier, IdentifierList, SrcReferrer};
use microcad_lang_types::{Length, Scalar, Tuple, Ty, Type, Value, ValueAccess, create_tuple};
use microcad_package::{
    parameter,
    rst::{Parameter, ParameterList},
};

use crate::{ArgumentValue, ArgumentValueList, EvalError, EvalResult, argument};

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

/// Matching of `ParameterList` with `ArgumentValueList` into Tuple
pub struct ArgumentMatch<'a> {
    arguments: Vec<(&'a Identifier, &'a ArgumentValue)>,
    params: Vec<(&'a Identifier, &'a Parameter)>,
    result: Tuple,
}

/// Result of a multi match
#[derive(Debug)]
pub struct MultiMatchResult(Vec<Tuple>);

impl<'a> ArgumentMatch<'a> {
    /// Match a `ParameterList` with an `ArgumentValueList` into a tuple.
    ///
    /// Returns `Ok(Tuple)` if matches or `Err(...)` if matching fails.
    pub fn find_match(
        arguments: &'a ArgumentValueList,
        params: &'a ParameterList,
    ) -> EvalResult<Tuple> {
        // Track evaluated values per parameter index
        let mut bound_values: Vec<Option<Value>> = vec![None; params.len()];

        // -----------------------------------------------------------------
        // Step 1: Bind caller arguments to parameter slots
        // -----------------------------------------------------------------
        for arg in arguments.iter() {
            let mut target_idx = None;

            // Attempt 1: Named match (arg.id matches param.id)
            if let Some(ref arg_id) = arg.id {
                target_idx = params.iter().position(|param| &param.id == arg_id);
            }

            // Attempt 2: Positional match (first unbound parameter slot)
            if target_idx.is_none() {
                target_idx = bound_values.iter().position(|slot| slot.is_none());
            }

            if let Some(idx) = target_idx {
                if bound_values[idx].is_some() {
                    return Err(EvalError::DuplicateArgument {
                        id: params[idx].id.clone(),
                    }
                    .into());
                }
                bound_values[idx] = Some(arg.value.clone());
            } else {
                return Err(EvalError::TooManyArguments {
                    given: arguments.len(),
                    expected: params.len(),
                }
                .into());
            }
        }

        // -----------------------------------------------------------------
        // Step 2: Build the final `Tuple.named` list (including defaults)
        // -----------------------------------------------------------------
        let mut named = Vec::with_capacity(params.len());

        for (i, param) in params.iter().enumerate() {
            let value = match bound_values[i].take() {
                Some(val) => val,
                None => match param.default_value {
                    Some(ref default_val) => default_val.clone(),
                    None => {
                        return Err(EvalError::MissingRequiredArgument {
                            id: param.id.clone(),
                        }
                        .into());
                    }
                },
            };

            named.push((param.id.clone(), value));
        }

        Ok(Tuple {
            positional: vec![],
            named,
        })
    }

    /// Match a `ParameterList` with an `ArgumentValueList` into an vector of tuples.
    ///
    /// Returns `Ok(MultiMatchResult)`` if matches or Err() if not
    pub fn find_multi_match(
        arguments: &'a ArgumentValueList,
        params: &'a ParameterList,
    ) -> EvalResult<MultiMatchResult> {
        todo!()
        /*Ok(MultiMatchResult {
            args: m.multiply(params),
        })*/
    }
}

#[test]
fn argument_matching() {
    let params: ParameterList = [
        parameter!(a: Scalar),
        parameter!(b: Length),
        parameter!(c: Scalar),
        parameter!(d: Length = Length::mm(4.0)),
    ]
    .into_iter()
    .collect();

    let arguments: ArgumentValueList = [
        argument!(a: Scalar = Scalar::from_num(1.0)),
        argument!(b: Length = Length::mm(2.0)),
        argument!(Scalar = Scalar::from_num(3.0)),
    ]
    .into_iter()
    .collect();

    let result = ArgumentMatch::find_match(&arguments, &params).expect("expect valid arguments");

    assert_eq!(
        result,
        create_tuple!(a = 1.0, b = Length::mm(2.0), c = 3.0, d = Length::mm(4.0))
    );
}

#[test]
fn argument_match_fail() {
    let params: ParameterList = [
        parameter!(x: Scalar),
        parameter!(y: Length),
        parameter!(z: Area),
    ]
    .into_iter()
    .collect();
    let arguments: ArgumentValueList = [
        argument!(x: Scalar = Scalar::from_num(1.0)),
        argument!(Length = Length::mm(1.0)),
    ]
    .into_iter()
    .collect();
    assert!(ArgumentMatch::find_match(&arguments, &params).is_err());
}
