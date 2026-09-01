// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Constant fold.

use microcad_builtin::BuiltinItem;
use microcad_lang_base::SymbolId;
use microcad_lang_types::Arguments;

use crate::{
    LowerContext,
    ir::{self, ConstantValue},
};

pub struct Fold<'a, 'source> {
    context: &'a mut LowerContext<'source>,
}

impl<'a, 'source> Fold<'a, 'source> {
    pub fn new(context: &'a mut LowerContext<'source>) -> Self {
        Self { context }
    }
}

impl<Expr: ir::ExprSpec> ir::ArgumentList<Expr> {
    pub fn into_arguments(&self) -> Option<Arguments> {
        let mut args = Vec::new();

        for arg in &self.args {
            match arg.expr().value() {
                Some(value) => args.push(match arg {
                    ir::Argument::Named { name, .. } | ir::Argument::AutoNamed { name, .. } => {
                        (name.clone(), value.clone())
                    }
                    _ => return None,
                }),
                None => return None,
            }
        }

        Some(Arguments::from_iter(args))
    }
}

impl<'a, 'source> ir::visitor::LeafVisitorMut for Fold<'a, 'source> {}

impl<'a, 'source> ir::visitor::ConstantVisitorMut for Fold<'a, 'source> {
    fn visit_constant_expr(&mut self, expr: &mut ir::ConstantExpression) {
        match expr {
            ir::ConstantExpression::Invalid | ir::ConstantExpression::Value(_) => {}

            // Fold builtin constants into values
            ir::ConstantExpression::Path(path) => match path.symbol_id() {
                Some(SymbolId::Builtin(builtin_id)) => {
                    match self.context.builtins.get(*builtin_id) {
                        Some(BuiltinItem::Constant(constant)) => {
                            *expr = ir::ConstantExpression::Value(ConstantValue::from_value(
                                constant.value(),
                            ));
                        }
                        _ => {}
                    }
                }
                _ => {}
            },

            // Fold built-in calls
            ir::ConstantExpression::Call(call) => {
                call.args.args.iter_mut().for_each(|arg| match arg {
                    ir::Argument::Unnamed(expr) => self.visit_constant_expr(expr),
                    ir::Argument::Named { expr, .. } | ir::Argument::AutoNamed { expr, .. } => {
                        self.visit_constant_expr(expr);
                    }
                });

                let builtin_fn = match &call.path.symbol_id() {
                    Some(microcad_lang_base::SymbolId::Builtin(builtin_id)) => {
                        match self.context.builtins.get(*builtin_id) {
                            Some(BuiltinItem::Function(builtin_fn)) => builtin_fn,
                            _ => return,
                        }
                    }
                    _ => return,
                };

                match call.args.into_arguments() {
                    Some(arguments) => match builtin_fn.call_isolated(arguments) {
                        Ok(value) => {
                            *expr = ir::ConstantExpression::Value(ConstantValue::from_value(value))
                        }
                        Err(_) => return,
                    },
                    None => return,
                }
            }
        }
    }
}

// Sub-trait implementations (inherit default traversal behavior)
impl<'a, 'source> ir::visitor::WorkbenchStatementVisitorMut for Fold<'a, 'source> {}
impl<'a, 'source> ir::visitor::WorkbenchVisitorMut for Fold<'a, 'source> {}
impl<'a, 'source> ir::visitor::FnVisitorMut for Fold<'a, 'source> {}
impl<'a, 'source> ir::visitor::VisitorMut for Fold<'a, 'source> {}
