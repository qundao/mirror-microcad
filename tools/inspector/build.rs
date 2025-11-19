// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Build script for microcad inspector

fn main() {
    slint_build::compile("ui/mainwindow.slint").expect("No error");
}
