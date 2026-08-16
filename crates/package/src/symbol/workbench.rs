// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Identifier, Refer};
use microcad_lang_lower::ir;
use serde::{Deserialize, Serialize};

pub type WorkbenchExpression = ir::WorkbenchExpression;
pub type WorkbenchStatement = ir::WorkbenchStatement;
pub type Init = ir::Init;
pub type WorkbenchKind = microcad_lang_base::element::WorkbenchKind;

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Workbench {
    //pub attr: WorkbenchAttributes
    /// Workbench kind.
    pub kind: WorkbenchKind,
    /// Workbench's building plan.
    pub parameters: ir::ParameterList,

    /// Initializers.
    /// The default initializer and tthe workbench parameters are located in the last init.
    pub inits: Box<[Init]>,

    pub statements: Box<[WorkbenchStatement]>,
}
