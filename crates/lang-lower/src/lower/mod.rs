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

use microcad_lang_base::{Hashed, Identifier, Refer, SrcRef, SrcReferrer};
use microcad_lang_parse::ast;
use microcad_lang_parse::parse;
use microcad_lang_types::ty::TypeError;
use miette::{Diagnostic, SourceCode};
use thiserror::Error;

use crate::Identifiable;
use crate::{Lower, LowerContext, ir};

/// Parsing errors
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

    #[error("Loading of source file {1:?} failed: {2}")]
    LoadSource(SrcRef, std::path::PathBuf, std::io::Error),

    /// Grammar rule error
    #[error("Invalid id '{0}'")]
    InvalidIdentifier(Refer<String>),

    #[error("Unknown type: {0}")]
    UnknownType(#[label("Unknown type")] Refer<String>),

    /// Matrix type with invalid dimensions
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

    /// An invalid state was encountered
    #[error("Invalid statement")]
    InvalidStatement { src_ref: SrcRef },

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
}

/// Result with parse error
pub type LowerResult<T> = Result<T, LowerError>;

impl SrcReferrer for LowerError {
    fn src_ref(&self) -> SrcRef {
        match self {
            LowerError::DuplicateArgument { id, .. } => id.src_ref(),
            LowerError::LoadSource(src_ref, ..)
            | LowerError::InvalidGlobPattern(src_ref)
            | LowerError::UseGlobAlias(src_ref)
            | LowerError::InvalidLiteral { src_ref, .. }
            | LowerError::InvalidExpression { src_ref }
            | LowerError::InvalidStatement { src_ref }
            | LowerError::InvalidRangeType { src_ref }
            | LowerError::ImplicitWorkbenchReturn { src_ref } => src_ref.clone(),
            LowerError::ParseIntError(parse_int_error) => parse_int_error.src_ref(),
            LowerError::InvalidIdentifier(id) => id.src_ref(),
            LowerError::UnknownUnit(unit) => unit.src_ref(),
            LowerError::UnknownType(ty) => ty.src_ref(),
            LowerError::TypeError(ty) => ty.src_ref(),
            LowerError::AstParser(err) => err.src_ref(),
        }
    }
}

/// Parse error, possibly with source code
#[derive(Debug, Error)]
#[error("Failed to parse")] // todo
pub struct LowerErrorsWithSource {
    /// The errors encountered during parsing
    pub errors: Vec<LowerError>,
    /// The parsed source code
    pub source_code: Option<Hashed<String>>,
}

impl From<LowerError> for LowerErrorsWithSource {
    fn from(value: LowerError) -> Self {
        LowerErrorsWithSource {
            errors: vec![value],
            source_code: None,
        }
    }
}

impl From<Vec<LowerError>> for LowerErrorsWithSource {
    fn from(value: Vec<LowerError>) -> Self {
        LowerErrorsWithSource {
            errors: value,
            source_code: None,
        }
    }
}

impl Diagnostic for LowerErrorsWithSource {
    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.source_code
            .as_ref()
            .map(|source| source.value() as &dyn SourceCode)
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        Some(Box::new(
            self.errors.iter().map(|e| -> &dyn Diagnostic { e }),
        ))
    }
}

impl SrcReferrer for LowerErrorsWithSource {
    fn src_ref(&self) -> SrcRef {
        self.errors[0].src_ref()
    }
}

pub(crate) fn build_ast(
    source: &str,
    lower_context: &mut LowerContext,
) -> Result<ast::Program, LowerErrorsWithSource> {
    parse(source).map_err(|errors| {
        let errors = errors
            .0
            .into_iter()
            .map(|error| {
                let src_ref = lower_context.src_ref(&error.span);
                LowerError::AstParser(Refer::new(error, src_ref))
            })
            .collect::<Vec<_>>();
        LowerErrorsWithSource {
            errors,
            source_code: Some(
                lower_context
                    .source
                    .clone()
                    .map(|source| source.to_string()),
            ),
        }
    })
}

/// Extracts and maps specific variants out of a statement collection tuple list.
///
/// Does not check if the statements are actually valid in this context.
pub fn extract_statements_with_tail<F, G, T>(
    statements: &ast::StatementList,
    mut extractor: F,
    mut tail_extractor: G,
) -> LowerResult<Box<[T]>>
where
    F: FnMut(&ast::Statement) -> LowerResult<Option<T>>,
    G: FnMut(&ast::ExpressionStatement) -> LowerResult<Option<T>>,
{
    let mut mapped = Vec::new();
    statements
        .statements
        .iter()
        .map(|(stmt, _)| stmt)
        .try_for_each(|stmt| -> Result<(), LowerError> {
            match extractor(stmt)? {
                Some(m) => mapped.push(m),
                None => {}
            }
            Ok(())
        })?;

    match &statements.tail {
        Some(tail) => match tail_extractor(tail)? {
            Some(m) => mapped.push(m),
            None => {}
        },
        None => {}
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
        .try_for_each(|stmt| -> Result<(), LowerError> {
            match extractor(stmt)? {
                Some(m) => mapped.push(m),
                None => {}
            }
            Ok(())
        })?;

    Ok(mapped.into_boxed_slice())
}

/// Named and check for duplicates
pub fn sort_and_check<T>(mut named: Vec<T>, context: &mut LowerContext) -> LowerResult<Box<[T]>>
where
    T: Identifiable + SrcReferrer,
{
    use microcad_lang_base::PushDiag;
    named.sort_by(|lhs, rhs| lhs.id().cmp(&rhs.id()));

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
            context
                .diagnostics
                .error(
                    &arg.src_ref(),
                    LowerError::DuplicateArgument {
                        id: arg.id().clone(),
                        previous: prev_arg.id().clone(),
                    },
                )
                .ok();
            Ok(())
        })?;

    Ok(named.into_boxed_slice())
}

impl Lower<Option<ast::Visibility>> for ir::Visibility {
    fn lower(node: &Option<ast::Visibility>, _context: &mut LowerContext) -> LowerResult<Self> {
        Ok(match node {
            Some(ast::Visibility::Public) => Self::Public,
            None => Self::Private,
        })
    }
}

impl Lower<ast::Identifier> for ir::Identifier {
    fn lower(node: &ast::Identifier, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(Refer::new(
            node.name.clone(),
            context.src_ref(&node.span),
        )))
    }
}

impl Lower<ast::UseName> for ir::QualifiedName {
    fn lower(node: &ast::UseName, context: &mut LowerContext) -> LowerResult<Self> {
        let name = node
            .parts
            .iter()
            .filter_map(|part| match part {
                ast::UseStatementPart::Identifier(ident) => {
                    Some(ir::Identifier::lower(ident, context))
                }
                ast::UseStatementPart::Glob(_) => None,
                ast::UseStatementPart::Error(_) => None,
            })
            .collect::<Result<Vec<_>, _>>()?;

        let name = ir::QualifiedName::new(name, context.src_ref(&node.span));
        Ok(name)
    }
}

impl Lower<ast::StatementList> for ir::Aliases {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            explicit_aliases: extract_statements(node, |stmt| match stmt {
                ast::Statement::Use(use_statement) => match use_statement.name.parts.last() {
                    Some(ast::UseStatementPart::Identifier(id)) => Ok(Some(ir::ExplicitAlias {
                        attr: ir::Attributes::lower(&use_statement.attributes, context)?,
                        keyword_src_ref: context.src_ref(&use_statement.keyword_span),
                        visibility: ir::Visibility::lower(&use_statement.visibility, context)?,
                        path: ir::QualifiedName::lower(&use_statement.name, context)?,
                        id: ir::Identifier::lower(
                            match &use_statement.use_as {
                                // Use id `C` from `as C`
                                Some(id) => id,
                                // Use id `Circle` from last part of path `std::geo2d::Circle`
                                None => id,
                            },
                            context,
                        )?,
                    })),
                    None => unreachable!(),
                    Some(_) => Ok(None),
                },
                _ => Ok(None),
            })?,
            wildcards: extract_statements(node, |stmt| match stmt {
                ast::Statement::Use(use_statement) => match use_statement.name.parts.last() {
                    Some(ast::UseStatementPart::Glob(_)) => Ok(Some(ir::WildcardAlias {
                        attr: ir::Attributes::lower(&use_statement.attributes, context)?,
                        keyword_src_ref: context.src_ref(&use_statement.keyword_span),
                        visibility: ir::Visibility::lower(&use_statement.visibility, context)?,
                        path: ir::QualifiedName::lower(&use_statement.name, context)?,
                    })),
                    None => unreachable!(),
                    Some(_) => Ok(None),
                },
                _ => Ok(None),
            })?,
        })
    }
}
