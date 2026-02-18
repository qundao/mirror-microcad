// Copyright © 2025 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{builtin::*, model::*};

/// Builtin definition for a translation.
#[derive(Debug)]
pub struct Translate;

impl BuiltinWorkbenchDefinition for Translate {
    fn id() -> &'static str {
        "translate"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Transform
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        use microcad_lang::value::*;

        &|args| {
            Ok(BuiltinWorkpieceOutput::Transform(
                AffineTransform::Translation(Vec3::new(
                    *args.get::<Length>("x"),
                    *args.get::<Length>("y"),
                    *args.get::<Length>("z"),
                )),
            ))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(x: Length = Length::default()),
            parameter!(y: Length = Length::default()),
            parameter!(z: Length = Length::default()),
        ]
        .into_iter()
        .collect()
    }
}
