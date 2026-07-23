// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scaffolding builds Unresolved Symbol Tree as mid-level intermediate represenation from IR.

mod path_resolver;
mod scaffoldable;
mod tree_builder;

use crate::{
    Mir, ResolveContext, mir, resolve::ResolveError, scaffold::scaffoldable::Scaffoldables,
    tree::SymbolMetadata,
};
use microcad_lang_base::{CompilationResult, Diagnostics, Identifier, Refer, SrcRef, SrcReferrer};

pub use path_resolver::PathResolver;
pub use tree_builder::TreeBuilder;

use microcad_lang_lower::{CastInto, Ir, ir};

pub type ScaffoldResult = Result<mir::UnresolvedSymbolTree, ResolveError>;

pub trait Scaffold {
    fn scaffold(&self, context: &mut ResolveContext) -> ScaffoldResult;
}

impl Scaffold for ir::FileModule {
    fn scaffold(&self, context: &mut ResolveContext) -> ScaffoldResult {
        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::UnresolvedSymbolDef::FileModule(mir::FileModule {
                attr: self.attr.0.clone().cast_into(),
                //path: context.path_resolver.file_module_path_as_string(&self.id)?,
            }), // TODO Resolve file name already here.
        )
        .into())
    }
}

impl Scaffold for ir::ExplicitAlias {
    fn scaffold(&self, _context: &mut ResolveContext) -> ScaffoldResult {
        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::Alias(self.path.clone().into()),
        )
        .into())
    }
}

impl Scaffold for ir::WildcardAlias {
    fn scaffold(&self, _context: &mut ResolveContext) -> ScaffoldResult {
        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: None,
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::Wildcard(self.path.clone().into()),
        )
        .into())
    }
}

impl Scaffold for ir::Constant {
    fn scaffold(&self, _context: &mut ResolveContext) -> ScaffoldResult {
        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::Constant {
                attr: self.attr.0.clone().cast_into(),
                ty: self.ty.clone(),
                expr: self.expr.clone().cast_into(),
            },
        )
        .into())
    }
}

impl Scaffold for ir::InlineModule {
    fn scaffold(&self, context: &mut ResolveContext) -> ScaffoldResult {
        let mut builder = TreeBuilder::new(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::UnresolvedSymbolDef::InlineModule(mir::InlineModule {
                attr: self
                    .outer_attr
                    .0
                    .clone()
                    .extend(self.inner_attr.0.clone())
                    .cast_into(),
            }),
        ));
        builder.scaffold(context, self.items.scaffoldables())?;
        Ok(builder.build())
    }
}

impl Scaffold for ir::Function {
    fn scaffold(&self, context: &mut ResolveContext) -> ScaffoldResult {
        let mut builder = TreeBuilder::new(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_ref,
            },
            mir::Function {
                attr: self
                    .outer_attr
                    .0
                    .clone()
                    .extend(self.inner_attr.0.clone())
                    .cast_into(),
                statements: self.statements.clone().cast_into(),
                parameters: self.signature.parameters.clone().cast_into(),
                return_ty: self.signature.return_type.clone(),
            },
        ));

        builder.scaffold(context, self.items.scaffoldables())?;
        Ok(builder.build())
    }
}

impl Scaffold for ir::Workbench {
    fn scaffold(&self, context: &mut ResolveContext) -> ScaffoldResult {
        let mut builder = TreeBuilder::new(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref(),
                keyword_src_ref: self.keyword_ref,
            },
            mir::Workbench {
                kind: self.kind.clone(),
                inits: self
                    .inits
                    .iter()
                    .map(|init| mir::Init {
                        attr: init.attr.0.clone().cast_into(),
                        parameters: init.parameters.clone().cast_into(),
                        statements: init.statements.clone().cast_into(),
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                parameters: self.parameters.clone().cast_into(),
                statements: self.statements.clone().cast_into(),
            },
        ));

        builder.scaffold(context, self.items.scaffoldables())?;
        Ok(builder.build())
    }
}

impl Scaffold for ir::Source {
    fn scaffold(&self, context: &mut ResolveContext) -> ScaffoldResult {
        let mut builder = TreeBuilder::new(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: self
                    .id
                    .as_ref()
                    .map(|id| Identifier(Refer::none(id.clone()))),
                visibility: mir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            mir::SourceFile {
                attr: self.attr.0.clone().cast_into(),
                statements: self.statements.clone().cast_into(),
            },
        ));

        builder.scaffold(context, self.items.scaffoldables())?;
        Ok(builder.build())
    }
}

impl Scaffold for Ir {
    fn scaffold(&self, context: &mut ResolveContext) -> ScaffoldResult {
        self.tree.scaffold(context)
    }
}

pub fn scaffold(ir: &Ir, context: impl Into<ResolveContext>) -> CompilationResult<Mir> {
    use microcad_lang_base::ToHash;
    let mut context = context.into();

    match ir.scaffold(&mut context) {
        Ok(tree) => {
            let diagnostics: Diagnostics = context.diagnostics.into();
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
            // context.diag(fatal_error);
            Err(context.diagnostics.into())
        }
    }
}
