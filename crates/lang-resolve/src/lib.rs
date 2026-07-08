// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod resolve;

pub mod symbol;

use microcad_lang_base::{Artifact, CompilationResult, Source};
use microcad_lang_lower::Ir;

pub use symbol::{Symbol, SymbolRef, SymbolTree};

pub use resolve::{Resolve, ResolveContext, ResolveResult};

impl Artifact for SymbolTree {
    fn kind() -> microcad_lang_base::ArtifactKind {
        microcad_lang_base::ArtifactKind::SymbolTree
    }
}

pub fn resolve(source: &Source, ir: &Ir) -> CompilationResult<SymbolTree> {
    todo!()
}
