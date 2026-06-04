// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Type annotation.

use microcad_lang_types::ty;

use microcad_lang_base::Refer;
use microcad_lang_proc_macros::SrcReferrer;

/// Type within source code.
#[derive(Clone, Debug, PartialEq, SrcReferrer)]
pub struct TypeAnnotation(pub Refer<ty::Type>);

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ty::Ty for TypeAnnotation {
    fn ty(&self) -> ty::Type {
        self.0.value.clone()
    }
}
