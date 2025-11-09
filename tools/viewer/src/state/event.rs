// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad State event.

/// An event that is fired when the state is
pub enum StateEvent {
    ChangeGroundRadius,
    SelectAll,
    Deselect,
    SelectOne(Uuid),
}
