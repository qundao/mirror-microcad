// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve stack.

use derive_more::From;

use crate::{
    library::SymbolNodeId,
    resolve::locals::{LocalTable, Locals},
};

#[derive(Debug, Default)]
pub(crate) struct FunctionFrame {
    //symbol: mir::SymbolHandle,
    locals: LocalTable,
}

impl Locals for FunctionFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        self.locals.local_table()
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        self.locals.local_table_mut()
    }
}

#[derive(Debug, Default)]
pub(crate) struct FunctionScopeFrame(LocalTable);

impl Locals for FunctionScopeFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        self.0.local_table()
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        self.0.local_table_mut()
    }
}

#[derive(Debug, Default)]
pub(crate) struct WorkbenchFrame {
    //symbol: mir::SymbolHandle,
    locals: LocalTable,
}

impl Locals for WorkbenchFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        self.locals.local_table()
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        self.locals.local_table_mut()
    }
}

#[derive(Debug, Default)]
pub(crate) struct WorkbenchGroupFrame(LocalTable);

impl Locals for WorkbenchGroupFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        self.0.local_table()
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        self.0.local_table_mut()
    }
}

#[derive(Debug, Default)]
pub(crate) struct SourceFrame {
    //symbol: mir::SymbolHandle,
    locals: LocalTable,
}

impl Locals for SourceFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        self.locals.local_table()
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        self.locals.local_table_mut()
    }
}

#[derive(Debug)]
pub(crate) struct SymbolFrame {
    pub node_id: SymbolNodeId,
}

#[derive(Debug, From)]
pub(crate) enum ResolveStackFrame {
    Source(SourceFrame),
    Function(FunctionFrame),
    FunctionScope(FunctionScopeFrame),
    Workbench(WorkbenchFrame),
    WorkbenchGroup(WorkbenchGroupFrame),
    Symbol(SymbolFrame),
}

impl Locals for ResolveStackFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        match &self {
            ResolveStackFrame::Source(source_file_frame) => source_file_frame.local_table(),
            ResolveStackFrame::Function(function_frame) => function_frame.local_table(),
            ResolveStackFrame::FunctionScope(function_scope) => function_scope.local_table(),
            ResolveStackFrame::Workbench(workbench) => workbench.local_table(),
            ResolveStackFrame::WorkbenchGroup(workbench_group) => workbench_group.local_table(),
            _ => None,
        }
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        match self {
            ResolveStackFrame::Source(source_file_frame) => source_file_frame.local_table_mut(),
            ResolveStackFrame::Function(function_frame) => function_frame.local_table_mut(),
            ResolveStackFrame::FunctionScope(function_scope) => function_scope.local_table_mut(),
            ResolveStackFrame::Workbench(workbench) => workbench.local_table_mut(),
            ResolveStackFrame::WorkbenchGroup(workbench_group) => workbench_group.local_table_mut(),
            _ => None,
        }
    }
}

/// A generic stack.
#[derive(Debug)]
pub struct ResolveStack(Vec<ResolveStackFrame>);

impl ResolveStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, frame: impl Into<ResolveStackFrame>) {
        self.0.push(frame.into());
    }

    pub fn pop(&mut self) -> ResolveStackFrame {
        self.0.pop().expect("A stack frame")
    }

    pub fn top(&self) -> &ResolveStackFrame {
        self.0.last().expect("A stack frame") // Intentionally no error handling here
    }

    pub fn top_mut(&mut self) -> &mut ResolveStackFrame {
        self.0.last_mut().expect("A stack frame")
    }

    /// Traverses the stack from top (innermost) to bottom (outermost) immutably.
    /// Halts early if the closure returns `ControlFlow::Break`.
    pub fn traversal<T>(
        &self,
        mut f: impl FnMut(&ResolveStackFrame) -> std::ops::ControlFlow<T>,
    ) -> Option<T> {
        for frame in self.0.iter().rev() {
            if let std::ops::ControlFlow::Break(value) = f(frame) {
                return Some(value);
            }
        }
        None
    }

    pub fn traversal_mut<T>(
        &mut self,
        mut f: impl FnMut(&mut ResolveStackFrame) -> std::ops::ControlFlow<T>,
    ) -> Option<T> {
        for frame in self.0.iter_mut().rev() {
            if let std::ops::ControlFlow::Break(value) = f(frame) {
                return Some(value);
            }
        }
        None
    }

    pub fn scope<T, Ctx>(
        &mut self,
        ctx: &mut Ctx,
        frame: impl Into<ResolveStackFrame>,
        f: impl FnOnce(&mut Self, &mut Ctx) -> T,
    ) -> T {
        self.push(frame);
        let result = f(self, ctx);
        self.pop();
        result
    }
}

impl Default for ResolveStack {
    fn default() -> Self {
        Self(vec![])
    }
}
