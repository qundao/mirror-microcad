// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scaffolding builds Unresolved Symbol Tree as mid-level intermediate represenation from IR.

use microcad_lang_base::{CompilationResult, Diagnostics, Refer, Source, SrcRef, SrcReferrer};
use miette::Diagnostic;
use thiserror::Error;

use crate::{
    Mir, mir,
    tree::{SymbolHandle, SymbolMetadata},
};

use microcad_lang_lower::{Ir, ir};

pub struct TreeBuilder {
    pub tree: mir::UnresolvedSymbolTree,
    // Stack of active parents
    pub stack: Vec<SymbolHandle>,
}

impl TreeBuilder {
    pub fn new(root_data: impl Into<mir::UnresolvedSymbolTree>) -> Self {
        Self {
            tree: root_data.into(),
            stack: vec![SymbolHandle::root()],
        }
    }

    /// Add a sub-tree to the current parent
    pub fn add(&mut self, tree: impl Into<mir::UnresolvedSymbolTree>) -> &mut Self {
        let parent = self.stack.last().copied();
        self.tree.insert(parent, tree);
        self
    }

    pub fn scaffold<'a>(
        &mut self,
        context: &mut ScaffoldContext,
        mut items: impl Iterator<Item = &'a dyn Scaffold>,
    ) -> Result<(), ScaffoldError> {
        items.try_for_each(|item| {
            self.add(item.scaffold(context)?);
            Ok(())
        })?;
        Ok(())
    }

    /// Enter a child scope (push to stack)
    pub fn enter(&mut self, tree: impl Into<mir::UnresolvedSymbolTree>) -> &mut Self {
        self.stack
            .push(self.tree.last_handle().expect("At least one node"));
        self.add(tree);
        // The last inserted node (the one we just added) becomes the new parent
        self
    }

    /// Exit the current scope (pop from stack)
    pub fn exit(&mut self) -> &mut Self {
        self.stack.pop();
        self
    }

    pub fn build(self) -> mir::UnresolvedSymbolTree {
        self.tree
    }

    /*
    pub fn build_rst(self) -> Rst {
        let root = self.tree.root().expect("Root node expected");

        // Convert unresolved symbols into resolved symbols
        let nodes: Vec<Symbol<ResolvedSymbolDef>> = root
            .descendants()
            .map(|symbol| def::resolve_symbol(symbol).expect("TODO Error handling"))
            .collect();

        Rst { nodes }
    }

    */
}

// In your logic/compiler layer (where Scaffold is defined)
pub trait ItemsExt {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold>;
}

impl<T> ItemsExt for Box<[T]>
where
    T: Scaffold,
{
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.iter().map(|i| i as &dyn Scaffold)
    }
}

impl ItemsExt for ir::Aliases {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.explicit_aliases
            .scaffoldables()
            .chain(self.wildcards.scaffoldables())
    }
}

impl ItemsExt for ir::FunctionItems {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.aliases
            .scaffoldables()
            .chain(self.constants.scaffoldables())
    }
}

impl ItemsExt for ir::WorkbenchItems {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.aliases
            .scaffoldables()
            .chain(self.constants.scaffoldables())
            .chain(self.functions.scaffoldables())
    }
}

impl ItemsExt for ir::InlineModuleItems {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.aliases
            .scaffoldables()
            .chain(self.constants.scaffoldables())
            .chain(self.modules.scaffoldables())
            .chain(self.functions.scaffoldables())
            .chain(self.workbenches.scaffoldables())
    }
}

impl ItemsExt for ir::SourceItems {
    fn scaffoldables(&self) -> impl Iterator<Item = &dyn Scaffold> {
        self.file_modules
            .scaffoldables()
            .chain(self.aliases.scaffoldables())
            .chain(self.constants.scaffoldables())
            .chain(self.inline_modules.scaffoldables())
            .chain(self.functions.scaffoldables())
            .chain(self.workbenches.scaffoldables())
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum ScaffoldError {}

impl SrcReferrer for ScaffoldError {
    fn src_ref(&self) -> SrcRef {
        SrcRef::none()
    }
}

pub struct ScaffoldContext<'source> {
    pub(crate) source: &'source Source,
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
            source,
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
