// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_driver::prelude as mu;

/// µcad lsp config
#[derive(Default, Debug)]
pub struct Config {
    /// Use viewer
    pub use_viewer: bool,
    /// Driver configuration
    pub driver: mu::DriverConfig,
}
