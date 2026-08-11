// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use super::ir;

use microcad_lang_base::Name;
use serde::{Deserialize, Serialize};

/// Items of a source file that will become symbols.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct SourceItems {
    /// List of file modules: `mod foo;`.
    pub file_modules: Box<[ir::FileModule]>,
    /// Inline modules: `mod bar {...}`.
    pub inline_modules: Box<[ir::InlineModule]>,
    /// Use statements: `use ...`.
    pub aliases: ir::Aliases,
    /// Constants: `const FOO = 42;`.
    pub constants: Box<[ir::Constant]>,
    /// Functions: `fn foo(...) {...}`.
    pub functions: Box<[ir::Function]>,
    /// Workbenches: `part Bar(...) {...}`.
    pub workbenches: Box<[ir::Workbench]>,
}

/// IR of a µcad source file
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// The module id of the source file, extracted from [`Source`].
    pub id: Option<Name>,
    /// Inner attributes.
    pub attr: ir::InnerAttributes,
    /// Items that will become Symbols
    pub items: ir::SourceItems,
    /// Workbench statements
    pub statements: Box<[ir::WorkbenchStatement]>,
}
