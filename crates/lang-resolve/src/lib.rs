// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod resolve;

pub mod rst;

use microcad_lang_base::{CompilationResult, Source};
use microcad_lang_lower::Ir;

pub use rst::{Rst, Symbol, SymbolRef};

pub use resolve::{Resolve, ResolveContext, ResolveResult};

pub fn resolve(_source: &Source, ir: &Ir) -> CompilationResult<Rst> {
    let mut context = ResolveContext::new(ir);

    for inline_module in &ir.items.inline_modules {
        context.builder.add(rst::def::inline_module(inline_module));
        // let node = inline_module.resolve(&mut context);
        //   builder.add(node);
    }

    Ok((context.builder.build_rst(), context.diagnostics))
}
