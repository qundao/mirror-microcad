// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Function call evaluation

use crate::{eval::*, syntax::*, value::*};

impl CallTrait for FunctionDefinition {
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
        match ArgumentMatch::find_multi_match(args, &self.signature.parameters.eval(context)?) {
            Ok(matches) => {
                let mut results = vec![];
                for args in matches.args {
                    let result: Value = context
                        .scope(StackFrame::Function(self.id(), args.into()), |context| {
                            self.body.statements.eval(context)
                        })?;
                    if let Some(return_type) = &self.signature.return_type {
                        assert!(
                            return_type.ty() == result.ty(),
                            "Unexpected function result type (MISSED IN RESOLVE)",
                        );
                        assert!(
                            return_type.ty() != Type::Model,
                            "Forbidden function result type Model (MISSED IN RESOLVE)",
                        );
                        results.push(result.un_return());
                    } else if result != Value::None {
                        unreachable!("Unexpected function result (MISSED IN RESOLVE)")
                    }
                }
                match results.len() {
                    0 => Ok(Value::None),
                    1 => Ok(results.first().expect("one result item").clone()),
                    _ => Ok(Value::Array(results.into_iter().collect())),
                }
            }
            Err(err) => {
                context.error(args, err)?;
                Ok(Value::None)
            }
        }
    }
}
