// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::SrcReferrer;
use microcad_lang_types::{Value, ty::Ty};

use crate::{ResolveResult, resolve::ResolveError, rst};

pub trait TypeCheck {
    fn specified_type(&self) -> &rst::TypeAnnotation {
        &None
    }

    fn actual_type(&self) -> &rst::TypeAnnotation;

    fn type_check(&self) -> ResolveResult<()> {
        match (self.specified_type(), self.actual_type()) {
            (Some(specified), Some(actual)) => {
                if actual == specified {
                    Ok(())
                } else {
                    Err(ResolveError::TypeMismatch {
                        specified: specified.ty(),
                        specified_src_ref: specified.src_ref(),
                        actual: actual.ty(),
                        actual_src_ref: actual.src_ref(),
                    })
                }
            }
            _ => Ok(()),
        }
    }
}
