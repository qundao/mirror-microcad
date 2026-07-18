// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scaffolding builds Unresolved Symbol Tree as mid-level intermediate represenation from IR.

mod path_resolver;
mod scaffoldable;
mod tree_builder;

use microcad_lang_base::{
    CompilationResult, Diagnostics, Source, SourceLocation, SrcRef, SrcReferrer,
};
use miette::Diagnostic;
use thiserror::Error;

use crate::{Mir, mir, scaffold::scaffoldable::Scaffoldables, tree::SymbolMetadata};

pub use path_resolver::{DefaultPathResolver, PathResolver};
pub use tree_builder::TreeBuilder;

use microcad_lang_lower::{Ir, ir};

#[derive(Debug, Error, Diagnostic)]
pub enum ScaffoldError {
    #[error("Source has no file path: {loc}")]
    SourceHasNoPath {
        loc: SourceLocation,
        #[label("The source code")]
        src_ref: SrcRef,
    },
}

impl SrcReferrer for ScaffoldError {
    fn src_ref(&self) -> SrcRef {
        match &self {
            ScaffoldError::SourceHasNoPath { src_ref, .. } => *src_ref,
        }
    }
}

pub struct ScaffoldContext<'source> {
    pub(crate) path_resolver: Box<dyn PathResolver<'source> + 'source>,
    pub(crate) diags: Vec<ScaffoldError>,
    // pub(crate) path_resolver: Box<dyn FileModulePathResolver>>
}

impl<'source> ScaffoldContext<'source> {
    pub fn diag(&mut self, err: ScaffoldError) {
        self.diags.push(err);
    }
}

impl<'source> From<&'source Source> for ScaffoldContext<'source> {
    fn from(source: &'source Source) -> Self {
        ScaffoldContext {
            path_resolver: Box::new(DefaultPathResolver { source }),
            diags: vec![],
        }
    }
}

pub type ScaffoldResult = Result<mir::UnresolvedSymbolTree, ScaffoldError>;

pub trait Scaffold {
    fn scaffold(&self, context: &mut ScaffoldContext) -> ScaffoldResult;
}

impl Scaffold for ir::FileModule {
    fn scaffold(&self, context: &mut ScaffoldContext) -> ScaffoldResult {
        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::UnresolvedSymbolDef::FileModule(mir::FileModule {
                attr: self.attr.clone().into(),
                path: context.path_resolver.file_module_path_as_string(&self.id)?,
            }), // TODO Resolve file name already here.
        )
        .into())
    }
}

impl Scaffold for ir::ExplicitAlias {
    fn scaffold(&self, _context: &mut ScaffoldContext) -> ScaffoldResult {
        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::Alias(self.path.clone()),
        )
        .into())
    }
}

impl Scaffold for ir::WildcardAlias {
    fn scaffold(&self, _context: &mut ScaffoldContext) -> ScaffoldResult {
        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: None,
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::Wildcard(self.path.clone()),
        )
        .into())
    }
}

impl Scaffold for ir::Constant {
    fn scaffold(&self, _context: &mut ScaffoldContext) -> ScaffoldResult {
        // TODO: Check id is in UPPERCASE

        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::Constant {
                attr: self.attr.clone().into(),
                ty: self.ty.clone(),
                expr: self.expr.clone(),
            },
        )
        .into())
    }
}

impl Scaffold for ir::InlineModule {
    fn scaffold(&self, context: &mut ScaffoldContext) -> ScaffoldResult {
        let mut builder = TreeBuilder::new(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: ir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            mir::UnresolvedSymbolDef::InlineModule(mir::InlineModule {
                attr: (self.outer_attr.clone(), self.inner_attr.clone()).into(),
            }),
        ));
        builder.scaffold(context, self.items.scaffoldables())?;
        Ok(builder.build())
    }
}

impl Scaffold for ir::Function {
    fn scaffold(&self, context: &mut ScaffoldContext) -> ScaffoldResult {
        let mut builder = TreeBuilder::new(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: ir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            mir::Function {
                attr: (self.outer_attr.clone(), self.inner_attr.clone()).into(),
                statements: self.statements.clone(),
                parameters: self.signature.parameters.clone().into(),
                return_ty: self.signature.return_type.clone(),
            },
        ));

        builder.scaffold(context, self.items.scaffoldables())?;
        Ok(builder.build())
    }
}

impl From<ir::Workbench> for mir::Workbench {
    fn from(workbench: ir::Workbench) -> Self {
        Self {
            kind: workbench.kind,
            inits: workbench
                .inits
                .into_iter()
                .map(|init| mir::Init {
                    attr: init.attr.clone().into(),
                    parameters: init.parameters.into(),
                    statements: init.statements.clone(),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            parameters: workbench.parameters,
            statements: workbench.statements.clone(),
        }
    }
}

impl Scaffold for ir::Workbench {
    fn scaffold(&self, context: &mut ScaffoldContext) -> ScaffoldResult {
        let mut builder = TreeBuilder::new(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: ir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            mir::Workbench::from(self.clone()),
        ));

        builder.scaffold(context, self.items.scaffoldables())?;
        Ok(builder.build())
    }
}

impl Scaffold for ir::Source {
    fn scaffold(&self, context: &mut ScaffoldContext) -> ScaffoldResult {
        let mut builder = TreeBuilder::new(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(mir::Identifier::from("root")), // Might be some
                visibility: mir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            mir::SourceFile {
                attr: self.attr.clone().into(),
                statements: self.statements.clone(),
            },
        ));

        builder.scaffold(context, self.items.scaffoldables())?;
        Ok(builder.build())
    }
}

impl Scaffold for Ir {
    fn scaffold(&self, context: &mut ScaffoldContext) -> ScaffoldResult {
        self.tree.scaffold(context)
    }
}

pub fn scaffold<'source>(
    ir: &Ir,
    context: impl Into<ScaffoldContext<'source>>,
) -> CompilationResult<Mir> {
    use microcad_lang_base::ToHash;
    let mut context = context.into();

    match ir.scaffold(&mut context) {
        Ok(tree) => {
            let diagnostics: Diagnostics = context.diags.into();
            if diagnostics.has_errors() {
                Err(diagnostics)
            } else {
                Ok((
                    Mir {
                        input_hash: ir.output_hash,
                        output_hash: tree.to_hash(),
                        tree,
                    },
                    diagnostics,
                ))
            }
        }
        Err(fatal_error) => {
            // Ensure the fatal error is logged in the diagnostics
            context.diag(fatal_error);
            Err(context.diags.into())
        }
    }
}
