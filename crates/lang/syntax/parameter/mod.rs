// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter syntax elements

mod parameter_list;

use crate::{ord_map::*, syntax::*};

use microcad_lang_base::{SrcRef, TreeDisplay, TreeState};
use microcad_lang_proc_macros::{Identifiable, SrcReferrer};
pub use parameter_list::*;

/// A parameter of a parameter list.
#[derive(Clone, Debug, Default, SrcReferrer, Identifiable)]
pub struct Parameter {
    /// Name of the parameter
    pub(crate) id: Identifier,
    /// Type of the parameter or `None`
    pub specified_type: Option<TypeAnnotation>,
    /// default value of the parameter or `None`
    pub default_value: Option<Expression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl OrdMapValue<Identifier> for Parameter {
    fn key(&self) -> Option<Identifier> {
        Some(self.id())
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

impl TreeDisplay for Parameter {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(specified_type), Some(default_value)) => writeln!(
                f,
                "{:depth$}Parameter: {}: {} = {}",
                "", self.id, specified_type, default_value
            ),
            (Some(specified_type), None) => {
                writeln!(f, "{:depth$}Parameter: {}: {}", "", self.id, specified_type)
            }
            (None, Some(default_value)) => {
                writeln!(f, "{:depth$}Parameter: {} = {}", "", self.id, default_value)
            }
            _ => unreachable!("impossible parameter declaration"),
        }
    }
}
