// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use super::ir;

use serde::{Deserialize, Serialize};

/// Items of a source file that will become symbols.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct SourceItems<Name: ir::NameSpec = ir::Name> {
    /// List of file modules: `mod foo;`.
    pub file_modules: Box<[ir::FileModule<Name>]>,
    /// Inline modules: `mod bar {...}`.
    pub inline_modules: Box<[ir::InlineModule<Name>]>,
    /// Use statements: `use ...`.
    pub aliases: ir::Aliases<Name>,
    /// Constants: `const FOO = 42;`.
    pub constants: Box<[ir::Constant<Name>]>,
    /// Functions: `fn foo(...) {...}`.
    pub functions: Box<[ir::Function<Name>]>,
    /// Workbenches: `part Bar(...) {...}`.
    pub workbenches: Box<[ir::Workbench<Name>]>,
}

/// IR of a µcad source file
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Source<Name: ir::NameSpec = ir::Name> {
    /// The module id of the source file, extracted from [`Source`].
    pub id: Option<microcad_lang_base::Name>,
    /// Inner attributes.
    pub attr: ir::InnerAttributes<Name>,
    /// Items that will become Symbols
    pub items: ir::SourceItems<Name>,
    /// Workbench statements
    pub statements: Box<[ir::WorkbenchStatement<Name>]>,
}
