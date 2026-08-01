// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Identifier, Refer};
use microcad_lang_lower::ir;
use microcad_lang_types::Type;
use serde::{Deserialize, Serialize};

use crate::rst::{ParameterList, ResolvedName};

pub type WorkbenchExpression = ir::WorkbenchExpression<ResolvedName>;
pub type WorkbenchStatement = ir::WorkbenchStatement<ResolvedName>;
pub type WorkbenchKind = microcad_lang_base::element::WorkbenchKind;

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]

pub struct InitStatement {
    pub id: Identifier,
    pub ty: Type,
    pub expression: WorkbenchExpression,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Init {
    pub parameters: ParameterList,
    pub statements: Box<[InitStatement]>,
}

#[derive(Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Workbench {
    //pub attr: WorkbenchAttributes
    /// Workbench kind.
    pub kind: Refer<WorkbenchKind>,

    /// Initializers.
    /// The default initializer and tthe workbench parameters are located in the last init.
    pub inits: Box<[Init]>,

    pub statements: Box<[WorkbenchStatement]>,
}
