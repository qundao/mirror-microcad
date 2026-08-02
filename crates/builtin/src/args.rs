// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_types::Value;

use crate::{BuiltinError, BuiltinEvalContext};

pub fn unpack_binary_args(
    args: Value,
    context: &mut BuiltinEvalContext,
) -> Result<(Value, Value), BuiltinError> {
    match args {
        Value::Array(array) => match array.as_slice() {
            [lhs, rhs] => Ok((lhs.clone(), rhs.clone())),
            slice => Err(BuiltinError::InvalidArgumentCount {
                name: context.current_fn(),
                expected: 2,
                found: slice.len(),
            }),
        },
        _ => Err(BuiltinError::TypeMismatch {
            name: context.current_fn(),
            expected: "2-element Tuple or Array",
            found: "Non-container or wrong size",
        }),
    }
}
