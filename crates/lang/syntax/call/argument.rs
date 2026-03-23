// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A single argument

use microcad_lang_base::{SrcRef, TreeDisplay, TreeState};
use microcad_lang_proc_macros::SrcReferrer;

use crate::{ord_map::*, syntax::*};

/// Argument in a [`Call`].
#[derive(Clone, Debug, PartialEq, SrcReferrer)]
pub struct Argument {
    /// Name of the argument
    pub id: Option<Identifier>,
    /// Value of the argument
    pub expression: Expression,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl OrdMapValue<Identifier> for Argument {
    fn key(&self) -> Option<Identifier> {
        self.id.clone()
    }
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.id {
            Some(ref id) => write!(f, "{id} = {}", self.expression),
            None => write!(f, "{}", self.expression),
        }
    }
}

impl TreeDisplay for Argument {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        match self.id {
            Some(ref id) => writeln!(f, "{:depth$}Argument '{id:?}':", "")?,
            None => writeln!(f, "{:depth$}Argument:", "")?,
        };
        depth.indent();
        self.expression.tree_print(f, depth)
    }
}

#[test]
fn test_argument_debug() {
    let arg1 = Argument {
        id: Some("id1".into()),
        expression: Expression::QualifiedName("my::name1".into()),
        src_ref: SrcRef(None),
    };

    let arg2 = Argument {
        id: None,
        expression: Expression::QualifiedName("my::name2".into()),
        src_ref: SrcRef(None),
    };

    let arg3 = Argument {
        id: Some(Identifier::none()),
        expression: Expression::QualifiedName("my::name2".into()),
        src_ref: SrcRef(None),
    };

    let mut args = ArgumentList::default();

    args.try_push(arg1).expect("test error");
    args.try_push(arg2).expect("test error");
    args.try_push(arg3).expect("test error");

    log::info!("{args}");
    log::info!("{args:?}");
}
