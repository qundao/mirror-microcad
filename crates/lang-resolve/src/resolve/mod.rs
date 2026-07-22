// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod scaffold;

mod case_check;
mod type_check;

use microcad_lang_base::{
    CompilationResult, Diagnostics, HashId, Source, SrcRef, SrcReferrer, element::Case,
};
use microcad_lang_lower::Ir;
use miette::Diagnostic;
use thiserror::Error;

use crate::{
    Rst, mir,
    rst::{self, ResolvedSymbolRef},
    scaffold::{ScaffoldContext, ScaffoldError},
};

/// Resolve error.
#[derive(Debug, Error, Diagnostic)]
pub enum ResolveError {
    #[error("{0}")]
    ScaffoldError(#[from] ScaffoldError),

    #[error("Wrong case")]
    #[diagnostic(severity = "warning")]
    WrongCase {
        expected: Case,
        actual: Case,
        #[label]
        src_ref: SrcRef,
    },
    #[error("Type mismatch: {specified} != {actual}")]
    TypeMismatch {
        specified: rst::Type,
        #[label("Specified type")]
        specified_src_ref: SrcRef,
        actual: rst::Type,
        #[label("Actual type")]
        actual_src_ref: SrcRef,
    },
    #[error("No source with hash: {0}")]
    NoSourceWithHash(HashId),
}

impl SrcReferrer for ResolveError {
    fn src_ref(&self) -> SrcRef {
        match self {
            ResolveError::ScaffoldError(err) => err.src_ref(),
            ResolveError::WrongCase { src_ref, .. } => *src_ref,
            ResolveError::TypeMismatch {
                specified_src_ref, ..
            } => *specified_src_ref,
            ResolveError::NoSourceWithHash(_) => SrcRef::none(),
        }
    }
}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;

/// Resolve Context
pub struct ResolveContext {
    //pub stack: ResolveStack,
    pub source_hash_id: HashId,
    //pub file_module_resolver: Box<dyn ResolveFileModule>,
    /// Diagnostic handler.
    //pub rst_cache: Box<dyn RstCacheInterface>,
    pub diagnostics: Diagnostics,

    pub tree: rst::ResolvedSymbolTree,
}

impl ResolveContext {
    pub fn new(source: Source) -> Self {
        let mut source_map = microcad_lang_base::SourceMap::new();
        let source_hash_id = source.hash_id();
        source_map.insert(source);

        Self {
            source_hash_id,
            diagnostics: Diagnostics::default(),
            tree: rst::ResolvedSymbolTree::new(),
        }
    }

    fn get_symbol<'tree>(&mut self, name: mir::SymbolPath) -> ResolvedSymbolRef<'tree> {
        todo!()
    }

    pub fn diag<E>(&mut self, err: E)
    where
        E: Into<miette::Report> + SrcReferrer,
    {
        self.diagnostics.push(err)
    }

    /*pub fn top<'tree>(&'tree self) -> SymbolRef<'tree, UnresolvedSymbolDef> {
        self.builder
            .tree
            .get(*self.builder.stack.last().unwrap())
            .unwrap()
    }*/

    fn scaffold_context<'source>(
        &'source self,
        source: &'source Source,
    ) -> ScaffoldContext<'source> {
        ScaffoldContext::from(source)
    }
}

/// Trait to resolve an IR node into a symbol.
pub trait Resolve<T = rst::Rst> {
    fn resolve(&self, context: &mut ResolveContext) -> ResolveResult<T>;
}

impl Resolve<rst::WorkbenchStatement> for mir::WorkbenchStatement {
    fn resolve(&self, context: &mut ResolveContext) -> ResolveResult<rst::WorkbenchStatement> {
        Ok(rst::WorkbenchStatement {
            attr: todo!(),
            src_ref: todo!(),
            visibility: todo!(),
            keyword_src_ref: todo!(),
            id: todo!(),
            ty: todo!(),
            expression: todo!(),
        })
    }
}

pub fn resolve(context: &mut ResolveContext, ir: &Ir) -> CompilationResult<Rst> {
    /*let source = context
        .source_map
        .get_by_hash_id(context.source_hash_id)
        .expect("TODO Error handling");
    use microcad_lang_base::ToHash;
    let (mir, diag) = scaffold::scaffold(ir, context.scaffold_context(source))?;

    context.diagnostics.append(diag);

    let root = mir.tree.root().expect("Some root node and error handling");

    // Convert unresolved symbols into resolved symbols
    let tree = rst::ResolvedSymbolTree::from_unresolved(root, context);

    let mut diags = Diagnostics::default();
    diags.append(context.diagnostics);
    Ok((
        Rst {
            input_hash: mir.output_hash,
            output_hash: tree.to_hash(),
            tree,
        },
        diags,
    ))*/
    todo!()
}

/*


pub fn resolve_constant<'tree>(
    constant: &rst::def::Constant,
    parent: rst::SymbolRef<'tree, UnresolvedSymbolDef>,
) -> ResolveResult<Value> {
    use ir::ConstantExpression::*;
    match &constant.expr {
        Invalid => todo!(),
        Literal(literal) => Ok(literal.value().clone()),
        Name(name) => {
            use UnresolvedSymbolDef::*;
            let path: rst::SymbolPath = name.into();
            let resolved = parent.resolve(path.clone());
            match resolved {
                Some(symbol) => match &symbol.def {
                    SourceFile(_) => todo!(),
                    InlineModule => todo!(),
                    FileModule => todo!(),
                    Workbench => todo!(),
                    Function(_) => todo!(),
                    Constant(constant) => resolve_constant(constant, symbol),
                    Builtin => todo!(),
                    Alias => todo!(),
                    Wildcard => todo!(),
                },
                None => todo!("Error handling"),
            }
        }
        FormatString(_) => todo!(),
        ArrayExpression(_) => todo!(),
        TupleExpression(_) => todo!(),
        BinaryOp(_) => todo!(),
        UnaryOp(_) => todo!(),
    }
}


*/
