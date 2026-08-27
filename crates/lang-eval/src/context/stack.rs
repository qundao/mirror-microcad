// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::From;
use microcad_lang_base::{HashMap, Name, ToCompactString};
use microcad_lang_types::{
    Arguments, Value,
    model::{Element, GetProperty, ModelTreeBuilder, ModelTreeBuilderMut, Properties, Workpiece},
};

pub trait Lookup {
    /// Local a local or property value by traversing up the stack
    fn look_up_local(&self, name: impl AsRef<str>) -> Option<&Value>;
}

/// A map of locals.
#[derive(Debug, Default)]
pub struct LocalTable(HashMap<Name, Value>);

impl Lookup for LocalTable {
    fn look_up_local(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.0.get(&Name::from(name.as_ref()))
    }
}

#[derive(Debug, Default)]
pub struct FunctionFrame {
    pub locals: LocalTable,
}

impl Lookup for FunctionFrame {
    fn look_up_local(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.locals.look_up_local(name)
    }
}

impl FunctionFrame {
    pub fn new(args: Arguments) -> Self {
        let locals = LocalTable(
            args.named_iter()
                .map(|(id, value)| (id.to_compact_string(), value.clone()))
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

impl Lookup for FunctionScopeFrame {
    fn look_up_local(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.locals.look_up_local(name)
    }
}

#[derive(Debug)]
pub struct WorkbenchFrame {
    pub builder: ModelTreeBuilder,
}

impl WorkbenchFrame {
    pub fn new(workpiece: Workpiece) -> Self {
        Self {
            builder: ModelTreeBuilder::new(Element::Workpiece(workpiece)),
        }
    }
}

impl ModelTreeBuilderMut for WorkbenchFrame {
    fn model_tree_builder_mut(&mut self) -> &mut ModelTreeBuilder {
        &mut self.builder
    }
}

impl Lookup for WorkbenchFrame {
    fn look_up_local(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.builder.get_property(name).map(|prop| &prop.value)
    }
}

#[derive(Debug)]
pub struct WorkbenchGroupFrame {
    pub builder: ModelTreeBuilder,
}

impl WorkbenchGroupFrame {
    pub fn new() -> Self {
        Self {
            builder: ModelTreeBuilder::new(Element::Group),
        }
    }
}

impl Lookup for WorkbenchGroupFrame {
    fn look_up_local(&self, name: impl AsRef<str>) -> Option<&Value> {
        self.builder.get_property(name).map(|prop| &prop.value)
    }
}

impl ModelTreeBuilderMut for WorkbenchGroupFrame {
    fn model_tree_builder_mut(&mut self) -> &mut ModelTreeBuilder {
        &mut self.builder
    }
}

#[derive(Debug, Default)]
pub struct WorkbenchInitFrame {
    pub properties: Properties,
}

#[derive(Debug)]
pub struct CallFrame {
    pub path: microcad_package::symbol::Path,
}

#[derive(Debug, From)]
pub enum StackFrame {
    Call(CallFrame),
    Function(FunctionFrame),
    FunctionScope(FunctionScopeFrame),
    Workbench(WorkbenchFrame),
    WorkbenchGroup(WorkbenchGroupFrame),
    WorkbenchInit(WorkbenchInitFrame),
}

impl StackFrame {
    pub fn get_local(&self, name: &Name) -> Option<&Value> {
        match self {
            StackFrame::Function(FunctionFrame { locals })
            | StackFrame::FunctionScope(FunctionScopeFrame { locals }) => locals.0.get(name),
            _ => None,
        }
    }

    pub fn put_local(&mut self, name: Name, value: Value) {
        match self {
            StackFrame::Function(FunctionFrame { locals })
            | StackFrame::FunctionScope(FunctionScopeFrame { locals }) => {
                locals.0.insert(name, value);
            }
            _ => {}
        }
    }
}

impl ModelTreeBuilderMut for StackFrame {
    fn model_tree_builder_mut(&mut self) -> &mut ModelTreeBuilder {
        match self {
            StackFrame::Workbench(workbench_frame) => workbench_frame.model_tree_builder_mut(),
            StackFrame::WorkbenchGroup(workbench_group_frame) => {
                workbench_group_frame.model_tree_builder_mut()
            }
            _ => panic!("No model tree builder"),
        }
    }
}

impl Lookup for StackFrame {
    fn look_up_local(&self, name: impl AsRef<str>) -> Option<&Value> {
        match &self {
            StackFrame::Call(_) => None,
            StackFrame::Function(function_frame) => function_frame.look_up_local(name),
            StackFrame::FunctionScope(function_scope_frame) => {
                function_scope_frame.look_up_local(name)
            }
            StackFrame::Workbench(workbench_frame) => workbench_frame.look_up_local(name),
            StackFrame::WorkbenchGroup(workbench_group_frame) => {
                workbench_group_frame.look_up_local(name)
            }
            StackFrame::WorkbenchInit(_) => None,
        }
    }
}

/// A generic stack.
#[derive(Debug, Default)]
pub struct Stack(Vec<StackFrame>);

impl Stack {
    pub fn current_call_scope(&self) -> impl Iterator<Item = &StackFrame> {
        self.0.iter().rev().scan(false, |hit_call, frame| {
            if *hit_call {
                return None;
            }
            if matches!(frame, StackFrame::Call(_)) {
                *hit_call = true;
            }
            Some(frame)
        })
    }
}

impl ModelTreeBuilderMut for Stack {
    fn model_tree_builder_mut(&mut self) -> &mut ModelTreeBuilder {
        self.top_mut().model_tree_builder_mut()
    }
}

impl Lookup for Stack {
    fn look_up_local(&self, name: impl AsRef<str>) -> Option<&Value> {
        let name = name.as_ref();
        self.current_call_scope()
            .find_map(|frame| frame.look_up_local(name))
    }
}

impl StackRead for Stack {
    type Frame = StackFrame;

    fn get_local(&self, name: &Name) -> Option<&Value> {
        self.current_call_scope()
            .find_map(|frame| frame.get_local(name))
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

pub trait StackRead {
    type Frame;

    fn get_local(&self, _name: &Name) -> Option<&Value> {
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
