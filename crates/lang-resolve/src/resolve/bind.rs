// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The `bind` sub-step. This binds all `SymbolPath`s to actual SymbolIds and LocalIds

/*

use derive_more::From;
use microcad_lang_base::{HashMap, Identifier, SrcRef, SrcReferrer};
use microcad_lang_lower::ir;

use crate::{ResolveContext, ResolveResult};

/// A map of locals.
///
/// The `Vec<SrcRef>` represents the usages of this local.
#[derive(Debug, Default)]
pub struct LocalTable(HashMap<Identifier, Vec<SrcRef>>);

pub trait Locals {
    fn local_table(&self) -> Option<&LocalTable>;
    fn local_table_mut(&mut self) -> Option<&mut LocalTable>;

    fn declare_local(&mut self, id: Identifier) {
        if let Some(local_table) = self.local_table_mut() {
            local_table.0.insert(id, vec![]);
        }
    }
    /// Returns true if the local exists.
    fn use_local(&mut self, id: &Identifier) -> bool {
        if let Some(local_table) = self.local_table_mut()
            && let Some(local) = local_table.0.get_mut(id)
        {
            local.push(id.src_ref());
            true
        } else {
            false
        }
    }

    fn unused_locals(&self) -> impl Iterator<Item = &Identifier> {
        self.local_table()
            .unwrap()
            .0
            .iter()
            .filter(|(_, usages)| usages.is_empty())
            .map(|(id, _)| id)
    }
}

impl Locals for LocalTable {
    fn local_table(&self) -> Option<&LocalTable> {
        Some(&self)
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        Some(self)
    }
}

#[derive(Debug, Default)]
struct FunctionFrame {
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
struct FunctionScopeFrame(LocalTable);

impl Locals for FunctionScopeFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        self.0.local_table()
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        self.0.local_table_mut()
    }
}

#[derive(Debug, Default)]
struct WorkbenchFrame {
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
struct WorkbenchGroupFrame(LocalTable);

impl Locals for WorkbenchGroupFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        self.0.local_table()
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        self.0.local_table_mut()
    }
}

#[derive(Debug, Default)]
struct SourceFileFrame {
    //symbol: mir::SymbolHandle,
    locals: LocalTable,
}

impl Locals for SourceFileFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        self.locals.local_table()
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        self.locals.local_table_mut()
    }
}

/// Tells if we are in function or workbench environment
pub enum Environment {
    /// We are currently in a function
    Function,
    /// We are currently in a workbench
    Workbench,
    /// We are currently in an inline module
    InlineModule,
    /// We are currently in a source file
    SourceFile,
}

#[derive(Debug, From)]
pub enum BindStackFrame {
    SourceFile(SourceFileFrame),
    InlineModule(ir::Path),
    Function(FunctionFrame),
    FunctionScope(FunctionScopeFrame),
    Workbench(WorkbenchFrame),
    WorkbenchGroup(WorkbenchGroupFrame),
}

impl BindStackFrame {
    fn environment(&self) -> Environment {
        match &self {
            BindStackFrame::SourceFile(_) => Environment::SourceFile,
            BindStackFrame::InlineModule(_) => Environment::InlineModule,
            BindStackFrame::Function(_) | BindStackFrame::FunctionScope(_) => Environment::Function,
            BindStackFrame::Workbench(_) | BindStackFrame::WorkbenchGroup(_) => {
                Environment::Workbench
            }
        }
    }
}

impl Locals for BindStackFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        match &self {
            BindStackFrame::SourceFile(source_file_frame) => source_file_frame.local_table(),
            BindStackFrame::Function(function_frame) => function_frame.local_table(),
            BindStackFrame::FunctionScope(function_scope) => function_scope.local_table(),
            BindStackFrame::Workbench(workbench) => workbench.local_table(),
            BindStackFrame::WorkbenchGroup(workbench_group) => workbench_group.local_table(),
            _ => None,
        }
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        match self {
            BindStackFrame::SourceFile(source_file_frame) => source_file_frame.local_table_mut(),
            BindStackFrame::Function(function_frame) => function_frame.local_table_mut(),
            BindStackFrame::FunctionScope(function_scope) => function_scope.local_table_mut(),
            BindStackFrame::Workbench(workbench) => workbench.local_table_mut(),
            BindStackFrame::WorkbenchGroup(workbench_group) => workbench_group.local_table_mut(),
            _ => None,
        }
    }
}

/// A generic stack.
pub struct Stack<Frame>(Vec<Frame>);

impl<Frame> Stack<Frame> {
    pub fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, frame: impl Into<Frame>) {
        self.0.push(frame.into());
    }

    fn pop(&mut self) -> Frame {
        self.0.pop().expect("A stack frame")
    }

    fn top(&self) -> &Frame {
        self.0.last().expect("A stack frame") // Intentionally no error handling here
    }

    pub fn top_mut(&mut self) -> &mut Frame {
        self.0.last_mut().expect("A stack frame")
    }

    /// Traverses the stack from top (innermost) to bottom (outermost) immutably.
    /// Halts early if the closure returns `ControlFlow::Break`.
    pub fn traversal<T>(&self, mut f: impl FnMut(&Frame) -> std::ops::ControlFlow<T>) -> Option<T> {
        for frame in self.0.iter().rev() {
            if let std::ops::ControlFlow::Break(value) = f(frame) {
                return Some(value);
            }
        }
        None
    }

    pub fn traversal_mut<T>(
        &mut self,
        mut f: impl FnMut(&mut Frame) -> std::ops::ControlFlow<T>,
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
        frame: impl Into<Frame>,
        f: impl FnOnce(&mut Stack<Frame>, &mut Ctx) -> T,
    ) -> T {
        self.push(frame);
        let result = f(self, ctx);
        self.pop();
        result
    }
}

impl<Frame> Default for Stack<Frame> {
    fn default() -> Self {
        Self(vec![])
    }
}

pub struct Binder<'ctx> {
    ctx: &'ctx mut ResolveContext,
    stack: Stack<BindStackFrame>,
}

impl<'ctx> Locals for Binder<'ctx> {
    fn local_table(&self) -> Option<&LocalTable> {
        self.stack.top().local_table()
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        self.stack.top_mut().local_table_mut()
    }

    fn declare_local(&mut self, id: Identifier) {
        match self.stack.top_mut().local_table_mut() {
            Some(locals) => locals.declare_local(id),
            None => {}
        }
    }

    fn use_local(&mut self, id: &Identifier) -> bool {
        let found = self.stack.traversal_mut(|frame| {
            if let Some(locals) = frame.local_table_mut() {
                if locals.use_local(id) {
                    // We found and recorded the usage, stop traversing!
                    return std::ops::ControlFlow::Break(());
                }
            }
            // Not in this frame, keep looking downwards
            std::ops::ControlFlow::Continue(())
        });

        // If traversal_mut returned Some(()), it means we broke early (found it).
        found.is_some()
    }

    fn unused_locals(&self) -> impl Iterator<Item = &Identifier> {
        self.stack
            .0
            .iter()
            .rev()
            .flat_map(|frame| frame.unused_locals())
    }
}

impl<'ctx> Binder<'ctx> {
    pub fn new(ctx: &'ctx mut ResolveContext) -> Self {
        Self {
            ctx,
            stack: Default::default(),
        }
    }

    pub fn scope<T>(
        &mut self,
        frame: impl Into<BindStackFrame>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        // 1. Temporarily swap out the stack to avoid self-borrow issues
        let mut stack = std::mem::take(&mut self.stack);
        stack.push(frame);
        self.stack = stack;

        // 2. Define a guard that pops the frame on Drop
        struct PopGuard<'a, 'ctx>(&'a mut Binder<'ctx>);

        impl<'a, 'ctx> Drop for PopGuard<'a, 'ctx> {
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
}

trait Bind<'ctx> {
    fn bind(&mut self, binder: &mut Binder<'ctx>) -> ResolveResult<()>;
}

impl<'ctx> Bind<'ctx> for mir::Path {
    fn bind(&mut self, binder: &mut Binder<'ctx>) -> ResolveResult<()> {
        match self {
            microcad_package::symbol::Path::Unresolved(unresolved_path) => {
                todo!()
                // *self = binder.bind(unresolved_path)?;
                //Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl<'ctx> Bind<'ctx> for mir::ConstantExpression {
    fn bind(&mut self, binder: &mut Binder<'ctx>) -> ResolveResult<()> {
        use mir::ConstantExpression;

        Ok(match self {
            ConstantExpression::Invalid | ConstantExpression::Literal(_) => {}
            ConstantExpression::Path(name) => name.bind(binder)?,
            ConstantExpression::Call(call) => todo!(),
        })
    }
}

impl<'ctx> Bind<'ctx> for mir::FunctionExpression {
    fn bind(&mut self, binder: &mut Binder<'ctx>) -> ResolveResult<()> {
        use mir::FunctionExpression;

        Ok(match self {
            FunctionExpression::Invalid => {}
            FunctionExpression::Literal(literal) => {}
            FunctionExpression::Path(name) => name.bind(binder)?,
            FunctionExpression::Scope(scope) => todo!(),
            FunctionExpression::If(_) => todo!(),
            FunctionExpression::Call(call) => todo!(),
            _ => todo!(),
        })
    }
}

impl<'ctx> Bind<'ctx> for mir::Function {
    fn bind(&mut self, binder: &mut Binder<'ctx>) -> ResolveResult<()> {
        binder.scope(FunctionFrame::default(), |binder| {
            use mir::FunctionStatement;
            for statement in &mut self.statements {
                match statement {
                    FunctionStatement::Local(local_assignment) => {
                        local_assignment.expression.bind(binder)?;

                        // Add local *after* we have bound the expression.
                        binder.declare_local(local_assignment.id.clone());
                    }
                    FunctionStatement::Return(return_statement) => match return_statement.expr {
                        Some(ref mut value) => {
                            value.bind(binder)?;
                            return Ok(());
                        }
                        None => return Ok(()),
                    },
                    _ => todo!(),
                }
            }

            Ok(())
        })
    }
}

*/
