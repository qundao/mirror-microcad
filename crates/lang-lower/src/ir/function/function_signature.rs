// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function signature syntax element

use crate::ir;

use microcad_lang_base::{Identifier, SrcRef};
use microcad_lang_proc_macros::SrcReferrer;

/// Parameters and return type of a function
#[derive(Clone, Debug, SrcReferrer)]
pub struct FunctionSignature {
    /// Function's parameters
    pub parameters: ir::ParameterList,
    /// Function's return type
    pub return_type: Option<ir::TypeAnnotation>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl FunctionSignature {
    /// Get parameter by name
    pub fn parameter_by_name(&self, name: &Identifier) -> Option<&ir::Parameter> {
        use crate::Identifiable;
        self.parameters.iter().find(|arg| arg.id_ref() == name)
    }
}

impl std::fmt::Display for FunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}){}",
            self.parameters,
            if let Some(ret) = &self.return_type {
                format!("-> {ret}")
            } else {
                String::default()
            }
        )
    }
}
