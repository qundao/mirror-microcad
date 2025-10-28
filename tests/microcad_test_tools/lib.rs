// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate tests out of *Markdown* files which include µcad code
//!
//! Path will be scanned recursively for *Markdown* files (`*.md`).
//! Code must be marked by *Markdown* code markers (code type: `µcad`) with a test ID attached.
//! In case of a failing test `#fail` must be appended to the test ID.
//!
//! Relative path's of scanned folder names will be used to build a modules structure
//! in the resulting code.
//! If test IDs include `.` name will be split into several names which will be
//! used to crates sub modules.

pub mod output;
pub mod test_env;
