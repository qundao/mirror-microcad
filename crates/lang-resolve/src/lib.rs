// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod resolve;

pub mod mir;

/// The mid-level intermediate represenation (MIR).
pub use mir::Mir;

pub use resolve::{ResolveContext, ResolveResult, resolve, scaffold};

pub use scaffold::scaffold;
