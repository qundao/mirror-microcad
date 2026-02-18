// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
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
            Self::None => write!(f, "<NONE>"),
            Self::Default => write!(f, "Default"),
            Self::TypeAuto => write!(f, "TypeAuto"),
            Self::Type => write!(f, "Type"),
            Self::Short => write!(f, "Short"),
            Self::Id => write!(f, "Id"),
            Self::Empty => write!(f, "Empty"),
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

    /// Create new instance and do matching
    fn new(arguments: &'a ArgumentValueList, params: &'a ParameterValueList) -> EvalResult<Self> {
        let mut am = Self {
            arguments: arguments.iter().map(|(id, v)| (id, v)).collect(),
            params: params.iter().collect(),
            result: Tuple::new_named(std::collections::HashMap::new(), arguments.src_ref()),
            priority: Priority::None,
        };

        // Try to match all arguments with different strategies.
        // The highest priority (see [Priority::high_to_low] for order) sets the
        // result's overall priority which can the be used to select between
        // available parameter sets (see [WorkbenchDefinition::call}).
        log::trace!("matching arguments:\n{am:?}");
        if !am.match_empty(Priority::Empty) {
            am.match_ids(Priority::Id, |l, r| l == r)?;
            am.match_ids(Priority::Short, |l, r| &l.short_id() == r)?;
            am.match_types(Priority::Type, |l, r| l == r, true);
            am.match_types(Priority::TypeAuto, |l, r| l.is_matching(r), false);
            am.match_defaults(Priority::Default);
            // TODO: this extra step is useful (but not documented!)
            am.match_types(Priority::TypeAuto, |l, r| l.is_matching(r), false);

            am.check_missing()?;
        }
        Ok(am)
    }

    /// Match empty parameters with empty arguments.
    fn match_empty(&mut self, priority: Priority) -> bool {
        if self.arguments.is_empty() && self.params.is_empty() {
            self.priority.set_once(priority);
            true
        } else {
            false
        }
    }

    /// Match arguments by id.
    fn match_ids(
        &mut self,
        priority: Priority,
        match_fn: impl Fn(&Identifier, &Identifier) -> bool,
    ) -> EvalResult<()> {
        let mut type_mismatch = Vec::new();
        if self.arguments.is_empty() {
            return Ok(());
        }
        self.arguments.retain(|(id, arg)| {
            let id = match (id.is_empty(), &arg.inline_id) {
                (true, Some(id)) => id,
                _ => id,
            };

            if id.is_empty() {
                return true;
            }
            match self.params.iter().position(|(i, _)| match_fn(i, id)) {
                None => true,
                Some(n) => {
                    if let Some(ty) = &self.params[n].1.specified_type {
                        if !arg.ty().is_matching(ty) {
                            type_mismatch.push((id.clone(), arg.ty(), ty));
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
                    false
                }
            }
        });

        if type_mismatch.is_empty() {
            Ok(())
        } else {
            let type_mismatch = type_mismatch
                .iter()
                .map(|(id, ty1, ty2)| format!("{id}: {ty1} != {ty2}"))
                .collect::<Vec<_>>()
                .join(", ");
            Err(EvalError::IdMatchButNotType(type_mismatch))
        }
    }

    /// Match arguments by type.
    fn match_types(
        &mut self,
        priority: Priority,
        match_fn: impl Fn(&Type, &Type) -> bool,
        mut exclude_defaults: bool,
    ) {
        if self.arguments.is_empty() {
            return;
        }

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

    /// Match arguments with parameter defaults.
    fn match_defaults(&mut self, priority: Priority) {
        if self.params.is_empty() {
            return;
        }
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
    let _ = env_logger::try_init();
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
