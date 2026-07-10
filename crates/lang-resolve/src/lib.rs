// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod resolve;

pub mod rst;

use microcad_lang_base::{CompilationResult, Id, Identifier, Source, SrcRef};
use microcad_lang_lower::{
    Ir,
    ir::{DocBlock, Visibility},
};

pub use rst::{Rst, Symbol, SymbolRef};

pub use resolve::{Resolve, ResolveContext, ResolveResult};

use crate::rst::{SymbolAttributes, UnresolvedName};

pub fn resolve(source: &Source, ir: &Ir) -> CompilationResult<Rst> {
    let mut context = ResolveContext::new(ir);

    for inline_module in &ir.items.inline_modules {
        // let node = inline_module.resolve(&mut context);
        //   builder.add(node);
    }

    Ok((context.builder.build(), context.diagnostics))
}
