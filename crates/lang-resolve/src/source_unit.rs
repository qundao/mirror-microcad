// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A source unit that can be compiled into an IR.

use microcad_lang_base::{Source, StageResult};
use microcad_lang_lower::{Ir, LowerContext, LowerError};
use microcad_lang_parse::{Ast, ParseError};

use crate::{ResolveResult, locate};

/// A µcad source file document.
pub struct SourceUnit {
    pub source: Source,
    pub ast: StageResult<Ast, ParseError>,
    pub ir: StageResult<Ir, LowerError>,
    //pub model: Option<mu::CompilationResult<mu::Model>>,
}

impl SourceUnit {
    /// New source file from a source.
    pub fn new(source: Source) -> Self {
        Self {
            source,
            ast: StageResult::default(),
            ir: StageResult::default(),
        }
    }

    /// Load a source unit from file.
    pub fn load(path: impl AsRef<std::path::Path>) -> ResolveResult<Self> {
        let source = Source::load(locate::resolved_path(path)?)?;

        let ast = StageResult::from(microcad_lang_parse::parse(&source));

        match ast.artifact() {
            Some(ast_artifact) => {
                let mut lower_context = LowerContext::new(&source);
                let ir =
                    StageResult::from(microcad_lang_lower::lower(&mut lower_context, ast_artifact));
                Ok(Self { source, ast, ir })
            }
            None => Ok(Self {
                source,
                ast,
                ir: StageResult::default(),
            }),
        }
    }

    pub fn is_ok(&self) -> bool {
        self.ast.is_ok() && self.ir.is_ok()
    }
}
