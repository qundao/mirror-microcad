// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::rc::*;

use crate::syntax::*;

pub trait Info {
    fn info(&self) -> SymbolInfo;
}

pub struct ParameterInfo {
    id: String,
    ty: String,
    def: Option<String>,
}

pub struct CallInfo {
    id: String,
    params: Vec<ParameterInfo>,
    doc: DocBlock,
}

pub struct SymbolInfo {
    /// Human readable symbol kind.
    kind: String,
    /// Optional documentation block.
    doc: Option<DocBlock>,
    /// Parameters and alternative parameters if any.
    signatures: Vec<CallInfo>,
}

impl SymbolInfo {
    pub(super) fn from_const_expression(
        visibility: Visibility,
        id: Identifier,
        expression: &Rc<Expression>,
    ) -> SymbolInfo {
    }
}

impl From<&Rc<SourceFile>> for SymbolInfo {
    fn from(value: &Rc<SourceFile>) -> Self {
        todo!()
    }
}

impl From<&Rc<ModuleDefinition>> for SymbolInfo {
    fn from(value: &Rc<ModuleDefinition>) -> Self {
        todo!()
    }
}

impl From<&Rc<WorkbenchDefinition>> for SymbolInfo {
    fn from(value: &Rc<WorkbenchDefinition>) -> Self {
        todo!()
    }
}

impl From<&Rc<FunctionDefinition>> for SymbolInfo {
    fn from(value: &Rc<FunctionDefinition>) -> Self {
        todo!()
    }
}

impl From<&Rc<Builtin>> for SymbolInfo {
    fn from(value: &Rc<Builtin>) -> Self {
        todo!()
    }
}
