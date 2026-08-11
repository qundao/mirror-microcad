// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation error

use microcad_builtin::BuiltinError;
use microcad_lang_base::{Identifier, IdentifierList, SrcRef, element::WorkbenchKind};
use microcad_lang_types::{Integer, Type, ValueError, model::ModelOutputType, ty::TypeList};
use microcad_package::tree::SymbolPath;
use miette::Diagnostic;

use thiserror::Error;

/// Evaluation error.
#[derive(Debug, Error, Diagnostic)]
#[allow(missing_docs)]
pub enum EvalError {
    /// An error occurred during handling values.
    #[error("Value error: {0}")]
    ValueError(#[from] ValueError),

    /// An error occurred during handling values.
    #[error("Builtin error: {0}")]
    BuiltinFunctionError(#[from] BuiltinError),

    /// Can't find a project file by it's qualified name.
    #[error("Not implemented: {0}")]
    Todo(String),

    /// List index out of bounds.
    #[error("List index out of bounds: {index} >= {len}")]
    ListIndexOutOfBounds {
        /// Wrong index
        index: usize,
        /// Length of list
        len: usize,
    },

    /// Parameter type mismatch.
    #[error("Type mismatch for `{id}`: expected {expected}, got {found}")]
    TypeMismatch {
        /// Parameter name
        id: Identifier,
        /// Expected type
        expected: Type,
        /// Found type
        found: Type,
    },

    /// Array elements have different types.
    #[error("Array elements have different types: {0}")]
    ArrayElementsDifferentTypes(TypeList),

    /// Argument count mismatch.
    #[error("Argument count mismatch: expected {expected}, got {found} in {args}")]
    ArgumentCountMismatch {
        /// Argument list including the error
        args: String,
        /// Expected number of arguments
        expected: usize,
        /// Found number of arguments
        found: usize,
    },

    /// Assertion failed.
    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    /// Different type expected.
    #[error("Expected type `{expected}`, found type `{found}")]
    ExpectedType {
        /// Expected type.
        expected: Type,
        /// Found type.
        found: Type,
    },

    /// No locals  available on stack.
    #[error("Local stack needed to store {0}")]
    LocalStackEmpty(Identifier),

    /// Unexpected stack frame type
    #[error("Unexpected stack frame of type '{1}' cannot store {0}")]
    WrongStackFrame(Identifier, &'static str),

    /// Unexpected element within expression.
    #[error("Unexpected {0} {1} within expression")]
    UnexpectedNested(&'static str, Identifier),

    /// Missing arguments
    #[error("Missing arguments: {0}")]
    MissingArguments(IdentifierList),

    /// Arguments match by identifier but have incompatible types
    #[error("Arguments match by identifier but have incompatible types: {0}")]
    IdMatchButNotType(String),

    /// Builtin error
    #[error("Builtin error: {0}")]
    BuiltinError(String),

    /// Trying to use multiplicity where it is not allowed
    #[error("Multiplicity not allowed '{0}'")]
    MultiplicityNotAllowed(IdentifierList),

    /// An error if you try to mix 2d and 3d geometries.
    #[error("Cannot mix 2d and 3d geometries")]
    CannotMixGeometry,

    /// A condition of an if statement is not a boolean
    #[error("If condition is not a boolean: {condition}")]
    IfConditionIsNotBool {
        condition: String,
        #[label("Not a boolean")]
        src_ref: SrcRef,
    },

    /// Workbench didn't find a initialization routine matching the given arguments
    #[error("Workbench {name} cannot find initialization for those arguments")]
    #[diagnostic(help("Possible initializations: \n\t{}", possible_params.join("\n\t")))]
    NoInitializationFound {
        #[label("Got: {name}( {actual_params} )")]
        src_ref: SrcRef,
        name: Identifier,
        actual_params: String,
        possible_params: Vec<String>,
    },
    /// Workbench didn't find a initialization routine matching the given arguments
    #[error("Workbench {name} has ambiguous initialization for those arguments")]
    #[diagnostic(help("Ambiguous initializations: \n\t{}", ambiguous_params.join("\n\t")))]
    AmbiguousInitialization {
        #[label("Got: {name}( {actual_params} )")]
        src_ref: SrcRef,
        name: Identifier,
        actual_params: String,
        ambiguous_params: Vec<String>,
    },

    /// Initializer missed to set a property from plan
    #[error("Building plan incomplete. Missing properties: {0}")]
    BuildingPlanIncomplete(IdentifierList),

    /// This errors happens if the expression is supposed to produce models but did not.
    #[error("This expression statement did not produce any model")]
    EmptyModelExpression,

    /// This error happens if the workbench produced a different output type.
    #[error("The {kind} workbench produced a {produced} output, but expected a {expected} output.")]
    WorkbenchInvalidOutput {
        kind: WorkbenchKind,
        produced: ModelOutputType,
        expected: ModelOutputType,
    },

    /// Cannot call operation without workpiece, e.g. `op()`.
    #[error("Cannot call operation without workpiece.")]
    CannotCallOperationWithoutWorkpiece,

    /// There is no model in this workbench
    #[error("Missing model in workbench")]
    NoModelInWorkbench,

    /// Assignment failed because value already has been defined before.
    #[error("Value {name} already in defined: {value}")]
    #[diagnostic(help("Values in microcad are immutable"))]
    ValueAlreadyDefined {
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

    /// Assignment failed because left side is not an l-value
    #[error("Assignment failed because {0} is not an l-value")]
    NotAnLValue(Identifier),

    /// Found unused global symbols.
    #[error("Unused global symbol {0}.")]
    UnusedGlobalSymbol(String),

    /// Unused local.
    #[error("Unused local {0}.")]
    UnusedLocal(Identifier),

    /// Evaluation aborted because of prior resolve errors
    #[error("Evaluation aborted because of prior resolve errors!")]
    ResolveFailed,

    /// Bad range (first > last)
    #[error("First number ({first}) must be smaller than last ({last})")]
    BadRange {
        first: Integer,
        last: Integer,
        #[label("Bad range")]
        src_ref: SrcRef,
    },

    /// Ambiguous types in tuple
    #[error("Ambiguous type '{ty}' in tuple")]
    AmbiguousType {
        ty: Type,
        #[label(
            "Some unnamed values in this tuple share the same type '{ty}'.\nMaybe check the units or use identifiers in this tuple."
        )]
        src_ref: SrcRef,
    },

    /// Invalid control flow
    #[error("Invalid flow")]
    InvalidFlow(#[label("This statement does not return a value.")] SrcRef),

    #[error("range expression boundaries must be integers")]
    InvalidRangeBoundaryType {
        #[label("This expression does not evaluate to an integer")]
        src_ref: SrcRef,
    },

    #[error("This expression is expected to return a value.")]
    ExpectedExpression { src_ref: SrcRef },

    #[error("This call returns a value but it is ignored")]
    CallReturnValueIgnored(SrcRef),

    #[error("Symbol `{symbol_path}` cannot be called.")]
    SymbolCanNotBeCalled {
        symbol_path: SymbolPath,

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
    LocalNotFound(#[label("Name of the local")] Identifier),

    #[error("Duplicated argument {id}.")]
    DuplicateArgument {
        #[label("The argument")]
        id: Identifier,
    },

    #[error("Too many arguments: got {given}, expected {expected}")]
    TooManyArguments { given: usize, expected: usize },

    #[error("Missing required argument {id}")]
    MissingRequiredArgument { id: Identifier },
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
