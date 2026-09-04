// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lowering compile step

pub mod ir;

mod error;
pub use error::{LowerError, LowerResult};

mod desugar;
mod fold;
mod scaffold;

use microcad_builtin::BuiltinRegistry;
use microcad_lang_base::{
    CompilationResult, HashId, LookUpName, PushDiag, Source, Span, SpanToSrcRef, SrcRef, SymbolId,
    hash_id,
};

pub use ir::CastInto;

/// Intermediate representation
use microcad_lang_parse::Ast;
use microcad_macros::Artifact;
use serde::{Deserialize, Serialize};

use crate::{ir::Arena, scaffold::Scaffold};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Artifact)]
pub struct Ir {
    pub input_hash: HashId,
    pub output_hash: HashId,
    pub tree: ir::Tree,
}

pub trait Unresolver {
    fn unresolve(&self, id: impl Into<SymbolId>) -> String;
}

impl Desugar<Ast> for ir::desugared::Source {
    fn desugar(node: &Ast, context: &mut LowerContext) -> LowerResult<Self> {
        ir::desugared::Source::desugar(node.tree(), context)
    }
}

pub struct LowerContext<'source> {
    pub source: &'source Source,
    pub arena: ir::Arena,
    pub node_id_stack: Vec<ir::NodeId>,
    pub builtins: BuiltinRegistry,
    pub errors: Vec<LowerError>,
}

impl<'source> LookUpName for LowerContext<'source> {
    fn look_up_built_in_name(
        &self,
        builtin_id: &microcad_builtin::BuiltinId,
    ) -> Option<microcad_lang_base::Name> {
        self.builtins.look_up_built_in_name(builtin_id)
    }
}

impl<'source> LowerContext<'source> {
    pub fn new(source: &'source Source) -> Self {
        Self {
            source,
            arena: Arena::default(),
            node_id_stack: vec![],
            builtins: BuiltinRegistry::new(),
            errors: vec![],
        }
    }

    pub fn top_node(&self) -> &ir::NodeId {
        self.node_id_stack.last().unwrap()
    }

    pub fn scaffold_item(&mut self, node: impl Into<ir::Item>) -> ir::NodeId {
        self.arena.new_node(node.into())
    }

    pub fn scaffold_item_with_children(
        &mut self,
        node: impl Into<ir::Item>,
        children: impl Scaffold,
    ) -> ir::NodeId {
        let node_id = self.scaffold_item(node);
        self.node_id_stack.push(node_id);
        children.scaffold(self);
        self.node_id_stack.pop();
        node_id
    }
}

impl<'source> PushDiag<LowerError> for LowerContext<'source> {
    fn push_diag(&mut self, err: impl Into<LowerError>) {
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

pub trait Desugar<AstNode>: Sized {
    fn desugar(node: &AstNode, context: &mut LowerContext) -> LowerResult<Self>;
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

pub fn lower<'source>(
    context: &mut LowerContext<'source>,
    ast: &Ast,
) -> CompilationResult<Ir, LowerError> {
    // Short-circuit on fatal errors
    let ir = match ir::desugared::Source::desugar(ast.tree(), context) {
        Ok(ir) => ir,
        Err(fatal_error) => {
            // Ensure the fatal error is logged in the diagnostics
            context.push_diag(fatal_error);
            let errors = std::mem::take(&mut context.errors);
            return Err(errors);
        }
    };

    let root = ir.scaffold(context);
    let arena = std::mem::take(&mut context.arena);
    let mut tree = ir::Tree::from((root, arena));

    {
        use crate::ir::visitor::VisitorMut;
        fold::Fold::new(context).visit_tree(&mut tree);
    }

    let ir = Ir {
        input_hash: ast.output_hash(),
        output_hash: hash_id!(tree),
        tree,
    };

    let errors = std::mem::take(&mut context.errors);

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok((ir, errors))
    }
}
