// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lowering compile step

pub mod ir;

mod lower;

use microcad_builtin::BuiltinRegistry;
use microcad_lang_base::{
    CompilationResult, Diagnostics, HashId, Source, Span, SpanToSrcRef, SrcRef, SymbolId, ToHash,
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

pub trait Unresolver {
    fn unresolve(&self, id: impl Into<SymbolId>) -> String;
}

pub trait MakeHumanReadable {
    fn make_human_readable<U: Unresolver>(&mut self, unresolver: &U);
}

impl<T> MakeHumanReadable for Box<T>
where
    T: MakeHumanReadable,
{
    fn make_human_readable<U: Unresolver>(&mut self, unresolver: &U) {
        self.as_mut().make_human_readable(unresolver);
    }
}

impl<T> MakeHumanReadable for Box<[T]>
where
    T: MakeHumanReadable,
{
    fn make_human_readable<U: Unresolver>(&mut self, unresolver: &U) {
        self.iter_mut()
            .for_each(|expr| expr.make_human_readable(unresolver));
    }
}

impl<T> MakeHumanReadable for Option<T>
where
    T: MakeHumanReadable,
{
    fn make_human_readable<U: Unresolver>(&mut self, unresolver: &U) {
        if let Some(s) = self.as_mut() {
            s.make_human_readable(unresolver);
        }
    }
}

impl MakeHumanReadable for Ir {
    fn make_human_readable<U: Unresolver>(&mut self, unresolver: &U) {
        self.tree.make_human_readable(unresolver);
    }
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
    pub builtins: BuiltinRegistry,
    pub errors: Vec<LowerError>,
}

impl<'source> LowerContext<'source> {
    pub fn new(source: &'source Source) -> Self {
        Self {
            source,
            builtins: BuiltinRegistry::new(),
            errors: vec![],
        }
    }
    pub fn diag(&mut self, err: impl Into<LowerError>) {
        self.errors.push(err.into());
    }
}

impl<'source> From<&'source Source> for LowerContext<'source> {
    fn from(source: &'source Source) -> Self {
        Self::new(source)
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

impl Unresolver for BuiltinRegistry {
    fn unresolve(&self, id: impl Into<SymbolId>) -> String {
        let id = id.into();
        match id {
            SymbolId::Builtin(builtin_id) => {
                if let Some(builtin) = self.get(builtin_id) {
                    String::from(builtin.name())
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
}

pub fn lower<'source>(context: &mut LowerContext<'source>, ast: &Ast) -> CompilationResult<Ir> {
    // Short-circuit on fatal errors
    let ir = match Ir::lower(ast, context) {
        Ok(mut ir) => {
            ir.make_human_readable(&context.builtins);
            ir
        }
        Err(fatal_error) => {
            let errors = std::mem::take(&mut context.errors);
            // Ensure the fatal error is logged in the diagnostics
            context.diag(fatal_error);
            return Err(errors.into());
        }
    };

    let errors = std::mem::take(&mut context.errors);
    let diagnostics: Diagnostics = errors.into();
    if diagnostics.has_errors() {
        Err(diagnostics)
    } else {
        Ok((ir, diagnostics))
    }
}
