// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval for Marker {
    fn eval(&self, _: &mut EvalContext) -> EvalResult<Value> {
        if self.is_input_placeholder() {
            Ok(Value::Model(
                ModelBuilder::new(Element::InputPlaceholder, self.src_ref()).build(),
            ))
        } else {
            Ok(Value::None)
        }
    }
}
