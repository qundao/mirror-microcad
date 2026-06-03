// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lowering compile step

pub mod ir;

#[allow(clippy::module_inception)]
mod lower;

use microcad_lang_base::{
    ComputedHash, DiagResult, Diagnostic, Diagnostics, Hashed, Identifier, LineIndex, PushDiag,
    Refer, Span, SrcRef, SrcReferrer,
};

pub use lower::{LowerError, LowerErrorsWithSource, LowerResult};

/// Check if the element only includes one identifier
pub trait SingleIdentifier {
    /// If the element only includes one identifier, return it
    fn single_identifier(&self) -> Option<&Identifier>;

    /// Returns true if the element only includes a single identifier.
    fn is_single_identifier(&self) -> bool {
        self.single_identifier().is_some()
    }
}

/// Identifier accessor.
pub trait Identifiable {
    /// Get clone of the identifier.
    fn id(&self) -> Identifier {
        self.id_ref().clone()
    }

    /// Get reference to the identifier.
    fn id_ref(&self) -> &Identifier;

    /// Get identifier as string.
    fn id_as_str(&self) -> &str {
        self.id_ref().0.as_str()
    }
}

/// Interface for elements which have *initializers*.
pub trait Initialized<'a> {
    /// return iterator of body statements.
    fn statements(&'a self) -> std::slice::Iter<'a, ir::Statement>;

    /// Return iterator over all initializers.
    fn inits(&'a self) -> ir::Inits<'a>
    where
        Self: std::marker::Sized,
    {
        ir::Inits::new(self)
    }
}

pub struct LowerContext<'source> {
    pub source: Hashed<&'source str>,
    line_index: LineIndex,
    line_offset: u32,
    diagnostics: Diagnostics,
}

impl<'source> LowerContext<'source> {
    pub fn new(source: &'source str) -> Self {
        LowerContext {
            source: Hashed::new(source),
            line_index: LineIndex::new(source),
            line_offset: 0,
            diagnostics: Diagnostics::default(),
        }
    }

    pub fn with_line_offset(self, line_offset: u32) -> Self {
        Self {
            source: self.source,
            line_index: self.line_index,
            line_offset,
            diagnostics: Diagnostics::default(),
        }
    }

    pub fn src_ref(&self, span: &Span) -> SrcRef {
        self.line_index
            .src_ref(self.source.value(), span, self.source.computed_hash())
            .with_line_offset(self.line_offset)
    }

    // Use `impl PushDiag` here
    pub fn warning(&mut self, diagnostic: LowerError) -> DiagResult<()> {
        let src_ref = diagnostic.src_ref();
        self.diagnostics
            .push_diag(Diagnostic::Warning(std::rc::Rc::new(Refer::new(
                diagnostic.into(),
                src_ref,
            ))))
    }
}

pub trait Lower: Sized {
    type AstNode;

    fn lower(node: &Self::AstNode, context: &mut LowerContext) -> Result<Self, LowerError>;
}
