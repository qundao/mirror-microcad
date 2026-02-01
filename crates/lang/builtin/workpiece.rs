// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin function evaluation entity

use custom_debug::Debug;
use strum::Display;

use crate::{
    builtin::*, eval::*, model::*, render::*, resolve::*, src_ref::*, syntax::*, value::*,
};

/// The kind of the built-in workbench determines its output.
#[derive(Debug, Clone, Display, PartialEq)]
pub enum BuiltinWorkbenchKind {
    /// A parametric 2D primitive.
    Primitive2D,
    /// A parametric 3D primitive.
    Primitive3D,
    /// An affine transformation.
    Transform,
    /// An operation on a model.
    Operation,
}

/// The return value when calling a built-in workpiece.
pub enum BuiltinWorkpieceOutput {
    /// 2D geometry output.
    Primitive2D(Box<dyn RenderWithContext<Geometry2DOutput>>),
    /// 3D geometry output.
    Primitive3D(Box<dyn RenderWithContext<Geometry3DOutput>>),
    /// Transformation.
    Transform(AffineTransform),
    /// Operation.
    Operation(Box<dyn Operation>),
}

/// Builtin sketch function type.
pub type BuiltinWorkpieceFn = dyn Fn(&Tuple) -> RenderResult<BuiltinWorkpieceOutput>;

/// The built-in workpiece.
#[derive(Clone, Debug)]
pub struct BuiltinWorkpiece {
    /// Kind of the workpiece.
    pub kind: BuiltinWorkbenchKind,
    /// Output type.
    pub output_type: OutputType,
    /// Creator symbol.
    pub creator: Hashed<Creator>,
    /// The function that will be called when the workpiece is rendered.
    #[debug(skip)]
    pub f: &'static BuiltinWorkpieceFn,
}

impl BuiltinWorkpiece {
    /// Call the workpiece with its arguments.
    pub fn call(&self) -> RenderResult<BuiltinWorkpieceOutput> {
        (self.f)(&self.creator.arguments)
    }
}

impl std::fmt::Display for BuiltinWorkpiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{kind} {creator}",
            kind = self.kind,
            creator = *self.creator,
        )
    }
}

impl ComputedHash for BuiltinWorkpiece {
    fn computed_hash(&self) -> crate::render::HashId {
        self.creator.computed_hash()
    }
}

/// Builtin part definition
pub trait BuiltinWorkbenchDefinition {
    /// Get id of the builtin part
    fn id() -> &'static str;

    /// The kind of the built-in workbench.
    fn kind() -> BuiltinWorkbenchKind;

    /// A help string as markdown.
    fn help() -> Option<&'static str> {
        None
    }

    /// The expected output type.
    fn output_type() -> OutputType {
        OutputType::NotDetermined
    }

    /// The function that generates an output from the workpiece.
    fn workpiece_function() -> &'static BuiltinWorkpieceFn;

    /// Construct the workpiece from an argument tuple.
    fn workpiece(creator: Creator) -> BuiltinWorkpiece {
        BuiltinWorkpiece {
            kind: Self::kind(),
            output_type: Self::output_type(),
            creator: Hashed::new(creator),
            f: Self::workpiece_function(),
        }
    }

    /// Create model from workpiece and creator.
    fn model(creator: Creator) -> Model {
        let workpiece = Self::workpiece(creator);
        let model = ModelBuilder::new(Element::BuiltinWorkpiece(workpiece), SrcRef(None)).build();

        // Add a @input placeholder if we have a built-in operation or transform.
        // This assures that multiplicity for built-ins is working correctly.
        if Self::kind() == BuiltinWorkbenchKind::Operation
            || Self::kind() == BuiltinWorkbenchKind::Transform
        {
            model.append(ModelBuilder::new(Element::InputPlaceholder, SrcRef(None)).build());
        }
        model
    }

    /// Workbench function
    fn function() -> &'static BuiltinFn {
        &|params, args, context| {
            log::trace!(
                "Built-in workbench {call} {id:?}({args})",
                call = crate::mark!(CALL),
                id = Self::id()
            );
            Ok(Value::Model(
                ArgumentMatch::find_multi_match(args, params)?
                    .args
                    .iter()
                    .map(|tuple| Self::model(Creator::new(context.current_symbol(), tuple.clone())))
                    .collect::<Models>()
                    .to_multiplicity(SrcRef(None)),
            ))
        }
    }

    /// Workbench function
    fn doc() -> Option<DocBlock> {
        Self::help().map(DocBlock::new_builtin)
    }

    /// Part initialization parameters
    fn parameters() -> ParameterValueList {
        ParameterValueList::default()
    }

    /// Create builtin symbol
    fn symbol() -> Symbol {
        Symbol::new_builtin(Builtin {
            id: Identifier::no_ref(Self::id()),
            parameters: Self::parameters(),
            kind: BuiltinKind::Workbench(Self::kind()),
            f: Self::function(),
            doc: Self::doc(),
        })
    }
}
