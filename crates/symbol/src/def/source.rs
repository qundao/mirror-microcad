// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::def::workbench;

#[derive(Debug, derive_more::From)]
pub enum SourceStatement {
    Expression(workbench::ExpressionStatement),
    Local(workbench::LocalAssignment),
}

#[derive(Debug)]
pub struct Source {
    /// Local or expression statements.
    pub script: Vec<SourceStatement>,
}
