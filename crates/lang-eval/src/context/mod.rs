// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::From;

use microcad_builtin::BuiltinRegistry;
use microcad_lang_base::{BuiltinId, HashMap, Identifier};
use microcad_lang_types::{Arguments, Value};

use crate::EvalError;

/// A map of locals.
///
/// The `Vec<SrcRef>` represents the usages of this local.
#[derive(Debug, Default)]
pub struct LocalTable(HashMap<Identifier, Value>);

#[derive(Debug, Default)]
pub struct FunctionFrame {
    //symbol: mir::SymbolHandle,
    pub locals: LocalTable,
}

impl FunctionFrame {
    pub fn new(args: Arguments) -> Self {
        let locals = LocalTable(
            args.iter()
                .map(|(id, value)| (id.clone(), value.clone()))
                .collect(),
        );

        Self { locals }
    }
}

#[derive(Debug, Default)]
pub struct FunctionScopeFrame {
    //symbol: mir::SymbolHandle,
    pub locals: LocalTable,
}

impl FunctionScopeFrame {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, From)]
pub enum StackFrame {
    Function(FunctionFrame),
    FunctionScope(FunctionScopeFrame),
}

impl StackFrame {
    pub fn get_local(&self, id: &Identifier) -> Option<&Value> {
        match self {
            StackFrame::Function(FunctionFrame { locals })
            | StackFrame::FunctionScope(FunctionScopeFrame { locals }) => locals.0.get(id),
        }
    }

    pub fn put_local(&mut self, id: Identifier, value: Value) {
        match self {
            StackFrame::Function(FunctionFrame { locals })
            | StackFrame::FunctionScope(FunctionScopeFrame { locals }) => {
                locals.0.insert(id, value);
            }
        }
    }
}

/// A generic stack.
#[derive(Debug)]
pub struct Stack(Vec<StackFrame>);

impl Stack {
    pub fn new() -> Self {
        Self::default()
    }
}

impl StackRead for Stack {
    type Frame = StackFrame;

    fn get_local(&self, id: &Identifier) -> Option<&Value> {
        self.0.iter().rev().find_map(|frame| frame.get_local(id))
    }

    fn top(&self) -> &StackFrame {
        self.0.last().expect("A stack frame") // Intentionally no error handling here
    }
}

impl StackWrite for Stack {
    fn push(&mut self, frame: impl Into<StackFrame>) {
        self.0.push(frame.into());
    }

    fn pop(&mut self) -> StackFrame {
        self.0.pop().expect("A stack frame")
    }

    fn top_mut(&mut self) -> &mut StackFrame {
        self.0.last_mut().expect("A stack frame")
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self(vec![])
    }
}

pub trait StackRead {
    type Frame;

    fn get_local(&self, _id: &Identifier) -> Option<&Value> {
        None
    }

    fn top(&self) -> &Self::Frame;
}

pub trait StackWrite: StackRead {
    fn top_mut(&mut self) -> &mut Self::Frame;
    fn pop(&mut self) -> Self::Frame {
        unimplemented!("Implement stack pop")
    }
    fn push(&mut self, _: impl Into<Self::Frame>) {
        unimplemented!("Implement stack push")
    }
}

#[derive(Debug, Default)]
pub struct EvalContext {
    stack: Stack,

    diag: Vec<EvalError>,

    pub builtins: BuiltinRegistry,
}

impl EvalContext {
    pub fn new() -> Self {
        let mut builtins = BuiltinRegistry::new();

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
}

impl StackRead for EvalContext {
    type Frame = StackFrame;

    fn get_local(&self, id: &Identifier) -> Option<&Value> {
        self.stack.get_local(id)
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
