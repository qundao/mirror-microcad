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

pub use diagnostic::*;
pub use diagnostics::*;
