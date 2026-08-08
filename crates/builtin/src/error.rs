// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad built-in errors.

use microcad_lang_types::ValueError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuiltinError {
    #[error("Value error: {0}")]
    ValueError(#[from] ValueError),

    #[error("Builtin error in '{name}': {message}")]
    ExecutionFailed { name: String, message: String },
}
