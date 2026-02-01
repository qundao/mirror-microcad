// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument match trait

use crate::{eval::*, value::*};

/// Match priorities
///
/// Argument matching in µcad is complex and comes in several priority layers.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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

impl Priority {
    pub(super) fn high_to_low() -> &'static [Priority] {
        &[
            Self::Empty,
            Self::Id,
            Self::Short,
            Self::Type,
            Self::TypeAuto,
            Self::Default,
        ]
    }

    fn set_once(&mut self, with: Self) {
        if *self == Priority::None {
            *self = with
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::None => write!(f, "<NONE>"),
            Priority::Default => write!(f, "Default/1"),
            Priority::TypeAuto => write!(f, "TypeAuto/2"),
            Priority::Type => write!(f, "Type/3"),
            Priority::Short => write!(f, "Short/4"),
            Priority::Id => write!(f, "Id/5"),
            Priority::Empty => write!(f, "Empty/5"),
        }
    }
}

/// Matching of `ParameterList` with `ArgumentValueList` into Tuple
pub struct ArgumentMatch<'a> {
    arguments: Vec<(&'a Identifier, &'a ArgumentValue)>,
    params: Vec<(&'a Identifier, &'a ParameterValue)>,
    result: Tuple,
    priority: Priority,
}

/// Result of a multi match
#[derive(Debug)]
pub struct MultiMatchResult {
    /// Matching arguments
    pub args: Vec<Tuple>,
    /// Match priority
    pub priority: Priority,
}

impl<'a> ArgumentMatch<'a> {
    /// Match a `ParameterList` with an `ArgumentValueList` into a tuple.
    ///
    /// Returns `Ok(Tuple)`` if matches or Err() if not
    pub fn find_match(
        arguments: &'a ArgumentValueList,
        params: &'a ParameterValueList,
    ) -> EvalResult<Tuple> {
        let am = Self::new(arguments, params)?;
        am.check_exact_types(params)?;
        Ok(am.result)
    }

    /// Match a `ParameterList` with an `ArgumentValueList` into an vector of tuples.
    ///
    /// Returns `Ok(MultiMatchResult)`` if matches or Err() if not
    pub fn find_multi_match(
        arguments: &'a ArgumentValueList,
        params: &'a ParameterValueList,
    ) -> EvalResult<MultiMatchResult> {
        let m = Self::new(arguments, params)?;
        Ok(MultiMatchResult {
            args: m.multiply(params),
            priority: m.priority,
        })
    }

    /// Create new instance and start matching.
    fn new(arguments: &'a ArgumentValueList, params: &'a ParameterValueList) -> EvalResult<Self> {
        let mut am = Self {
            arguments: arguments.iter().map(|(id, v)| (id, v)).collect(),
            params: params.iter().collect(),
            result: Tuple::new_named(std::collections::HashMap::new(), arguments.src_ref()),
            priority: Priority::None,
        };

        fn match_id_exact(left: &Identifier, right: &Identifier) -> bool {
            left == right
        }

        fn match_id_short(left: &Identifier, right: &Identifier) -> bool {
            left.short_id() == *right
        }

        fn match_type_exact(left: &Type, right: &Type) -> bool {
            left == right
        }

        fn match_type_auto(left: &Type, right: &Type) -> bool {
            left.is_matching(right)
        }

        log::trace!("matching arguments:\n{am:?}");
        if !am.match_empty(Priority::Empty) {
            am.match_ids(match_id_exact, Priority::Id)?;
            am.match_ids(match_id_short, Priority::Short)?;
            am.match_types(true, match_type_exact, Priority::Type);
            am.match_types(false, match_type_auto, Priority::TypeAuto);
            am.match_defaults(Priority::Default);
            am.match_types(false, match_type_auto, Priority::TypeAuto);

            am.check_missing()?;
        }
        Ok(am)
    }

    fn match_empty(&mut self, priority: Priority) -> bool {
        if self.arguments.is_empty() && self.params.is_empty() {
            self.priority.set_once(priority);
            true
        } else {
            false
        }
    }

    /// Match arguments by id
    fn match_ids(
        &mut self,
        match_fn: impl Fn(&Identifier, &Identifier) -> bool,
        priority: Priority,
    ) -> EvalResult<()> {
        let mut type_mismatch = IdentifierList::default();
        if !self.arguments.is_empty() {
            self.arguments.retain(|(id, arg)| {
                let id = match (id.is_empty(), &arg.inline_id) {
                    (true, Some(id)) => id,
                    _ => id,
                };

                if !id.is_empty() {
                    if let Some(n) = self.params.iter().position(|(i, _)| match_fn(i, id)) {
                        if let Some(ty) = &self.params[n].1.specified_type {
                            if !arg.ty().is_matching(ty) {
                                type_mismatch.push(id.clone());
                                return true;
                            }
                        }
                        let (id, _) = self.params.swap_remove(n);
                        log::trace!(
                            "{found} parameter by id: {id:?}",
                            found = crate::mark!(MATCH)
                        );
                        self.priority.set_once(priority);
                        self.result.insert((*id).clone(), arg.value.clone());
                        return false;
                    }
                }
                true
            });
        }
        if type_mismatch.is_empty() {
            Ok(())
        } else {
            Err(EvalError::IdMatchButNotType(type_mismatch.to_string()))
        }
    }

    /// Match arguments by type
    fn match_types(
        &mut self,
        mut exclude_defaults: bool,
        match_fn: impl Fn(&Type, &Type) -> bool,
        priority: Priority,
    ) {
        if !self.arguments.is_empty() {
            self.arguments.retain(|(arg_id, arg)| {
                // filter params by type
                let same_type: Vec<_> = self
                    .params
                    .iter()
                    .enumerate()
                    .filter(|(..)| arg_id.is_empty())
                    .filter_map(|(n, (id, param))| {
                        if param.ty() == Type::Invalid
                            || if let Some(ty) = &param.specified_type {
                                match_fn(&arg.ty(), ty)
                            } else {
                                false
                            }
                        {
                            Some((n, *id, *param))
                        } else {
                            None
                        }
                    })
                    .collect();

                // if type check is exact ignore exclusion
                if same_type.len() == 1 {
                    exclude_defaults = false;
                }
                // ignore params with defaults
                let mut same_type = same_type
                    .into_iter()
                    .filter(|(.., param)| !exclude_defaults || param.default_value.is_none());

                if let Some((n, id, _)) = same_type.next() {
                    if same_type.next().is_none() {
                        log::trace!(
                            "{found} parameter by type: {id:?}",
                            found = crate::mark!(MATCH)
                        );
                        self.priority.set_once(priority);
                        self.result.insert(id.clone(), arg.value.clone());
                        self.params.swap_remove(n);
                        return false;
                    } else {
                        log::trace!("more than one parameter with that type")
                    }
                } else {
                    log::trace!("no parameter with that type (or id mismatch)")
                }
                true
            })
        }
    }

    /// Fill arguments with defaults
    fn match_defaults(&mut self, priority: Priority) {
        if !self.params.is_empty() {
            // remove missing that can be found
            self.params.retain(|(id, param)| {
                // check for any default value
                if let Some(def) = &param.default_value {
                    // paranoia check if type is compatible
                    if def.ty() == param.ty() {
                        log::trace!(
                            "{found} argument by default: {id:?} = {def}",
                            found = crate::mark!(MATCH)
                        );
                        self.priority.set_once(priority);
                        self.result.insert((*id).clone(), def.clone());
                        return false;
                    }
                }
                true
            })
        }
    }

    /// Return error if params are missing or arguments are to many
    fn check_missing(&self) -> EvalResult<()> {
        if !self.params.is_empty() {
            let mut missing: IdentifierList =
                self.params.iter().map(|(id, _)| (*id).clone()).collect();
            missing.sort();
            Err(EvalError::MissingArguments(missing))
        } else if !self.arguments.is_empty() {
            let mut too_many: IdentifierList =
                self.arguments.iter().map(|(id, _)| (*id).clone()).collect();
            too_many.sort();
            Err(EvalError::TooManyArguments(too_many))
        } else {
            Ok(())
        }
    }

    fn check_exact_types(&self, params: &ParameterValueList) -> EvalResult<()> {
        let multipliers = Self::multipliers(&self.result, params);
        if multipliers.is_empty() {
            return Ok(());
        }
        Err(EvalError::MultiplicityNotAllowed(multipliers))
    }

    /// Process parameter multiplicity
    ///
    /// Return one or many tuples.
    fn multiply(&self, params: &ParameterValueList) -> Vec<Tuple> {
        let ids: IdentifierList = Self::multipliers(&self.result, params);
        if !ids.is_empty() {
            let mut result = Vec::new();
            self.result.multiplicity(ids, |t| result.push(t));
            result
        } else {
            vec![self.result.clone()]
        }
    }

    /// Return the multipliers' ids in the arguments.
    fn multipliers(args: &impl ValueAccess, params: &ParameterValueList) -> IdentifierList {
        let mut result: IdentifierList = params
            .iter()
            .filter_map(|(id, param)| {
                if let Some(a) = args.by_id(id) {
                    if a.ty().is_array_of(&param.ty()) {
                        return Some(id);
                    }
                }
                None
            })
            .cloned()
            .collect();
        result.sort();
        result
    }
}

impl std::fmt::Debug for ArgumentMatch<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "   Arguments: {args}\n  Parameters: {params}",
            args = self
                .arguments
                .iter()
                .map(|(id, arg)| format!(
                    "{id:?} : {val:?}",
                    id = match (id.is_empty(), &arg.inline_id) {
                        (_, None) => id,
                        (true, Some(inline_id)) => inline_id,
                        (false, Some(_)) => id,
                    },
                    val = arg.value
                ))
                .collect::<Vec<_>>()
                .join(", "),
            params = self
                .params
                .iter()
                .map(|(id, param)| format!("{id:?} {param:?}"))
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

#[test]
fn argument_matching() {
    use microcad_core::Length;
    let params: ParameterValueList = [
        crate::parameter!(a: Scalar),
        crate::parameter!(b: Length),
        crate::parameter!(c: Scalar),
        crate::parameter!(d: Length = Length::mm(4.0)),
    ]
    .into_iter()
    .collect();

    let arguments: ArgumentValueList = [
        crate::argument!(a: Scalar = 1.0),
        crate::argument!(b: Length = Length::mm(2.0)),
        crate::argument!(Scalar = 3.0),
    ]
    .into_iter()
    .collect();

    let result = ArgumentMatch::find_match(&arguments, &params).expect("expect valid arguments");

    assert_eq!(result, crate::tuple!("(a=1.0, b=2.0mm, c=3.0, d=4.0mm)"));
}

#[test]
fn argument_match_fail() {
    use microcad_core::Length;

    let params: ParameterValueList = [
        crate::parameter!(x: Scalar),
        crate::parameter!(y: Length),
        crate::parameter!(z: Area),
    ]
    .into_iter()
    .collect();
    let arguments: ArgumentValueList = [
        crate::argument!(x: Scalar = 1.0),
        crate::argument!(Length = Length::mm(1.0)),
    ]
    .into_iter()
    .collect();
    assert!(ArgumentMatch::find_match(&arguments, &params).is_err());
}
