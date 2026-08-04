// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_types::{Tuple, Value};

use crate::{BuiltinError, BuiltinEvalContext};

pub fn unpack_binary_args(
    args: Tuple,
    context: &mut BuiltinEvalContext,
) -> Result<(Value, Value), BuiltinError> {
    let lhs = args.get_named_unchecked("lhs");
    let rhs = args.get_named_unchecked("rhs");

    Ok((lhs.clone(), rhs.clone()))

    /*
        _ => Err(BuiltinError::TypeMismatch {
            name: context.current_fn(),
            expected: "2-element Tuple or Array",
            found: "Non-container or wrong size",
        }),
    }*/
}
