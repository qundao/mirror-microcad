// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Check wether a statement is legally placed.

use crate::{
    src_ref::{SrcRef, SrcReferrer},
    syntax::WorkbenchKind,
};

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum ScopeType {
    Source,
    Module,
    Sketch,
    Part,
    Op,
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
            Sketch => "sketch",
            Part => "part",
            Op => "op",
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
            Module | Sketch | Part | Op => &[Source, Module],
            Function => &[Source, Module, Sketch, Part, Op],
            Init => &[Sketch, Part, Op],
            If => &[Source, Sketch, Part, Op, Function, If, Expression],
            StatementList => &[Source, Module, Sketch, Part, Op, Function],
            Return => &[Function],
            ValueAssignment => &[Source, Module, Sketch, Part, Op, Function],
            ConstAssignment => &[Source, Module, Sketch, Part, Op],
            PropAssignment => &[Sketch, Part, Op],
            PubAssignment => &[Source, Module],
            Body => &[Source, Module, Sketch, Part, Op, Function],
            Use => &[Source, Module, Sketch, Part, Op, Function],
            PubUse => &[Source, Module],
            ExpressionStatement => &[
                Source,
                Sketch,
                Part,
                Op,
                Function,
                If,
                StatementList,
                Expression,
            ],
            Expression => &[
                Source,
                Sketch,
                Part,
                Op,
                Function,
                If,
                StatementList,
                Expression,
            ],
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

impl From<WorkbenchKind> for ScopeType {
    fn from(kind: WorkbenchKind) -> Self {
        match kind {
            WorkbenchKind::Part => ScopeType::Part,
            WorkbenchKind::Sketch => ScopeType::Sketch,
            WorkbenchKind::Operation => ScopeType::Op,
        }
    }
}
