// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use crate::{src_ref::*, syntax::*, ty::*};

/// Function definition
#[derive(Clone)]
pub struct FunctionDefinition {
    /// SrcRef of the `fn` keyword
    pub keyword_ref: SrcRef,
    /// Documentation.
    pub doc: Option<DocBlock>,
    /// Visibility
    pub visibility: Visibility,
    /// Name of the function
    pub(crate) id: Identifier,
    /// Function signature
    pub signature: FunctionSignature,
    /// Function body
    pub body: Body,
}

impl FunctionDefinition {
    pub(crate) fn return_type(&self) -> Type {
        self.signature
            .return_type
            .as_ref()
            .map(|ty| ty.ty())
            .unwrap_or(Type::Invalid)
    }
}

impl Identifiable for FunctionDefinition {
    fn id_ref(&self) -> &Identifier {
        &self.id
    }
}

impl SrcReferrer for FunctionDefinition {
    fn src_ref(&self) -> SrcRef {
        self.id.src_ref()
    }
}

impl TreeDisplay for FunctionDefinition {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}FunctionDefinition '{}':", "", self.id)?;
        depth.indent();
        if let Some(doc) = &self.doc {
            doc.tree_print(f, depth)?;
        }
        writeln!(f, "{:depth$}Signature:", "")?;
        self.signature.tree_print(f, depth)?;
        writeln!(f, "{:depth$}Body:", "")?;
        self.body.tree_print(f, depth)
    }
}

impl std::fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {}{}", self.id, self.signature)
    }
}

impl std::fmt::Debug for FunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {:?}{:?}", self.id, self.signature)
    }
}

impl Doc for FunctionDefinition {
    fn doc(&self) -> Option<DocBlock> {
        self.doc.clone()
    }
}
