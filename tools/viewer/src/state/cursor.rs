// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cursor state.

use microcad_lang::src_ref::SrcRef;

#[derive(Default)]
pub struct Cursor {
    _src_ref: SrcRef,
}
