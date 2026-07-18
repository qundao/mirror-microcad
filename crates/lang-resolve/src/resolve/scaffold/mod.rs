// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scaffolding builds Unresolved Symbol Tree as mid-level intermediate represenation from IR.

mod path_resolver;
mod scaffoldable;
mod tree_builder;

use microcad_lang_base::{
    CompilationResult, Diagnostics, Refer, Source, SourceLocation, SrcRef, SrcReferrer,
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

impl From<ir::DocBlock> for mir::DocBlock {
    fn from(value: ir::DocBlock) -> Self {
        let src_ref = value.src_ref();
        Self(Refer::new(
            value.0.into_iter().collect::<Vec<_>>().join("\n"),
            src_ref,
        ))
    }
}

impl Scaffold for ir::FileModule {
    fn scaffold(&self, _context: &mut ScaffoldContext) -> ScaffoldResult {
        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                doc: self.attr.doc.clone().into(),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::UnresolvedSymbolDef::FileModule, // TODO Resolve file name already here.
        )
        .into())
    }
}

impl Scaffold for ir::ExplicitAlias {
    fn scaffold(&self, _context: &mut ScaffoldContext) -> ScaffoldResult {
        Ok(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                doc: self.attr.doc.clone().into(),
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
                doc: self.attr.doc.clone().into(),
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
                doc: self.attr.doc.clone().into(),
                visibility: self.visibility.clone(),
                src_ref: self.src_ref,
                keyword_src_ref: self.keyword_src_ref,
            },
            mir::Constant {
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
                doc: self.outer_attr.doc.clone().into(),
                visibility: ir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            mir::UnresolvedSymbolDef::InlineModule,
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
                doc: self.outer_attr.doc.clone().into(),
                visibility: ir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            mir::Function {
                statements: self.statements.clone(),
                parameters: self.signature.parameters.clone().into(),
                return_ty: self.signature.return_type.clone(),
            },
        ));

        builder.scaffold(context, self.items.scaffoldables())?;
        Ok(builder.build())
    }
}

impl From<ir::Parameter> for mir::Parameter {
    fn from(parameter: ir::Parameter) -> Self {
        Self {
            doc: parameter.attr.doc.clone().into(),
            id: parameter.id.clone(),
            ty: parameter.specified_type,
            default_value: parameter.default_value,
            src_ref: parameter.src_ref,
        }
    }
}

impl From<ir::ParameterList> for mir::ParameterList {
    fn from(parameter_list: ir::ParameterList) -> Self {
        Self {
            parameters: parameter_list
                .0
                .value
                .into_iter()
                .map(|param| mir::Parameter::from(param))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl From<ir::Workbench> for mir::Workbench {
    fn from(workbench: ir::Workbench) -> Self {
        let parameters: mir::ParameterList = workbench.parameters.into();
        let statements = parameters
            .parameters
            .iter()
            .cloned()
            .map(|param| mir::InitStatement {
                id: param.id,
                ty: param.ty,
                expression: todo!(), //param.default_value.clone(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let default_init = mir::Init {
            doc: workbench.outer_attr.doc.clone().into(),
            parameters,
            statements,
        };

        Self {
            kind: workbench.kind,
            inits: workbench
                .inits
                .into_iter()
                .map(|init| mir::Init {
                    doc: init.attr.doc.clone().into(),
                    parameters: init.parameters.into(),
                    statements: init
                        .statements
                        .iter()
                        .map(|stmt| mir::InitStatement {
                            id: todo!(),
                            ty: todo!(),
                            expression: todo!(),
                        })
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                })
                .chain([default_init].into_iter())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            statements: workbench
                .statements
                .into_iter()
                .map(|stmt| mir::WorkbenchStatement {
                    attr: todo!(),
                    src_ref: todo!(),
                    visibility: todo!(),
                    keyword_src_ref: todo!(),
                    id: todo!(),
                    ty: todo!(),
                    expression: todo!(),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl Scaffold for ir::Workbench {
    fn scaffold(&self, context: &mut ScaffoldContext) -> ScaffoldResult {
        let mut builder = TreeBuilder::new(mir::UnresolvedSymbol::new(
            SymbolMetadata {
                id: Some(self.id.clone()),
                doc: self.outer_attr.doc.clone().into(),
                visibility: ir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            mir::Workbench {
                statements: self.statements.clone(),
                kind: self.kind.clone(),
                inits: todo!(),
            },
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
                doc: self.attr.doc.clone().into(),
                visibility: mir::Visibility::Public,
                src_ref: SrcRef::none(),
                keyword_src_ref: SrcRef::none(),
            },
            mir::SourceFile {
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
