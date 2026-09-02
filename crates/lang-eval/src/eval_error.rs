// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation error

use microcad_builtin::BuiltinError;
use microcad_lang_base::{
    Identifier, IdentifierList, Name, SrcRef, SrcReferrer, ToCompactString, element::WorkbenchKind,
};
use microcad_lang_types::{Type, ValueError, model::ModelType, ty::TypeList};
use miette::Diagnostic;

use thiserror::Error;

use crate::ArgumentMatchError;

/// Evaluation error.
#[derive(Debug, Error, Diagnostic)]
#[allow(missing_docs)]
pub enum EvalError {
    /// An error occurred during handling values.
    #[error("Value error: {0}")]
    ValueError(#[from] ValueError),

    /// Builtin error
    #[error("Builtin error: {0}")]
    BuiltinError(#[from] BuiltinError),

    #[error("Argument match error: {err}")]
    ArgumentMatch {
        symbol_name: Name,
        context_src_ref: SrcRef,
        err: ArgumentMatchError,
    },

    /// Trying to use multiplicity where it is not allowed
    #[error("Multiplicity not allowed '{0}'")]
    MultiplicityNotAllowed(IdentifierList),

    /// A condition of an if statement is not a boolean
    #[error("If condition is not a boolean: {err}")]
    IfConditionIsNotBool {
        src_ref: SrcRef,

        #[label("Not a boolean")]
        condition_src_ref: SrcRef,

        err: ValueError,
    },

    /// Workbench didn't find a initialization routine matching the given arguments
    #[error("Workbench `{path}` cannot find initialization for those arguments")]
    #[diagnostic(help("Possible initializations: \n\t{}", inits.join("\n\t")))]
    NoInitializationFound {
        #[label("Got: {path}( {arguments} )")]
        src_ref: SrcRef,
        path: String,
        arguments: String,
        inits: Vec<String>,
    },

    /// Workbench didn't find a initialization routine matching the given arguments
    #[error("Workbench `{path}` has ambiguous initialization for those arguments")]
    #[diagnostic(help("Ambiguous initializations: \n\t{}", inits.join("\n\t")))]
    AmbiguousInitialization {
        #[label("Got: {path}( {arguments} )")]
        src_ref: SrcRef,
        path: String,
        arguments: String,
        inits: Vec<String>,
    },

    /// This error happens if the workbench produced a different output type.
    #[error("The {kind} workbench produced a {produced} output, but expected a {expected} output.")]
    WorkbenchInvalidOutput {
        kind: WorkbenchKind,
        produced: ModelType,
        expected: ModelType,
    },

    /// Cannot call operation without workpiece, e.g. `op()`.
    #[error("Cannot call operation without workpiece.")]
    CannotCallOperationWithoutWorkpiece,

    /// There is no model in this workbench
    #[error("Missing model in workbench")]
    NoModelInWorkbench,

    /// Assignment failed because a property already has been defined before.
    #[error("Property `{name}` already defined: {value}")]
    #[diagnostic(help("Values in microcad are immutable"))]
    PropertyAlreadyDefined {
        /// Location of the error
        #[label(primary, "{name} is already defined")]
        location: SrcRef,
        /// Name of the value
        name: Identifier,
        /// Previous value
        value: String,
        /// Previous definition
        #[label("Previously defined here")]
        previous_location: SrcRef,
    },

    /// Unused local.
    #[error("Unused local {0}.")]
    UnusedLocal(Identifier),

    /// Ambiguous types in tuple
    #[error("Ambiguous type '{ty}' in tuple")]
    AmbiguousType {
        ty: Type,
        #[label(
            "Some unnamed values in this tuple share the same type '{ty}'.\nMaybe check the units or use identifiers in this tuple."
        )]
        src_ref: SrcRef,
    },

    #[error("range expression boundaries must be integers")]
    InvalidRangeBoundaryType {
        #[label("This expression does not evaluate to an integer")]
        src_ref: SrcRef,
    },

    #[error("This expression is expected to return a value.")]
    ExpectedExpression { src_ref: SrcRef },

    #[error("This call returns a value but it is ignored")]
    CallReturnValueIgnored(SrcRef),

    #[error("Symbol `{path}` cannot be called.")]
    SymbolCannotBeCalled {
        path: String,

        #[label("Symbol name")]
        src_ref: SrcRef,
    },

    #[error("Local `{id}` will not have a value.")]
    LocalExpressionDidNotProduceAValue {
        #[label("Local `{id}` will not be set.")]
        id: Identifier,
        src_ref: SrcRef,
        expr_src_ref: SrcRef,
    },

    #[error("Local `{0}` not found in scope.")]
    LocalNotFound(Name),

    #[error("Duplicated argument {id}.")]
    DuplicateArgument {
        #[label("The argument")]
        id: Identifier,
    },

    #[error("Too many arguments: got {given}, expected {expected}")]
    TooManyArguments { given: usize, expected: usize },

    #[error("Missing required argument {id}")]
    MissingRequiredArgument { id: Identifier },

    #[error("Constant expression expected")]
    ConstantExpressionExpected {
        #[label("This expression is not a constant")]
        src_ref: microcad_lang_base::SrcRef,
    },

    #[error("Unresolved symbol: {path}")]
    UnresolvedPath {
        path: String,

        #[label("This symbol could not be found in any package")]
        src_ref: SrcRef,
    },
}

impl EvalError {
    pub fn argument_match(
        src_ref: impl SrcReferrer,
        symbol_name: impl AsRef<str>,
        err: ArgumentMatchError,
    ) -> Box<Self> {
        Box::new(Self::ArgumentMatch {
            symbol_name: symbol_name.as_ref().to_compact_string(),
            context_src_ref: src_ref.src_ref(),
            err,
        })
    }
}

impl From<BuiltinError> for Box<EvalError> {
    fn from(err: BuiltinError) -> Self {
        Box::new(err.into())
    }
}

impl From<ValueError> for Box<EvalError> {
    fn from(err: ValueError) -> Self {
        Box::new(err.into())
    }
}

/// Result type of any evaluation.
pub type EvalResult<T> = std::result::Result<T, Box<EvalError>>;

impl From<Box<EvalError>> for miette::Report {
    fn from(value: Box<EvalError>) -> Self {
        miette::Report::new(*value)
    }
}
