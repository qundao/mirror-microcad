// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer processor request.

use crate::Config;

/// A processor request.
///
/// Commands that can be passed to the [`Processor`].
#[derive(Clone)]
pub enum ProcessorRequest {
    /// Initialize the interpreter.
    ///
    /// Request must only be sent once and sets the initialize flag to `true`.
    Initialize { config: Config },
    /// Parse file.
    ParseFile(std::path::PathBuf),
    /// Parse some code into a SourceFile.
    ParseSource {
        /// Virtual file path
        path: Option<std::path::PathBuf>,
        /// Optional name of the source code snippet, e.g. the full file name.
        name: Option<String>,
        /// The actual source code.
        source: String,
    },
    /// Evaluate source file into a model to be rendered.
    Eval,
    /// Render the geometry. This message should be sent when the source code has been modified.
    Render(Option<microcad_core::RenderResolution>),
    /// Export the geometry to a file.
    Export {
        /// File name.
        filename: std::path::PathBuf,
        /// Optional exporter ("svg", "stl").
        exporter: Option<String>,
    },
    /// Highlight models with a certain line number, if the line number is not [`None`].
    ///
    /// Rerenders the model.
    SetLineNumber(Option<u32>),
}
