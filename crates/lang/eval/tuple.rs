// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for TupleExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let (unnamed, named): (Vec<_>, _) = Eval::<ArgumentValueList>::eval(&self.args, context)?
            .iter()
            .map(|(id, arg)| (id.clone(), arg.value.clone()))
            .partition(|(id, _)| id.is_none());

        // check unnamed for ambiguous types
        let mut h = std::collections::HashSet::new();
        unnamed
            .iter()
            .map(|(_, value)| value.ty())
            .try_for_each(|ty| {
                if h.insert(ty.clone()) {
                    Ok(())
                } else {
                    Err(EvalError::AmbiguousType {
                        ty,
                        src_ref: self.src_ref.clone(),
                    })
                }
            })?;

        Ok(Value::Tuple(
            Tuple {
                named: named.into_iter().collect(),
                unnamed: unnamed.into_iter().map(|(_, v)| (v.ty(), v)).collect(),
                src_ref: self.src_ref.clone(),
            }
            .into(),
        ))
    }
}
