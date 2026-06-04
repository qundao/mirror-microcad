// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve the intermediate representation into a [`Symbol`].

mod externals;
mod grant;
mod lookup;
mod resolve_context;
mod resolve_error;
mod sources;
mod symbolize;

pub use externals::*;
pub use lookup::*;
pub use resolve_context::*;
pub use resolve_error::*;
pub use sources::*;

use grant::*;
