// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve Context

use microcad_core::hash::HashSet;
use microcad_lang_base::{
    Diag, DiagHandler, DiagResult, Diagnostic, GetSourceStrByHash, PushDiag, SrcReferrer,
    TreeDisplay, TreeState, WriteToFile,
};

use crate::{resolve::*, symbol::Symbol, syntax::*};

/// Resolve Context
#[derive(Default)]
pub struct ResolveContext {
    /// Symbol table.
    pub root: Symbol,
    /// Source file cache.
    pub(crate) sources: Sources,
    /// Diagnostic handler.
    pub(crate) diag: DiagHandler,
}

impl ResolveContext {
    /// Load resolve and check a source file and referenced files.
    pub fn create(
        root: std::rc::Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
        builtin: Option<Symbol>,
        diag: DiagHandler,
    ) -> ResolveResult<Self> {
        let mut context = Self {
            sources: Sources::load(root.clone(), search_paths)?,
            diag,
            ..Default::default()
        };
        match context.load(builtin) {
            Ok(()) => Ok(context),
            Err(err) => {
                context.error(&err.src_ref(), err)?;
                Ok(context)
            }
        }
    }

    fn load(&mut self, builtin: Option<Symbol>) -> ResolveResult<()> {
        self.symbolize()?;
        log::trace!("Symbolized Context:\n{self:?}");
        if let Some(builtin) = builtin {
            log::trace!("Added builtin library {id}.", id = builtin.id());
            self.root.add_symbol(builtin)?;
        }
        self.resolve()?;

        Ok(())
    }

    pub(crate) fn symbolize(&mut self) -> ResolveResult<()> {
        let named_symbols = self
            .sources
            .clone()
            .iter()
            .map(|source| {
                match (
                    self.sources.generate_name_from_path(&source.filename()),
                    source.symbolize(Visibility::Public, self),
                ) {
                    (Ok(name), Ok(symbol)) => Ok((name, symbol)),
                    (_, Err(err)) | (Err(err), _) => Err(err),
                }
            })
            .collect::<ResolveResult<Vec<_>>>()?;
        for (name, symbol) in named_symbols {
            if let Some(id) = name.single_identifier() {
                self.root.insert_symbol(id.clone(), symbol)?;
            } else {
                unreachable!("name is not an id")
            }
        }

        Ok(())
    }

    pub(super) fn resolve(&mut self) -> ResolveResult<()> {
        // resolve std as first
        if let Some(std) = self.root.get_child(&Identifier::no_ref("std")) {
            std.resolve(self)?;
        }

        // multi pass resolve
        const MAX_PASSES: usize = 3;
        let mut passes_needed = 0;
        let mut resolved = false;
        for _ in 0..MAX_PASSES {
            self.root
                .iter()
                .filter(|child| child.is_resolvable())
                .map(|child| child.resolve(self))
                .collect::<Result<Vec<_>, _>>()?;
            passes_needed += 1;
            if !self.has_links() {
                resolved = true;
                break;
            }
            self.diag.clear()
        }

        if resolved {
            log::info!("Resolve OK ({passes_needed} passes).");
        } else {
            log::info!("Resolve failed after {passes_needed} passes.");
        }
        log::debug!("Resolved symbol table:\n{self:?}");

        Ok(())
    }

    fn has_links(&self) -> bool {
        self.root
            .iter()
            .filter(|symbol| !symbol.is_deleted())
            .any(|symbol| symbol.has_links())
    }

    /// Load file into source cache and symbolize it into a symbol.
    pub fn symbolize_file(
        &mut self,
        visibility: Visibility,
        parent_path: impl AsRef<std::path::Path>,
        id: &Identifier,
    ) -> ResolveResult<Symbol> {
        let mut symbol = self
            .sources
            .load_mod_file(parent_path, id)?
            .symbolize(visibility, self)?;
        symbol.set_src_ref(id.src_ref());
        Ok(symbol)
    }
}

impl WriteToFile for ResolveContext {}

impl PushDiag for ResolveContext {
    fn push_diag(&mut self, diag: Diagnostic) -> DiagResult<()> {
        self.diag.push_diag(diag)
    }
}

impl GetSourceStrByHash for ResolveContext {
    fn get_str_by_hash(&self, hash: u64) -> Option<&str> {
        self.sources.get_str_by_hash(hash)
    }

    fn get_filename_by_hash(&self, hash: u64) -> Option<std::path::PathBuf> {
        self.sources.get_filename_by_hash(hash)
    }
}

impl Diag for ResolveContext {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag.pretty_print(f, self)
    }

    fn warning_count(&self) -> u32 {
        self.diag.warning_count()
    }

    fn error_count(&self) -> u32 {
        self.diag.error_count()
    }

    fn error_lines(&self) -> HashSet<usize> {
        self.diag.error_lines()
    }

    fn warning_lines(&self) -> HashSet<usize> {
        self.diag.warning_lines()
    }
}

impl GetSourceByHash for ResolveContext {
    fn get_by_hash(&self, hash: u64) -> ResolveResult<std::rc::Rc<SourceFile>> {
        self.sources.get_by_hash(hash)
    }
}

impl std::fmt::Debug for ResolveContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Sources:\n")?;
        write!(f, "{:?}", &self.sources)?;
        writeln!(f, "\nSymbols:\n")?;
        self.root.tree_print(f, TreeState::new_debug(1))?;
        let err_count = self.diag.error_count();
        if err_count == 0 {
            writeln!(f, "No errors.")?;
        } else {
            writeln!(f, "\n{err_count} error(s):\n")?;
            self.diag.pretty_print(f, &self.sources)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ResolveContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Resolved symbols:")?;
        self.root.tree_print(f, TreeState::new_display())?;

        if self.has_errors() {
            writeln!(
                f,
                "{diag}{err} error(s) and {warn} warning(s) so far.",
                err = self.error_count(),
                warn = self.warning_count(),
                diag = self.diagnosis()
            )?;
        } else {
            writeln!(f, "No errors so far.")?;
        }
        Ok(())
    }
}
