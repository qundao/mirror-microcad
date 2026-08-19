// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin::BuiltinError;
use microcad_lang_base::{Identifier, Refer, SrcRef, SrcReferrer};
use microcad_lang_parse::ast;
use microcad_lang_types::{TypeError, ValueError};
use miette::Diagnostic;
use thiserror::Error;

use crate::ir;

/// Errors and warnings during lowering
#[derive(Debug, Error, Diagnostic)]
#[allow(missing_docs)]
pub enum LowerError {
    #[error("Error parsing integer literal: {0}")]
    ParseIntError(#[label("{0}")] Refer<std::num::ParseIntError>),

    #[error("Value error: {0}")]
    ValueError(#[from] ValueError),

    #[error("Built-in error: {0}")]
    BuiltinError(#[from] BuiltinError),

    #[error("Unknown unit: {0}")]
    UnknownUnit(#[label("Unknown unit")] Refer<String>),

    #[error("Duplicate argument: {id}")]
    DuplicateArgument {
        #[label(primary, "Duplicate argument")]
        id: Identifier,
        #[label("Previous declaration")]
        previous: Identifier,
    },

    #[error("Inner doc comments must appear before inner attributes")]
    #[diagnostic(
        code(lower::inner_doc_after_inner_attr),
        help("Move this doc comment to the top of the block")
    )]
    InnerDocAfterInnerAttribute {
        #[label("this doc comment is out of order")]
        src_ref: SrcRef,
    },

    #[error("Inner attributes must appear before statements")]
    #[diagnostic(
        code(lower::inner_attr_after_stmt),
        help("Move this attribute above the first statement")
    )]
    InnerAttributeAfterStatement {
        #[label("this attribute is out of order")]
        src_ref: SrcRef,
    },
    /// Statements that are truly forbidden in a block
    #[error("This statement is not allowed here")]
    #[diagnostic(code(lower::unexpected_statement))]
    StatementNotAllowed {
        #[label("unexpected statement")]
        src_ref: SrcRef,
    },

    /// Grammar rule error
    #[error("Invalid id '{0}'")]
    InvalidIdentifier(Refer<String>),

    #[error("Unknown type: {0}")]
    UnknownType(#[label("Unknown type")] Refer<String>),

    /// A Type error
    #[error("Type error: {0}")]
    TypeError(#[from] Refer<TypeError>),

    /// Invalid glob pattern
    #[error("Invalid glob pattern, wildcard must be at the end of the pattern")]
    InvalidGlobPattern(SrcRef),

    /// A glob import is given an alias
    #[error("Glob imports can't be given an alias")]
    UseGlobAlias(SrcRef),

    /// A parser from the AST builder
    #[error(transparent)]
    #[diagnostic(transparent)]
    AstParser(Refer<microcad_lang_parse::ParseError>),

    /// An invalid literal was encountered
    #[error("Invalid literal: {error}")]
    InvalidLiteral {
        error: ast::LiteralErrorKind,
        #[label("{error}")]
        src_ref: SrcRef,
    },

    /// An invalid expression was encountered
    #[error("Invalid expression")]
    InvalidExpression { src_ref: SrcRef },

    /// An invalid statement was encountered
    #[error("Statement is not allowed in this context")]
    #[diagnostic(code("UnexpectedStatement"))]
    UnexpectedStatement { src_ref: SrcRef },

    /// A type range between non-integer literals
    #[error("range expressions must be between integers")]
    InvalidRangeType { src_ref: SrcRef },

    /// Implicit returns in tail expressions are treated as regular statements inside workbenches
    #[error("Ignored implicit return in workbench")]
    #[diagnostic(help("Add a trailing semicolon to remove the implicit return"))]
    ImplicitWorkbenchReturn {
        #[label("Workbenches don't return any value")]
        src_ref: SrcRef,
    },

    #[error("Statement is unreachable")]
    #[diagnostic(help("Remove this statement {src_ref}"), severity = "Warning")]
    Unreachable {
        #[label("Last statement to be evaluated")]
        last_ref: SrcRef,
        #[label("Statement")]
        src_ref: SrcRef,
    },

    #[error("This is not a constant expression")]
    InvalidConstantExpression {
        #[label("Expression")]
        src_ref: SrcRef,
    },

    #[error("The result of the function statement is ignored")]
    FunctionStatementIgnored(#[label("Removed this statement")] SrcRef),

    #[error("Unsupported command attribute: {path}")]
    UnsupportedCommandAttribute {
        path: ir::Path,
        #[label("Command attribute")]
        src_ref: SrcRef,
    },

    #[error("Unsupported key-value attribute: {key}")]
    UnsupportedKeyValueAttribute {
        key: ir::Path,
        #[label("Command attribute")]
        src_ref: SrcRef,
    },

    #[error("Unsupported tag attribute: {tag}")]
    UnsupportedTagAttribute {
        #[label("Tag attribute")]
        tag: ir::Identifier,
    },
}

/// Result with lower error
pub type LowerResult<T> = Result<T, LowerError>;

impl SrcReferrer for LowerError {
    fn src_ref(&self) -> SrcRef {
        match self {
            LowerError::ValueError(_) | LowerError::BuiltinError(_) => SrcRef::none(),
            LowerError::DuplicateArgument { id, .. } => id.src_ref(),
            LowerError::StatementNotAllowed { src_ref }
            | LowerError::InvalidGlobPattern(src_ref)
            | LowerError::UseGlobAlias(src_ref)
            | LowerError::InvalidLiteral { src_ref, .. }
            | LowerError::InvalidExpression { src_ref }
            | LowerError::UnexpectedStatement { src_ref }
            | LowerError::InvalidRangeType { src_ref }
            | LowerError::ImplicitWorkbenchReturn { src_ref } => *src_ref,
            LowerError::ParseIntError(parse_int_error) => parse_int_error.src_ref(),
            LowerError::InvalidIdentifier(id) => id.src_ref(),
            LowerError::UnknownUnit(unit) => unit.src_ref(),
            LowerError::UnknownType(ty) => ty.src_ref(),
            LowerError::TypeError(ty) => ty.src_ref(),
            LowerError::AstParser(err) => err.src_ref(),
            LowerError::Unreachable { src_ref, .. } => *src_ref,
            LowerError::InvalidConstantExpression { src_ref } => *src_ref,
            LowerError::InnerDocAfterInnerAttribute { src_ref } => *src_ref,
            LowerError::InnerAttributeAfterStatement { src_ref } => *src_ref,
            LowerError::FunctionStatementIgnored(src_ref) => *src_ref,
            LowerError::UnsupportedCommandAttribute { src_ref, .. } => *src_ref,
            LowerError::UnsupportedKeyValueAttribute { src_ref, .. } => *src_ref,
            LowerError::UnsupportedTagAttribute { tag } => tag.src_ref(),
        }
    }
}
