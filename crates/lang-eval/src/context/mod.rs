// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod stack;

pub use stack::*;

use microcad_builtin::BuiltinRegistry;
use microcad_lang_base::Name;
use microcad_lang_types::{ModelTree, Value, model::Property};

use crate::{EvalError, EvalResult};

#[derive(Debug, Default)]
pub struct EvalContext {
    stack: Stack,

    diag: Vec<EvalError>,

    pub builtins: BuiltinRegistry,

    pub arena: microcad_lang_types::model::Arena,
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

    /// Push a diagnostic
    pub fn diag(&mut self, diag: impl Into<EvalError>) {
        self.diag.push(diag.into());
    }

    pub fn top(&self) -> &StackFrame {
        self.stack.top()
    }

    pub fn top_mut(&mut self) -> &mut StackFrame {
        self.stack.top_mut()
    }

    pub(crate) fn model_add_property(&mut self, _property: Property) -> EvalResult<()> {
        todo!()
    }

    pub(crate) fn model_append_child(&mut self, _model: ModelTree) {
        todo!()
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
