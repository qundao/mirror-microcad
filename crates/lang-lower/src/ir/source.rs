// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function definition syntax element

use super::ir;

use serde::{Deserialize, Serialize};

/// Items of a source file that will become symbols.
#[derive(Debug, Clone, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct SourceItems<Path: ir::PathSpec = ir::Path> {
    /// List of file modules: `mod foo;`.
    pub file_modules: Box<[ir::FileModule<Path>]>,
    /// Inline modules: `mod bar {...}`.
    pub inline_modules: Box<[ir::InlineModule<Path>]>,
    /// Use statements: `use ...`.
    pub aliases: ir::Aliases<Path>,
    /// Constants: `const FOO = 42;`.
    pub constants: Box<[ir::Constant<Path>]>,
    /// Functions: `fn foo(...) {...}`.
    pub functions: Box<[ir::Function<Path>]>,
    /// Workbenches: `part Bar(...) {...}`.
    pub workbenches: Box<[ir::Workbench<Path>]>,
}

/// IR of a µcad source file
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Source<Path: ir::PathSpec = ir::Path> {
    /// The module id of the source file, extracted from [`Source`].
    pub id: Option<microcad_lang_base::Name>,
    /// Inner attributes.
    pub attr: ir::InnerAttributes<Path>,
    /// Items that will become Symbols
    pub items: ir::SourceItems<Path>,
    /// Workbench statements
    pub statements: Box<[ir::WorkbenchStatement<Path>]>,
}
