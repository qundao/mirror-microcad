// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::From;
use microcad_lang_base::{HashMap, Name, ToCompactString};
use microcad_lang_types::{
    Arguments, Model, ModelTree, Value,
    model::{Element, Properties},
};

/// A map of locals.
#[derive(Debug, Default)]
pub struct LocalTable(HashMap<Name, Value>);

#[derive(Debug, Default)]
pub struct FunctionFrame {
    pub locals: LocalTable,
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

#[derive(Debug, Default)]
pub struct WorkbenchFrame {
    pub properties: Properties,
    pub children: Vec<ModelTree>,
}

#[derive(Debug, Default)]
pub struct WorkbenchGroupFrame {
    pub model: Model,
    pub properties: Properties,
    pub children: Vec<ModelTree>,
}

impl WorkbenchGroupFrame {
    pub fn new() -> Self {
        Self {
            model: Model::from(Element::Group),
            properties: todo!(),
            children: todo!(),
        }
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

/// A generic stack.
#[derive(Debug, Default)]
pub struct Stack(Vec<StackFrame>);

impl Stack {}

impl StackRead for Stack {
    type Frame = StackFrame;

    fn get_local(&self, name: &Name) -> Option<&Value> {
        self.0.iter().rev().find_map(|frame| frame.get_local(name))
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
