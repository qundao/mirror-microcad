// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use super::ir;

use microcad_lang_base::{IsDefault, is_default};
use serde::{Deserialize, Serialize};

/// Items of a source file that will become symbols.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct SourceItems {
    /// List of file modules: `mod foo;`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub file_modules: Box<[ir::FileModule]>,
    /// Inline modules: `mod bar {...}`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub inline_modules: Box<[ir::InlineModule]>,
    /// Use statements: `use ...`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub aliases: ir::Aliases,
    /// Constants: `const FOO = 42;`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub constants: Box<[ir::Constant]>,
    /// Functions: `fn foo(...) {...}`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub functions: Box<[ir::Function]>,
    /// Workbenches: `part Bar(...) {...}`.
    #[serde(skip_serializing_if = "is_default", default)]
    pub workbenches: Box<[ir::Workbench]>,
}

impl IsDefault for SourceItems {
    fn is_default(&self) -> bool {
        self.file_modules.is_default()
            && self.inline_modules.is_default()
            && self.aliases.is_default()
            && self.constants.is_default()
            && self.functions.is_default()
            && self.workbenches.is_default()
    }
}

/// IR of a µcad source file
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// Inner attributes.
    pub attr: ir::InnerAttributes,
    /// Items that will become Symbols
    #[serde(skip_serializing_if = "is_default", default)]
    pub items: ir::SourceItems,
    /// Workbench statements
    #[serde(skip_serializing_if = "is_default", default)]
    pub statements: Box<[ir::WorkbenchStatement]>,
}
