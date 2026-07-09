// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod resolve;

pub mod rst;

use microcad_lang_base::{CompilationResult, Identifier, Source, SrcRef};
use microcad_lang_lower::{Ir, ir::Visibility};

pub use rst::{Rst, Symbol, SymbolRef};

pub use resolve::{Resolve, ResolveContext, ResolveResult};

use crate::rst::SymbolAttributes;

pub fn resolve(source: &Source, ir: &Ir) -> CompilationResult<Rst> {
    let mut builder = rst::Builder::new(rst::SymbolData {
        id: Identifier::from("root"),
        attr: SymbolAttributes::default(),
        def: rst::def::SymbolDef::SourceFile(rst::def::SourceFile {}),
        visibility: Visibility::Public,
        src_ref: SrcRef::none(),
        keyword_ref: SrcRef::none(),
    });

    let mut context = ResolveContext::new();

    for inline_module in &ir.items.inline_modules {
        // let node = inline_module.resolve(&mut context);
        //   builder.add(node);
    }

    Ok((builder.build(), context.diagnostics))
}
