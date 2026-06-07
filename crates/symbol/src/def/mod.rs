// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod attribute;
mod expression;
mod function;
mod source;
mod workbench;

pub use function::*;
use microcad_lang_base::{HashId, HashMap, SrcRef};
use microcad_lang_types::Value;
pub use source::Source;
pub use workbench::*;

pub use function::Function;

use crate::Symbol;

/// Resolved from `use foo::bar::*`
#[derive(Debug)]
pub struct Glob(Symbol);

#[derive(Debug)]
pub struct Constant {
    value: Value,
    src_ref: SrcRef,
    keyword_src_ref: SrcRef,
}

/// Symbol definition
#[derive(Debug, Default)]
pub enum SymbolDef {
    /// An empty definition, used during building the symbol.
    #[default]
    Empty,

    /// A library symbol, containing a manifest file and a `lib.µcad` source file.
    Library,
    /// Source file symbol.
    Source,
    /// Inline Module symbol: `mod foo {}`
    InlineModule,
    /// File Module Symbol: `mod foo;`

    /// Workbench symbol.
    Workbench,
    /// Function symbol.
    Function,
    /// Assignment.
    Constant,
    /// Builtin symbol.
    Builtin,
    /// Alias of a pub use statement.
    Alias,
    /// Use all available symbols in the module with the given name.
    Glob,
}

impl std::fmt::Display for SymbolDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                SymbolDef::Empty => "<empty>",
                SymbolDef::Library => "lib",
                SymbolDef::Source => "source",
                SymbolDef::InlineModule => "mod",
                SymbolDef::Workbench => "workbench",
                SymbolDef::Function => "fn",
                SymbolDef::Constant => "const",
                SymbolDef::Builtin => "builtin",
                SymbolDef::Alias => "alias",
                SymbolDef::Glob => "glob",
            }
        )
    }
}
