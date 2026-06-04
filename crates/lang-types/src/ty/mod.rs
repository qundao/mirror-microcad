// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad syntax elements of types

mod matrix_type;
mod quantity_type;
mod tuple_type;
mod r#type;
mod type_list;
mod unit;

pub use matrix_type::*;
pub use quantity_type::*;
pub use tuple_type::*;
pub use r#type::*;
pub use type_list::*;
pub use unit::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Unknown unit: {0}")]
    UnknownUnit(String),

    #[error("Unknown type: {0}")]
    UnknownType(String),

    /// Matrix type with invalid dimensions
    #[error("Invalid matrix type: {0}")]
    InvalidMatrixType(String),
}

/// Trait for structs and expressions that have a type
pub trait Ty {
    /// Return type
    fn ty(&self) -> Type;
}
