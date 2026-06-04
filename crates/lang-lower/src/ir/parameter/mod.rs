// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter syntax elements

mod parameter_list;

use crate::ir;

use microcad_lang_base::{Identifier, OrdMapValue, SrcRef};
use microcad_lang_proc_macros::{Identifiable, SrcReferrer};
pub use parameter_list::*;

/// A parameter of a parameter list.
#[derive(Clone, Debug, Default, SrcReferrer, Identifiable)]
pub struct Parameter {
    /// Name of the parameter
    pub(crate) id: Identifier,
    /// Type of the parameter or `None`
    pub specified_type: Option<ir::TypeAnnotation>,
    /// default value of the parameter or `None`
    pub default_value: Option<ir::Expression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl OrdMapValue<Identifier> for Parameter {
    fn key(&self) -> Option<Identifier> {
        Some(self.id.clone())
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(t), Some(v)) => write!(f, "{}: {t} = {v}", self.id),
            (Some(t), None) => write!(f, "{}: {t}", self.id),
            (None, Some(v)) => write!(f, "{} = {v}", self.id),
            _ => Ok(()),
        }
    }
}
