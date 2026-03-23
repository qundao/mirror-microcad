// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Processing of µcad source code.
//!
//! This module includes all components to parse, resolve and evaluate µcad code and diagnose errors.
//!
//! - Load and parse source files in [`mod@parse`] and [`syntax`]
//! - Resolve parsed sources in [`resolve`]
//! - Evaluate resolved sources in [`eval`]
//! - Diagnose any evaluation errors in [`diag`]
//!
//! The syntax definitions and parser of µcad can be found [here](../../../syntax).
//!
//! Good starting point to understand how µcad syntax works: [`syntax::SourceFile::load()`] loads a µcad source file.

pub mod builtin;
pub mod doc;
pub mod eval;
pub mod model;
pub mod ord_map;
pub mod parse;
pub mod parser;
pub mod render;
pub mod resolve;
pub mod symbol;
pub mod syntax;
pub mod ty;
pub mod value;

pub(crate) use microcad_lang_base::Id;

pub(crate) use crate::syntax::{Identifiable, Identifier};
