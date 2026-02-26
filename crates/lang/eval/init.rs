// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*, syntax::*};

impl InitDefinition {
    /// Evaluate a call to the init definition
    pub fn eval(&self, non_property_args: Tuple, context: &mut EvalContext) -> EvalResult<()> {
        let model = context.get_model()?;
        context.scope(StackFrame::Init(non_property_args.into()), |context| {
            let _: Value = self.body.statements.eval(context)?;

            if let Some(properties) = model.borrow().get_properties() {
                let missing: IdentifierList = properties
                    .iter()
                    .filter(|(_, value)| value.is_invalid())
                    .map(|(id, _)| id.clone())
                    .collect();

                if !missing.is_empty() {
                    context.error(self, EvalError::BuildingPlanIncomplete(missing))?;
                }
            }

            Ok(())
        })
    }
}
