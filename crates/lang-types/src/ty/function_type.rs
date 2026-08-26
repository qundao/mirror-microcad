// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A type signature for any callable (builtins, functions, workbenches).

use microcad_lang_base::Identifier;
use serde::{Deserialize, Serialize};

use crate::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionTypeParameters(pub Vec<(Identifier, Type)>);

impl std::fmt::Display for FunctionTypeParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(id, ty)| format!("{id}: {ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl FunctionTypeParameters {
    /// Create a new SignatureParameter
    pub fn new(mut parameters: Vec<(Identifier, Type)>) -> Self {
        parameters.sort_by_key(|(id, _)| id.clone());
        Self(parameters)
    }

    /// Check if a signature matches another.
    ///
    /// Two signature match, if the have the same identifiers and all types match.
    pub fn matches(&self, other: &FunctionTypeParameters) -> bool {
        self.0.len() == other.0.len()
            && self
                .0
                .iter()
                .zip(&other.0)
                .all(|((lhs_id, lhs_ty), (rhs_id, rhs_ty))| {
                    lhs_id == rhs_id && lhs_ty.matches(rhs_ty)
                })
    }

    pub fn matches_multiplicity(&self, other: &FunctionTypeParameters) -> bool {
        self.0.len() == other.0.len()
            && self
                .0
                .iter()
                .zip(&other.0)
                .all(|((lhs_id, lhs_ty), (rhs_id, rhs_ty))| {
                    lhs_id == rhs_id && (lhs_ty.matches(rhs_ty) || lhs_ty.is_list_of(rhs_ty))
                })
    }
}

impl From<Vec<(Identifier, Type)>> for FunctionTypeParameters {
    fn from(parameters: Vec<(Identifier, Type)>) -> Self {
        Self::new(parameters)
    }
}

/// A type signature for any callable (builtins, functions, workbenches).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionType {
    /// Parameters, sorted by id.
    /// The signature is variadic of this list is none.
    pub parameters: Option<FunctionTypeParameters>,

    /// Optional return value.
    /// If this value is None, the callable is not expected to return anything.
    pub return_ty: Option<Box<Type>>,
}

impl FunctionType {
    pub fn new(parameters: impl Into<FunctionTypeParameters>, return_ty: Option<Type>) -> Self {
        Self {
            parameters: Some(parameters.into()),
            return_ty: return_ty.map(Box::new),
        }
    }

    pub fn new_variadic(return_ty: Option<Type>) -> Self {
        Self {
            parameters: None,
            return_ty: return_ty.map(Box::new),
        }
    }

    pub fn is_variadic(&self) -> bool {
        self.parameters.is_none()
    }

    /// Check if a function call `(...) -> Type` matches `(...) -> Type`
    pub fn matches(&self, other: &FunctionType) -> bool {
        match (&self.parameters, &other.parameters) {
            // A variadic function cannot call a function with specified signature parameters.
            (None, Some(_)) => false,
            // A variadic can accept any function
            (Some(lhs), Some(rhs)) if lhs.matches(rhs) => self.match_return_type(other),
            (Some(lhs), Some(rhs)) if !lhs.matches(rhs) => false,
            _ => self.match_return_type(other),
        }
    }

    pub fn matches_multiplicity(&self, other: &FunctionType) -> bool {
        match (&self.parameters, &other.parameters) {
            // A variadic function cannot call a function with specified signature parameters.
            (None, Some(_)) => false,
            // A variadic can accept any function
            (Some(lhs), Some(rhs)) if lhs.matches_multiplicity(rhs) => {
                self.match_return_type(other)
            }
            _ => self.match_return_type(other),
        }
    }

    pub fn match_return_type(&self, other: &FunctionType) -> bool {
        match (&self.return_ty, &other.return_ty) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(lhs), Some(rhs)) => lhs.matches(rhs),
        }
    }
}

impl std::fmt::Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.parameters {
            Some(parameters) => write!(f, "({parameters})"),
            None => write!(f, "(*)"),
        }?;

        match &self.return_ty {
            Some(ty) => write!(f, " -> {ty}"),
            None => Ok(()),
        }
    }
}
