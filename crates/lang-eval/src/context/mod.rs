// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod stack;

pub use stack::*;

use microcad_builtin::BuiltinRegistry;
use microcad_lang_base::{LookUpName, Name, PushDiag};
use microcad_lang_resolve::symbol;
use microcad_lang_types::{Value, model::ModelTreeBuilderMut};

use crate::{EvalError, EvalResult};

#[derive(Debug, Default)]
pub struct EvalContext {
    stack: Stack,

    diag: Vec<Box<EvalError>>,

    pub builtins: BuiltinRegistry,

    pub arena: microcad_lang_types::model::Arena,
}

impl LookUpName for EvalContext {
    fn look_up_built_in_name(&self, builtin_id: &microcad_builtin::BuiltinId) -> Option<Name> {
        self.builtins.look_up_built_in_name(builtin_id)
    }
}

impl EvalContext {
    pub fn new() -> Self {
        let builtins = BuiltinRegistry::new();

        Self {
            builtins,
            ..Default::default()
        }
    }

    pub fn scope<T>(&mut self, frame: impl Into<StackFrame>, f: impl FnOnce(&mut Self) -> T) -> T {
        // 1. Temporarily swap out the stack to avoid self-borrow issues
        let mut stack = std::mem::take(&mut self.stack);
        stack.push(frame);
        self.stack = stack;

        // 2. Define a guard that pops the frame on Drop
        struct PopGuard<'a>(&'a mut EvalContext);

        impl<'a> Drop for PopGuard<'a> {
            fn drop(&mut self) {
                self.0.stack.pop();
            }
        }

        // 3. Instantiate the guard
        let _guard = PopGuard(self);

        // 4. Run the closure. When `_guard` goes out of scope right after this,
        // it will execute `self.stack.pop()` even if `f` panics or short-circuits.
        f(_guard.0)
    }

    pub fn top(&self) -> &StackFrame {
        self.stack.top()
    }

    pub fn top_mut(&mut self) -> &mut StackFrame {
        self.stack.top_mut()
    }

    pub(crate) fn eval_constant_symbol(
        &mut self,
        node_id: symbol::SymbolNodeId,
    ) -> EvalResult<Value> {
        /// Local up a symbol by its node id in the current package
        fn look_up_symbol(_node_id: symbol::SymbolNodeId) -> Option<symbol::SymbolDef> {
            todo!()
        }

        let symbol = match look_up_symbol(node_id) {
            Some(symbol) => symbol,
            None => todo!("Error handling: Symbol {node_id} not found"),
        };
        use crate::Eval;
        symbol.eval(self)
    }
}

impl ContextScope for EvalContext {
    fn look_up_local(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.stack.look_up_local(name)
    }

    fn current_symbol_name(&self) -> Option<String> {
        self.stack.current_symbol_name()
    }
}

impl StackRead for EvalContext {
    type Frame = StackFrame;

    fn get_local(&self, name: &Name) -> Option<&Value> {
        self.stack.get_local(name)
    }

    fn top(&self) -> &Self::Frame {
        self.stack.top()
    }
}

impl StackWrite for EvalContext {
    fn pop(&mut self) -> Self::Frame {
        self.stack.pop()
    }

    fn push(&mut self, frame: impl Into<Self::Frame>) {
        self.stack.push(frame);
    }

    fn top_mut(&mut self) -> &mut Self::Frame {
        self.stack.top_mut()
    }
}

impl ModelTreeBuilderMut for EvalContext {
    fn model_tree_builder_mut(&mut self) -> &mut microcad_lang_types::model::ModelTreeBuilder {
        self.top_mut().model_tree_builder_mut()
    }
}

impl PushDiag<Box<EvalError>> for EvalContext {
    fn push_diag(&mut self, err: impl Into<Box<EvalError>>) {
        self.diag.push(err.into())
    }
}
