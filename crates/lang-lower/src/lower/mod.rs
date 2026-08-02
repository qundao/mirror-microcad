// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lowering the AST.

mod attribute;
mod constant;
mod expression;
mod function;
mod lang_type;
mod module;
mod parameter;
mod source;
mod r#type;
mod workbench;

use microcad_lang_base::{
    Identifiable, Identifier, Refer, SpanToSrcRef, Spanned, SrcRef, SrcReferrer,
};
use microcad_lang_parse::ast;
use microcad_lang_types::ty::TypeError;
use miette::Diagnostic;
use thiserror::Error;

use crate::{Lower, LowerContext, ir};

/// Errors and warnings during lowering
#[derive(Debug, Error, Diagnostic)]
#[allow(missing_docs)]
pub enum LowerError {
    #[error("Error parsing integer literal: {0}")]
    ParseIntError(#[label("{0}")] Refer<std::num::ParseIntError>),

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
}

/// Result with lower error
pub type LowerResult<T> = Result<T, LowerError>;

impl SrcReferrer for LowerError {
    fn src_ref(&self) -> SrcRef {
        match self {
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
        }
    }
}

/// Extracts and maps specific variants out of a statement collection tuple list.
///
/// Does not check if the statements are actually valid in this context.
pub fn extract_statements_with_tail<F, G, T>(
    statements: &ast::StatementList,
    context: &mut LowerContext,
    mut extractor: F,
    mut tail_extractor: G,
) -> LowerResult<Box<[T]>>
where
    F: FnMut(&ast::Statement, &mut LowerContext) -> LowerResult<Option<T>>,
    G: FnMut(&ast::ExpressionStatement, &mut LowerContext) -> LowerResult<Option<T>>,
{
    let mut mapped = Vec::new();
    statements
        .statements
        .iter()
        .map(|(stmt, _)| stmt)
        .try_for_each(|stmt| -> Result<(), LowerError> {
            mapped.extend(extractor(stmt, context)?);
            Ok(())
        })?;

    if let Some(tail) = &statements.tail {
        mapped.extend(tail_extractor(tail, context)?);
    }
    Ok(mapped.into_boxed_slice())
}

/// Extracts and maps specific variants out of a statement collection tuple list.
///
/// Does not check if the statements are actually valid in this context.
pub fn extract_statements<F, T>(
    statements: &ast::StatementList,
    mut extractor: F,
) -> LowerResult<Box<[T]>>
where
    F: FnMut(&ast::Statement) -> LowerResult<Option<T>>,
{
    let mut mapped = Vec::new();
    statements
        .statements
        .iter()
        .map(|(stmt, _)| stmt)
        .try_for_each(|stmt| -> LowerResult<()> {
            if let Some(m) = extractor(stmt)? {
                mapped.push(m);
            }
            Ok(())
        })?;

    Ok(mapped.into_boxed_slice())
}

pub fn for_each_statement<F>(
    statements: &ast::StatementList,
    context: &mut LowerContext,
    mut check: F,
) -> LowerResult<()>
where
    F: FnMut(&ast::Statement, &mut LowerContext) -> LowerResult<()>,
{
    statements
        .statements
        .iter()
        .map(|(stmt, _)| stmt)
        .try_for_each(|stmt| check(stmt, context))
}

/// Named and check for duplicates
pub fn sort_and_check<T>(mut named: Vec<T>, context: &mut LowerContext) -> LowerResult<Box<[T]>>
where
    T: Identifiable + SrcReferrer,
{
    named.sort_by_key(|lhs| lhs.id());
    named
        .windows(2)
        .filter_map(|pair| {
            if pair[0].id() == pair[1].id() {
                Some((&pair[0], &pair[1]))
            } else {
                None
            }
        })
        .try_for_each(|(prev_arg, arg)| -> LowerResult<()> {
            context.diag(LowerError::DuplicateArgument {
                id: arg.id().clone(),
                previous: prev_arg.id().clone(),
            });
            Ok(())
        })?;

    Ok(named.into_boxed_slice())
}

impl<T> Lower<ast::StatementList> for Box<[T]>
where
    Option<T>: Lower<ast::Statement>,
    Option<T>: Lower<ast::ExpressionStatement>,
{
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        extract_statements_with_tail(node, context, Option::lower, Option::lower)
    }
}

impl Lower<Option<Spanned<ast::def::Visibility>>> for ir::Visibility {
    fn lower(
        node: &Option<Spanned<ast::def::Visibility>>,
        _context: &mut LowerContext,
    ) -> LowerResult<Self> {
        Ok(match node.as_ref().map(|v| &v.value) {
            Some(ast::def::Visibility::Public) => Self::Public,
            None => Self::Private,
        })
    }
}

impl Lower<ast::Identifier> for ir::Identifier {
    fn lower(node: &ast::Identifier, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(Refer::new(
            node.name.clone(),
            context.span_to_src_ref(&node.span),
        )))
    }
}

impl Lower<ast::def::UseName> for ir::SymbolPath {
    fn lower(node: &ast::def::UseName, context: &mut LowerContext) -> LowerResult<Self> {
        let prefix = node
            .prefix
            .as_ref()
            .map(|span| context.span_to_src_ref(span));
        let parts = node
            .parts
            .iter()
            .filter_map(|part| match part {
                ast::def::UseStatementPart::Identifier(ident) => {
                    Some(ir::Identifier::lower(ident, context))
                }
                ast::def::UseStatementPart::Glob(_) => None,
                ast::def::UseStatementPart::Error(_) => None,
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();

        Ok(Self {
            prefix,
            parts,
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl<EXPR> Lower<ast::LocalAssignment> for ir::LocalAssignment<EXPR>
where
    EXPR: Lower<ast::Expression>,
{
    fn lower(node: &ast::LocalAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            id: ir::Identifier::lower(&node.id, context)?,
            specified_type: Option::<ir::TypeAnnotation>::lower(&node.ty, context)?,
            expression: EXPR::lower(node.expr.as_ref(), context)?,
            src_ref: context.span_to_src_ref(&node.span),
        })
    }
}

impl Lower<ast::StatementList> for ir::Aliases {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            explicit_aliases: extract_statements(node, |stmt| match stmt {
                ast::Statement::Use(use_statement) => match use_statement.name.parts.last() {
                    Some(ast::def::UseStatementPart::Identifier(id)) => {
                        Ok(Some(ir::ExplicitAlias {
                            attr: ir::OuterAttributes::lower(&use_statement.attr, context)?,
                            keyword_src_ref: context.span_to_src_ref(&use_statement.keyword_span),
                            visibility: ir::Visibility::lower(&use_statement.vis, context)?,
                            path: ir::SymbolPath::lower(&use_statement.name, context)?,
                            id: ir::Identifier::lower(
                                match &use_statement.use_as {
                                    // Use id `C` from `as C`
                                    Some(id) => id,
                                    // Use id `Circle` from last part of path `std::geo2d::Circle`
                                    None => id,
                                },
                                context,
                            )?,
                            src_ref: context.span_to_src_ref(&use_statement.span),
                        }))
                    }
                    None => unreachable!(),
                    Some(_) => Ok(None),
                },
                _ => Ok(None),
            })?,
            wildcards: extract_statements(node, |stmt| match stmt {
                ast::Statement::Use(use_statement) => match use_statement.name.parts.last() {
                    Some(ast::def::UseStatementPart::Glob(_)) => Ok(Some(ir::WildcardAlias {
                        attr: ir::OuterAttributes::lower(&use_statement.attr, context)?,
                        keyword_src_ref: context.span_to_src_ref(&use_statement.keyword_span),
                        visibility: ir::Visibility::lower(&use_statement.vis, context)?,
                        path: ir::SymbolPath::lower(&use_statement.name, context)?,
                        src_ref: context.span_to_src_ref(&use_statement.span),
                    })),
                    None => unreachable!(),
                    Some(_) => Ok(None),
                },
                _ => Ok(None),
            })?,
        })
    }
}
