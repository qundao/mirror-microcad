// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Check wether a statement is legally placed.

use microcad_lang_base::{SrcRef, SrcReferrer};

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum ScopeType {
    Source,
    Module,
    Workbench,
    Function,
    Init,
    If,
    StatementList,
    Return,
    ValueAssignment,
    ConstAssignment,
    PubAssignment,
    PropAssignment,
    Body,
    Use,
    PubUse,
    ExpressionStatement,
    Expression,
}

impl ScopeType {
    pub(crate) fn to_str(self) -> &'static str {
        use ScopeType::*;
        match self {
            Source => "source file",
            Module => "module",
            Workbench => "workbench",
            Function => "function",
            Init => "initializer",
            If => "if statement",
            StatementList => "statement list",
            Return => "return statement",
            ValueAssignment => "value assignment",
            ConstAssignment => "constant assignment",
            PubAssignment => "public assignment",
            PropAssignment => "property assignment",
            Body => "code body",
            Use => "use statement",
            PubUse => "public use statement",
            ExpressionStatement => "expression statement",
            Expression => "expression",
        }
    }
    pub(crate) fn allowed_parents(&self) -> &'static [ScopeType] {
        use ScopeType::*;
        match self {
            Source => &[],
            Module | Workbench => &[Source, Module],
            Function => &[Source, Module, Workbench],
            Init => &[Workbench],
            If => &[Source, Workbench, Function, If, Expression],
            StatementList => &[Source, Module, Workbench, Function],
            Return => &[Function],
            ValueAssignment => &[Source, Module, Workbench, Function],
            ConstAssignment => &[Source, Module, Workbench],
            PropAssignment => &[Workbench],
            PubAssignment => &[Source, Module],
            Body => &[Source, Module, Workbench, Function],
            Use => &[Source, Module, Workbench, Function],
            PubUse => &[Source, Module],
            ExpressionStatement => &[Source, Workbench, Function, If, StatementList, Expression],
            Expression => &[Source, Workbench, Function, If, StatementList, Expression],
        }
    }
}

#[derive(Clone)]
pub(crate) struct Scope(pub ScopeType, pub SrcRef);

impl SrcReferrer for Scope {
    fn src_ref(&self) -> SrcRef {
        self.1.clone()
    }
}

impl Scope {
    pub(crate) fn to_str(&self) -> &'static str {
        self.0.to_str()
    }

    pub(crate) fn ty(&self) -> ScopeType {
        self.0
    }
}
