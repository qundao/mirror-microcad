// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin boolean operations.

use microcad_core::{BooleanOp, Geometry2D};

use crate::{builtin::*, eval::ArgumentMatch, model::*, render::*, src_ref::SrcRef, value::Tuple};

impl Operation for BooleanOp {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model| {
            let model = model.into_group().unwrap_or(model);
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render_with_context(context)?;

            Ok(Geometry2D::MultiPolygon(geometries.boolean_op(self)))
        })
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model| {
            let model = model.into_group().unwrap_or(model);
            let model_ = model.borrow();
            let geometries: Geometries3D = model_.children.render_with_context(context)?;

            Ok(Geometry3D::Manifold(geometries.boolean_op(self)))
        })
    }
}

/// Union operation.
pub struct Union;

impl BuiltinWorkbenchDefinition for Union {
    fn id() -> &'static str {
        "union"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(
                BooleanOp::Union,
            )))
        }
    }
}

/// Difference operation.
pub struct Subtract;

impl BuiltinWorkbenchDefinition for Subtract {
    fn id() -> &'static str {
        "subtract"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(
                BooleanOp::Subtract,
            )))
        }
    }
}

/// Intersection operation.
pub struct Intersect;

impl BuiltinWorkbenchDefinition for Intersect {
    fn id() -> &'static str {
        "intersect"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(
                BooleanOp::Intersect,
            )))
        }
    }
}

/// An operation that repeats a geometry n times.
pub struct Repeat;

impl Operation for Repeat {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model| {
            Ok(Geometry2D::Collection(
                model.borrow().children.render_with_context(context)?,
            ))
        })
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model| {
            Ok(Geometry3D::Collection(
                model.borrow().children.render_with_context(context)?,
            ))
        })
    }
}

impl BuiltinWorkbenchDefinition for Repeat {
    fn id() -> &'static str {
        "repeat"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        unimplemented!()
    }

    /// Workbench function
    fn function() -> &'static BuiltinFn {
        &|params, args, _| {
            log::trace!(
                "Built-in workbench {call} {id:?}({args})",
                call = crate::mark!(CALL),
                id = Self::id()
            );

            Ok(Value::Model(
                ArgumentMatch::find_multi_match(args, params)?
                    .iter()
                    .flat_map(|args| {
                        (0..args.get::<Integer>("n"))
                            .map(|_| {
                                ModelBuilder::new(Element::InputPlaceholder, SrcRef(None)).build()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Models>()
                    .to_multiplicity(SrcRef(None)),
            ))
        }
    }

    fn parameters() -> ParameterValueList {
        [parameter!(n: Integer)].into_iter().collect()
    }
}

impl From<BooleanOp> for BuiltinWorkpiece {
    fn from(value: BooleanOp) -> Self {
        match value {
            BooleanOp::Union => Union::workpiece(Creator::new(Union::symbol(), Tuple::default())),
            BooleanOp::Subtract => {
                Subtract::workpiece(Creator::new(Subtract::symbol(), Tuple::default()))
            }
            BooleanOp::Intersect => {
                Intersect::workpiece(Creator::new(Intersect::symbol(), Tuple::default()))
            }
            BooleanOp::Complement => unimplemented!(),
        }
    }
}
