// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A compiler artifact to be persisted, e.g. an IR written and read from file.

use microcad_lang_base::SrcRef;
use serde::{Deserialize, Serialize};

use crate::ir;

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct ExportAttribute {
    file: ir::ConstantExpression,
}

/// A statement in a source file.
///
/// Source file statements are similar to workbench statements with the following features
/// * No properties `prop` allowed
/// * Export attributes
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct SourceStatement {
    //pub exports: Vec<ExportAttribute>,
    pub attr: ir::ModelAttributes,
    pub src_ref: SrcRef,
    pub name: Option<ir::Identifier>,
    pub ty: ir::Type,
    pub expression: ir::WorkbenchExpression,
}

/// A desugared source file.
#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// Workbench statements
    pub statements: Box<[ir::SourceStatement]>,
}
