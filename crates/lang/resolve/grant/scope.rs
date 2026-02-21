// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Check wether a statement is legally placed.

use crate::src_ref::{SrcRef, SrcReferrer};
use crate::syntax::*;

#[derive(Clone, PartialEq)]
pub(crate) enum Scope {
    Source,
    Module(SrcRef),
    Workbench(SrcRef),
    Function(SrcRef),
    Init(SrcRef),
    If(SrcRef),
    StatementList(SrcRef),
    Return(SrcRef),
    Assignment(SrcRef, Visibility, Qualifier),
    Body(SrcRef, Option<Box<Scope>>),
    Use(SrcRef, Visibility),
    ExpressionStatement(SrcRef),
    Expression(SrcRef),
}

impl Scope {
    pub(crate) fn to_str(&self) -> &'static str {
        use Scope::*;
        match self {
            Source => "source file",
            Module(..) => "module",
            Workbench(..) => "workbench",
            Function(..) => "function",
            Init(..) => "initializer",
            If(..) => "if statement",
            StatementList(..) => "statement list",
            Return(..) => "return statement",
            Assignment(.., visibility, qualifier) => {
                use {Qualifier::*, Visibility::*};
                match (visibility, qualifier) {
                    (Private, Value) => "value assignment",
                    (Private, Const) => "constant assignment",
                    (Private, Prop) => "property assignment",
                    (Public, Value) => "public assignment",
                    (Public, Const) => "public constant assignment",
                    (Public, Prop) => "public property assignment",
                    _ => unreachable!(),
                }
            }
            Body(..) => "code body",
            Use(..) => "use statement",
            ExpressionStatement(..) => "expression statement",
            Expression(..) => "expression",
        }
    }

    pub(crate) fn allowed_parents(&self) -> &'static [&'static Scope] {
        use Scope::*;
        const SOURCE: &Scope = &Source;
        const MODULE: &Scope = &Module(SrcRef(None));
        const WORKBENCH: &Scope = &Workbench(SrcRef(None));
        const FUNCTION: &Scope = &Function(SrcRef(None));
        const IF: &Scope = &If(SrcRef(None));
        const STMT_LIST: &Scope = &StatementList(SrcRef(None));
        const EXPRESSION: &Scope = &Expression(SrcRef(None));

        match self {
            Source => &[],
            Module(..) | Workbench(..) => &[SOURCE, MODULE],
            Function(..) => &[SOURCE, MODULE, WORKBENCH],
            Init(..) => &[WORKBENCH],
            If(..) => &[SOURCE, WORKBENCH, FUNCTION, IF, EXPRESSION],
            StatementList(..) => &[SOURCE, MODULE, WORKBENCH, FUNCTION],
            Return(..) => &[FUNCTION],
            Assignment(.., visibility, qualifier) => {
                use {Qualifier::*, Visibility::*};
                match (visibility, qualifier) {
                    (Private, Value) => &[SOURCE, MODULE, WORKBENCH, FUNCTION],
                    (Private, Const) => &[SOURCE, MODULE, WORKBENCH],
                    (Private, Prop) => &[WORKBENCH],
                    (Public, Value) => &[],
                    (Public, Const) => &[SOURCE, MODULE],
                    (Public, Prop) => &[],
                    _ => unreachable!(),
                }
            }
            Body(..) => &[SOURCE, MODULE, WORKBENCH, FUNCTION],
            Use(.., visibility) => {
                use Visibility::*;
                match visibility {
                    Private | PrivateUse(..) => &[SOURCE, MODULE, WORKBENCH, FUNCTION],
                    _ => &[SOURCE, MODULE],
                }
            }
            ExpressionStatement(..) => &[SOURCE, WORKBENCH, FUNCTION, IF, STMT_LIST, EXPRESSION],
            Expression(..) => &[SOURCE, WORKBENCH, FUNCTION, IF, STMT_LIST, EXPRESSION],
        }
    }
}

impl SrcReferrer for Scope {
    fn src_ref(&self) -> SrcRef {
        use Scope::*;
        match self {
            Source => SrcRef(None),
            Module(src_ref)
            | Workbench(src_ref)
            | Function(src_ref)
            | Init(src_ref)
            | If(src_ref)
            | StatementList(src_ref)
            | Return(src_ref)
            | Assignment(src_ref, ..)
            | Use(src_ref, ..)
            | ExpressionStatement(src_ref)
            | Expression(src_ref) => src_ref.clone(),
            Body(..) => unreachable!("Body is transparent"),
        }
    }
}
