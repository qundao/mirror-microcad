// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lowering compile step

pub mod ir;

mod lower;

use microcad_lang_base::{
    Artifact, ArtifactKind, CompilationResult, Diagnostics, Identifier, LineIndex, Source, Span,
    SpanToSrcRef, SrcRef,
};

pub use lower::{LowerError, LowerResult};

/// Intermediate representation
pub use ir::Ir;
use microcad_lang_parse::Ast;

impl Artifact for Ir {
    fn kind() -> ArtifactKind {
        ArtifactKind::Ir
    }
}

pub(crate) trait IsDefault {
    fn is_default(&self) -> bool;
}

// The single function you point Serde to
pub(crate) fn is_default<T: IsDefault>(t: &T) -> bool {
    t.is_default()
}

impl<T> IsDefault for Box<[T]> {
    fn is_default(&self) -> bool {
        self.is_empty() // No PartialEq bound required!
    }
}

impl IsDefault for SrcRef {
    fn is_default(&self) -> bool {
        self.is_none()
    }
}

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

pub struct LowerContext<'source> {
    pub source: &'source Source,
    line_index: LineIndex,
    pub errors: Vec<LowerError>,
}

impl<'source> LowerContext<'source> {
    pub fn diag(&mut self, err: LowerError) {
        self.errors.push(err);
    }
}

impl<'source> From<&'source Source> for LowerContext<'source> {
    fn from(source: &'source Source) -> Self {
        Self {
            source,
            line_index: LineIndex::from(source),
            errors: Vec::default(),
        }
    }
}

impl<'source> SpanToSrcRef for LowerContext<'source> {
    fn span_to_src_ref(&self, span: &Span) -> SrcRef {
        self.line_index.src_ref(self.source.code.as_str(), span)
    }
}

pub trait Lower<AstNode>: Sized {
    fn lower(node: &AstNode, context: &mut LowerContext) -> LowerResult<Self>;
}

pub fn lower(source: &Source, ast: &Ast) -> CompilationResult<Ir> {
    let mut context = LowerContext::from(source);

    // Short-circuit on fatal errors
    let ir = match Ir::lower(ast, &mut context) {
        Ok(ir) => ir,
        Err(fatal_error) => {
            // Ensure the fatal error is logged in the diagnostics
            context.diag(fatal_error);
            return Err(context.errors.into());
        }
    };

    let diagnostics: Diagnostics = context.errors.into();
    if diagnostics.has_errors() {
        Err(diagnostics)
    } else {
        Ok((ir, diagnostics))
    }
}
