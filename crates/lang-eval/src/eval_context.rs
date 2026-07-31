// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// *Context* for *evaluation* of a resolved µcad file.
///
/// The context is used to store the current state of the evaluation.
pub struct EvalContext {
    /// Symbol table
    pub root: Symbol,
    /// Source cache
    sources: Sources,
    /// Stack of currently opened scopes with symbols while evaluation.
    pub(super) stack: Stack,
    /// Output channel for [__builtin::print].
    output: Box<dyn Output>,
    /// Exporter registry.
    exporters: ExporterRegistry,
    /// Importer registry.
    importers: ImporterRegistry,
}

impl EvalContext {
    /// Create a new context from a resolved symbol table.
    pub fn new(
        resolve_context: ResolveContext,
        output: Box<dyn Output>,
        exporters: ExporterRegistry,
        importers: ImporterRegistry,
    ) -> Self {
        log::debug!("Creating evaluation context");

        Self {
            root: resolve_context.root,
            sources: resolve_context.sources,
            output,
            exporters,
            importers,
            stack: Stack::default(),
        }
    }

    /// Current symbol, panics if there no current symbol.
    pub(crate) fn current_symbol(&self) -> Option<Symbol> {
        self.stack.current_symbol()
    }

    /// Access captured output.
    pub fn output(&self) -> Option<String> {
        self.output.output()
    }

    /// Print for `__builtin::print`.
    pub fn print(&mut self, what: String) {
        self.output.print(what).expect("could not write to output");
    }
}
