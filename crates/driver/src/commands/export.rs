// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::commands::CommandResult;

pub struct ExportSettings {}

pub trait Export {
    fn export(&self, settings: &ExportSettings) -> CommandResult<()>;
}
