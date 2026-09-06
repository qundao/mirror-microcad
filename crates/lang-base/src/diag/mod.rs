// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Handling of diagnostic errors.
//!
//! While *evaluation* µcad is collecting [`Diagnostic`] messages.
//!
//! This is done in [`DiagHandler`] by providing the following traits:
//!
//! - [`PushDiag`]: Collects error in [`DiagHandler`]
//! - [`Diag`]: Get diagnostic messages

mod diagnostic;
mod diagnostics;

pub use diagnostic::Diagnostic;
pub use diagnostics::{DiagRenderOptions, Diagnostics};

pub trait PushDiag<E> {
    fn push_diag(&mut self, err: impl Into<E>);

    /// Append diagnostics
    fn append_diags<T>(&mut self, errors: impl IntoIterator<Item = T>)
    where
        T: Into<E>,
    {
        for err in errors {
            self.push_diag(err);
        }
    }

    /// Push the error if the result is an error or return default.
    fn capture<T: Default>(&mut self, result: Result<T, impl Into<E>>) -> T {
        match result {
            Ok(t) => t,
            Err(err) => {
                self.push_diag(err);
                T::default()
            }
        }
    }

    /// Pushes the error and returns default value, so compilation can continue.
    fn catch<T: Default>(&mut self, err: impl Into<E>) -> Result<T, Box<E>> {
        self.push_diag(err);
        Ok(T::default())
    }

    fn err<T>(&mut self, err: impl Into<E>) -> Result<T, E> {
        Err(err.into())
    }
}
