// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{src_ref::*, syntax::*};
use custom_debug::Debug;
use strum::Display;

/// Kind of a [`WorkbenchDefinition`].
#[derive(Clone, Display, Debug, Copy, PartialEq)]
pub enum WorkbenchKind {
    /// 3D part
    Part,
    /// 2D sketch
    Sketch,
    /// Operation
    Operation,
}

impl WorkbenchKind {
    /// return kind name
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkbenchKind::Part => "part",
            WorkbenchKind::Sketch => "sketch",
            WorkbenchKind::Operation => "op",
        }
    }
}

/// Workbench definition, e.g `sketch`, `part` or `op`.
#[derive(Clone)]
pub struct WorkbenchDefinition {
    /// Documentation.
    pub doc: Option<DocBlock>,
    /// Workbench attributes.
    pub attribute_list: AttributeList,
    /// Visibility from outside modules.
    pub visibility: Visibility,
    /// Workbench kind.
    pub kind: Refer<WorkbenchKind>,
    /// Workbench name.
    pub id: Identifier,
    /// Workbench's building plan.
    pub plan: ParameterList,
    /// Workbench body
    pub body: Body,
}

impl WorkbenchDefinition {
    pub(crate) fn possible_params(&self) -> Vec<String> {
        std::iter::once(&self.plan)
            .chain(self.inits().map(|init| &init.parameters))
            .map(|params| format!("{}( {})", self.id, params))
            .collect()
    }
}

impl<'a> Initialized<'a> for WorkbenchDefinition {
    fn statements(&'a self) -> std::slice::Iter<'a, Statement> {
        self.body.statements.iter()
    }
}

impl SrcReferrer for WorkbenchDefinition {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl std::fmt::Display for WorkbenchDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{visibility}{kind} {id}({plan}) {body}",
            visibility = self.visibility,
            kind = self.kind,
            id = self.id,
            plan = self.plan,
            body = self.body
        )
    }
}

impl std::fmt::Debug for WorkbenchDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{visibility}{kind} {id:?}({plan:?}) {body:?}",
            visibility = self.visibility,
            kind = self.kind,
            id = self.id,
            plan = self.plan,
            body = self.body
        )
    }
}

impl TreeDisplay for WorkbenchDefinition {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}{visibility}Workbench ({kind}) '{id}':",
            "",
            visibility = self.visibility,
            kind = self.kind,
            id = self.id
        )?;
        depth.indent();
        if let Some(doc) = &self.doc {
            doc.tree_print(f, depth)?;
        }
        self.plan.tree_print(f, depth)?;
        self.body.tree_print(f, depth)
    }
}

impl Doc for WorkbenchDefinition {
    fn doc(&self) -> Option<DocBlock> {
        self.doc.clone()
    }
}
