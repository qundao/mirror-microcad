// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod resolve;

pub(crate) mod tree;

pub mod mir;
pub mod rst;

/// The mid-level intermediat
pub use mir::Mir;

pub use rst::Rst;

pub use resolve::{ResolveContext, ResolveResult, resolve, scaffold};

pub use scaffold::scaffold;
