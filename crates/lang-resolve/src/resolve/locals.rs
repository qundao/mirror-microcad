// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve locals.

use crate::library::{SymbolNodeId, symbol};
use derive_more::From;
use microcad_lang_base::{
    Identifier, SingleIdentifier, SrcRef, SrcReferrer, SymbolId, ToCompactString,
};
use microcad_lang_lower::ir::visitor::{
    ConstantVisitorMut, FnVisitorMut, LeafVisitorMut, SourceVisitorMut, VisitorMut,
    WorkbenchExpressionVisitorMut, WorkbenchVisitorMut,
};

/// A map of locals.
///
/// The `Vec<SrcRef>` represents the usages of this local.
#[derive(Debug, Default)]
pub struct LocalTable(microcad_lang_base::HashMap<Identifier, Vec<SrcRef>>);

impl LocalTable {
    fn exists(&self, id: &Identifier) -> bool {
        self.0.get(id).is_some()
    }
}

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
struct SourceFrame {
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

#[derive(Debug, From)]
enum LocalsStackFrame {
    SourceFile(SourceFrame),
    Function(FunctionFrame),
    FunctionScope(FunctionScopeFrame),
    Workbench(WorkbenchFrame),
    WorkbenchGroup(WorkbenchGroupFrame),
    Symbol(SymbolNodeId),
}

impl Locals for LocalsStackFrame {
    fn local_table(&self) -> Option<&LocalTable> {
        match &self {
            LocalsStackFrame::SourceFile(source_file_frame) => source_file_frame.local_table(),
            LocalsStackFrame::Function(function_frame) => function_frame.local_table(),
            LocalsStackFrame::FunctionScope(function_scope) => function_scope.local_table(),
            LocalsStackFrame::Workbench(workbench) => workbench.local_table(),
            LocalsStackFrame::WorkbenchGroup(workbench_group) => workbench_group.local_table(),
            _ => None,
        }
    }

    fn local_table_mut(&mut self) -> Option<&mut LocalTable> {
        match self {
            LocalsStackFrame::SourceFile(source_file_frame) => source_file_frame.local_table_mut(),
            LocalsStackFrame::Function(function_frame) => function_frame.local_table_mut(),
            LocalsStackFrame::FunctionScope(function_scope) => function_scope.local_table_mut(),
            LocalsStackFrame::Workbench(workbench) => workbench.local_table_mut(),
            LocalsStackFrame::WorkbenchGroup(workbench_group) => workbench_group.local_table_mut(),
            _ => None,
        }
    }
}

/// A generic stack.
pub struct LocalsStack(Vec<LocalsStackFrame>);

impl LocalsStack {
    pub fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, frame: impl Into<LocalsStackFrame>) {
        self.0.push(frame.into());
    }

    fn pop(&mut self) -> LocalsStackFrame {
        self.0.pop().expect("A stack frame")
    }

    fn top(&self) -> &LocalsStackFrame {
        self.0.last().expect("A stack frame") // Intentionally no error handling here
    }

    pub fn top_mut(&mut self) -> &mut LocalsStackFrame {
        self.0.last_mut().expect("A stack frame")
    }

    /// Traverses the stack from top (innermost) to bottom (outermost) immutably.
    /// Halts early if the closure returns `ControlFlow::Break`.
    pub fn traversal<T>(
        &self,
        mut f: impl FnMut(&LocalsStackFrame) -> std::ops::ControlFlow<T>,
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
        mut f: impl FnMut(&mut LocalsStackFrame) -> std::ops::ControlFlow<T>,
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
        frame: impl Into<LocalsStackFrame>,
        f: impl FnOnce(&mut Self, &mut Ctx) -> T,
    ) -> T {
        self.push(frame);
        let result = f(self, ctx);
        self.pop();
        result
    }
}

impl Default for LocalsStack {
    fn default() -> Self {
        Self(vec![])
    }
}

pub struct LocalsVisitor {
    stack: LocalsStack,
}

impl LocalsVisitor {
    pub fn scope<T>(
        &mut self,
        frame: impl Into<LocalsStackFrame>,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        // 1. Temporarily swap out the stack to avoid self-borrow issues
        let mut stack = std::mem::take(&mut self.stack);
        stack.push(frame);
        self.stack = stack;

        // 2. Define a guard that pops the frame on Drop
        struct PopGuard<'a>(&'a mut LocalsVisitor);

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
}

impl Locals for LocalsVisitor {
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

impl VisitorMut for LocalsVisitor {}

impl LeafVisitorMut for LocalsVisitor {
    fn visit_path(&mut self, path: &mut microcad_lang_lower::ir::Path) {
        if let symbol::Path::Unresolved(unresolved_path) = path {
            if let Some(id) = unresolved_path.single_identifier() {
                if self.local_table().unwrap().exists(id) {
                    *path = symbol::Path::Resolved(SymbolId::Local(id.to_compact_string()));
                } else {
                    todo!("Error handling: Local not in scope {id}")
                }
            }
        }
    }
}

impl ConstantVisitorMut for LocalsVisitor {}
impl WorkbenchVisitorMut for LocalsVisitor {
    fn visit_workbench(&mut self, workbench: &mut microcad_lang_lower::ir::workbench::Workbench) {
        self.visit_workbench_signature(&mut workbench.signature);

        workbench
            .statements
            .iter_mut()
            .for_each(|stmt| self.visit_workbench_statement(stmt))
    }

    fn visit_workbench_signature(
        &mut self,
        signature: &mut microcad_lang_lower::ir::workbench::WorkbenchSignature,
    ) {
        self.visit_parameter_list(&mut signature.parameters);
        signature
            .parameters
            .iter()
            .for_each(|param| self.declare_local(param.id.clone()));

        signature
            .inits
            .iter_mut()
            .for_each(|init| self.visit_workbench_init(init));
    }

    fn visit_workbench_init(&mut self, init: &mut microcad_lang_lower::ir::workbench::Init) {
        self.visit_workbench_init_attr(&mut init.attr);
        self.visit_parameter_list(&mut init.parameters);
        init.parameters
            .iter()
            .for_each(|param| self.declare_local(param.id.clone()));

        init.statements.iter_mut().for_each(|init_statement| {
            self.visit_name(&mut init_statement.name);
            self.visit_workbench_expr(&mut init_statement.expression);
        });
    }

    fn visit_workbench_init_attr(
        &mut self,
        _attr: &mut microcad_lang_lower::ir::workbench::InitAttributes,
    ) {
    }
}
impl WorkbenchExpressionVisitorMut for LocalsVisitor {
    fn visit_workbench_statement(
        &mut self,
        statement: &mut microcad_lang_lower::ir::workbench::WorkbenchStatement,
    ) {
        self.visit_model_attributes(&mut statement.attr);
        if let Some(name) = &mut statement.name {
            self.visit_name(name);
        }
        self.visit_workbench_expr(&mut statement.expression);
        if let Some(name) = &mut statement.name {
            self.declare_local(name.clone());
        }
    }

    fn visit_workbench_group(&mut self, group: &mut microcad_lang_lower::ir::workbench::Group) {
        self.visit_model_attributes(&mut group.attr);
        self.scope(WorkbenchGroupFrame::default(), |visitor| {
            group
                .statements
                .iter_mut()
                .for_each(|statement| visitor.visit_workbench_statement(statement));
        })
    }
}

impl SourceVisitorMut for LocalsVisitor {
    fn visit_source(&mut self, source: &mut microcad_lang_lower::ir::Source) {
        self.scope(SourceFrame::default(), |visitor| {
            source
                .statements
                .iter_mut()
                .for_each(|stmt| visitor.visit_source_statement(stmt));
            // TODO Output unused locals
        })
    }

    fn visit_source_statement(&mut self, statement: &mut symbol::SourceStatement) {
        if let Some(name) = &mut statement.name {
            self.visit_name(name);
        }
        self.visit_model_attributes(&mut statement.attr);
        self.visit_workbench_expr(&mut statement.expression);
        if let Some(name) = &mut statement.name {
            self.declare_local(name.clone());
        }
    }
}

impl FnVisitorMut for LocalsVisitor {
    fn visit_fn(&mut self, function: &mut symbol::Function) {
        self.scope(FunctionFrame::default(), |visitor| {
            visitor.visit_fn_signature(&mut function.signature);
            function
                .statements
                .iter_mut()
                .for_each(|stmt| visitor.visit_fn_statement(stmt));

            // TODO: Output unused locals here.
        })
    }

    fn visit_fn_signature(
        &mut self,
        signature: &mut microcad_lang_lower::ir::function::FunctionSignature,
    ) {
        self.visit_parameter_list(&mut signature.parameters);
        signature
            .parameters
            .iter()
            .for_each(|param| self.declare_local(param.id.clone()));
    }

    fn visit_fn_scope(&mut self, scope: &mut microcad_lang_lower::ir::function::Scope) {
        self.scope(FunctionScopeFrame::default(), |ctx| {
            scope
                .statements
                .iter_mut()
                .for_each(|stmt| ctx.visit_fn_statement(stmt))

            // TODO: Output unused locals here.
        })
    }

    fn visit_fn_local_assignment(
        &mut self,
        local_assignment: &mut microcad_lang_lower::ir::function::FunctionLocalAssignment,
    ) {
        self.visit_name(&mut local_assignment.id);
        self.visit_fn_expr(&mut local_assignment.expression);
        self.declare_local(local_assignment.id.clone());
    }
}
