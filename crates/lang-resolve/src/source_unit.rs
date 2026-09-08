// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A source unit that can be compiled into an IR.

use microcad_lang_base::{Source, StageResult};
use microcad_lang_lower::{Ir, LowerContext, LowerError};
use microcad_lang_parse::{Ast, ParseError};

use crate::{ResolveResult, locate};
/// A µcad source file document progressing through compiler pipeline stages.
#[derive(Debug)]
pub enum SourceUnit {
    Loaded(Source),
    Parsed {
        source: Source,
        ast: StageResult<Ast, ParseError>,
    },
    Lowered {
        source: Source,
        ast: StageResult<Ast, ParseError>,
        ir: StageResult<Ir, LowerError>,
    },
}

impl SourceUnit {
    /// Create a new loaded source file.
    pub fn new(source: Source) -> Self {
        Self::Loaded(source)
    }

    /// Load a source unit from disk and immediately run full compilation through lowering.
    pub fn load(path: impl AsRef<std::path::Path>) -> ResolveResult<Self> {
        let source = Source::load(locate::resolved_path(path)?)?;
        Ok(Self::new(source).parse().lower())
    }

    /// Progress from `Loaded` to `Parsed` stage by running the parser.
    pub fn parse(self) -> Self {
        match self {
            Self::Loaded(source) => {
                let ast = StageResult::from(microcad_lang_parse::parse(&source));
                Self::Parsed { source, ast }
            }
            already_parsed => already_parsed,
        }
    }

    /// Progress from `Parsed` to `Lowered` stage by lowering the AST to IR.
    pub fn lower(self) -> Self {
        match self {
            Self::Parsed { source, ast } => {
                let ir = match ast.artifact() {
                    Some(ast_artifact) => {
                        let mut lower_context = LowerContext::new(&source);
                        StageResult::from(microcad_lang_lower::lower(
                            &mut lower_context,
                            ast_artifact,
                        ))
                    }
                    None => StageResult::default(),
                };
                Self::Lowered { source, ast, ir }
            }
            Self::Loaded(source) => Self::Loaded(source).parse().lower(),
            already_lowered => already_lowered,
        }
    }

    /// Access the underlying `Source` regardless of stage.
    pub fn source(&self) -> &Source {
        match self {
            Self::Loaded(source) | Self::Parsed { source, .. } | Self::Lowered { source, .. } => {
                source
            }
        }
    }

    /// Returns `true` if all executed stages succeeded without fatal errors.
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Loaded(_) => true,
            Self::Parsed { ast, .. } => ast.is_ok(),
            Self::Lowered { ast, ir, .. } => ast.is_ok() && ir.is_ok(),
        }
    }

    /// Consumes the `SourceUnit` and yields the `Ir` if available.
    pub fn into_ir(self) -> Option<Ir> {
        match self {
            Self::Lowered { ir, .. } => ir.into_artifact(),
            _ => None,
        }
    }
}

impl std::fmt::Display for SourceUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Self::Loaded(_) => "LOADED",
            Self::Parsed { ast, .. } if ast.is_ok() => "AST OK",
            Self::Parsed { .. } => "AST ERR",
            Self::Lowered { ast, ir, .. } if ast.is_ok() && ir.is_ok() => "IR OK",
            Self::Lowered { .. } => "IR ERR",
        };
        write!(f, "{} [{status}]", self.source())
    }
}
