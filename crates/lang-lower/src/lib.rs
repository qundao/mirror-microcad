// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lowering compile step

pub mod ir;

mod lower;

use microcad_lang_base::{
    CompilationResult, Diagnostics, HashId, Source, Span, SpanToSrcRef, SrcRef, ToHash,
};

pub use ir::CastInto;

pub use lower::{LowerError, LowerResult};

/// Intermediate representation
use microcad_lang_parse::Ast;
use microcad_lang_proc_macros::Artifact;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Artifact)]
pub struct Ir {
    pub input_hash: HashId,
    pub output_hash: HashId,
    pub tree: ir::Source,
}

impl Lower<Ast> for Ir {
    fn lower(node: &Ast, context: &mut LowerContext) -> LowerResult<Self> {
        let tree = ir::Source::lower(node.tree(), context)?;

        Ok(Self {
            input_hash: node.output_hash(),
            output_hash: tree.to_hash(),
            tree,
        })
    }
}

pub struct LowerContext<'source> {
    pub source: &'source Source,
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
            errors: Vec::default(),
        }
    }
}

impl<'source> SpanToSrcRef for LowerContext<'source> {
    fn span_to_src_ref(&self, span: &Span) -> SrcRef {
        self.source.span_to_src_ref(span)
    }
}

pub trait Lower<AstNode>: Sized {
    fn lower(node: &AstNode, context: &mut LowerContext) -> LowerResult<Self>;
}

pub fn lower<'source>(
    context: impl Into<LowerContext<'source>>,
    ast: &Ast,
) -> CompilationResult<Ir> {
    let mut context = context.into();

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
