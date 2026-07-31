// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument value evaluation entity

use microcad_lang_types::{Array, Value};

use crate::{ArgumentValueList, EvalContext, EvalError, EvalResult};

/// Trait for calling methods of values
pub trait CallMethod<T = Value> {
    /// Evaluate method call into a value (if possible)
    ///
    /// - `name`: Name of the method
    /// - `args`: Arguments for the method
    /// - `context`: Evaluation context
    fn call_method(
        &self,
        id: &microcad_package::rst::ResolvedName,
        args: &ArgumentValueList,
        context: &mut EvalContext,
    ) -> EvalResult<T>;
}

impl CallMethod for Array {
    fn call_method(
        &self,
        id: &microcad_package::rst::ResolvedName,
        _: &ArgumentValueList,
        context: &mut EvalContext,
    ) -> EvalResult<Value> {
        use microcad_lang_base::SingleIdentifier;

        Ok(
            match id.single_identifier().expect("Single id").id().as_str() {
                "count" => self.len().into(),
                "first" | "head" => self.first(), // Keep head method as deprecated method.
                "last" => self.last(),
                "tail" => self.tail().into(),
                "rev" => self.rev().into(),
                "sorted" => self.sorted().into(),
                "all_equal" => self.all_equal().into(),
                "is_ascending" => self.is_ascending().into(),
                "is_descending" => self.is_descending().into(),
                _ => {
                    context.error(id, EvalError::UnknownMethod(id.clone()))?;
                    Value::None
                }
            },
        )
    }
}
