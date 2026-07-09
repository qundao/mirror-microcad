// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Result;
use crate::prelude as mu;

use microcad_lang_base::ArtifactKind;
use microcad_lang_base::CompilationResult;
use microcad_lang_base::{Artifact, DiagRenderOptions};

use miette::Diagnostic;
use miette::IntoDiagnostic;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum SourceError {
    /// An IO error
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    /// An that occured during lowering
    #[error("Lower error: {0}")]
    LowerError(#[from] mu::lower::LowerError),

    /// Invalid compilation state.
    #[error("Invalid state")]
    InvalidState,
}

/// A µcad source file document.
pub struct SourceFile {
    pub source: mu::Source,
    pub ast: Option<mu::CompilationResult<mu::Ast>>,
    pub ir: Option<mu::CompilationResult<mu::Ir>>,
}

impl SourceFile {
    /// New source file from a source.
    pub fn new(source: mu::Source) -> Self {
        Self {
            source,
            ast: None,
            ir: None,
        }
    }

    /// Load a source file from file.
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let source = mu::Source::load(mu::locate::resolved_path(path)?).into_diagnostic()?;
        Ok(Self::new(source))
    }

    /// Extract artifact from a compilation result.
    fn extract_artifact<T: Artifact>(result: &Option<CompilationResult<T>>) -> Result<&T> {
        match result {
            Some(Ok((artifact, _))) => Ok(artifact),
            _ => Err(miette::miette!("No artifact!")),
        }
    }

    fn _emit<T>(&self, path: impl AsRef<std::path::Path>, artifact: &T) -> Result
    where
        T: Artifact + Serialize,
    {
        let mut path = path.as_ref().to_path_buf();
        path.add_extension(&T::kind().to_string().to_lowercase());
        Ok(std::fs::write(path, artifact.to_ron()?).into_diagnostic()?)
    }

    /// Emit a compiler artifact with a path. The compiler artifact extension is added automatically.
    pub fn emit(&self, path: impl AsRef<std::path::Path>, artifact_kind: &ArtifactKind) -> Result {
        match artifact_kind {
            ArtifactKind::Ast => self._emit(path, Self::extract_artifact(&self.ast)?),
            ArtifactKind::Ir => self._emit(path, Self::extract_artifact(&self.ir)?),
            ArtifactKind::Rst => todo!(),
        }
    }

    /// Return iterator over diagnostics
    pub fn diagnostics(&self) -> impl Iterator<Item = &mu::Diagnostic> {
        fn extract_diags<'a, T>(
            result: &'a Option<mu::CompilationResult<T>>,
        ) -> Box<dyn Iterator<Item = &'a mu::Diagnostic> + 'a> {
            match result {
                Some(Ok((_, diags))) | Some(Err(diags)) => Box::new(diags.iter()),
                None => Box::new(std::iter::empty()),
            }
        }

        extract_diags(&self.ast).chain(extract_diags(&self.ir))
    }

    /// Loads the code from the file specified in the `url`.
    ///
    /// # Errors
    /// Returns an error if the URL is not a valid file path or if the file cannot be read.
    pub fn load_from_file(url: mu::Url) -> mu::Result<Self> {
        // 1. Convert the URL to a local file path
        let path: std::path::PathBuf = url
            .to_file_path()
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "The provided URL is not a valid local file path",
                )
            })
            .into_diagnostic()?;

        // 2. Construct and return the Source instance
        Ok(Self::new(mu::Source::load(path).into_diagnostic()?))
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> mu::Result<Self> {
        Ok(Self::load_from_file(mu::locate::to_url(
            path.as_ref().as_os_str().to_str().unwrap(),
        )?)?)
    }
}

impl mu::commands::GetCode for SourceFile {
    fn get_code(&self) -> Option<&str> {
        Some(self.source.code())
    }
}

impl mu::commands::SetCode for SourceFile {
    fn set_code(&mut self, code: String) -> Option<&str> {
        self.source = mu::Source::new(self.source.location.clone(), code);
        self.ast = None;
        self.ir = None;
        Some(self.source.code())
    }
}

impl mu::commands::Format for SourceFile {
    fn format(&mut self, params: &mu::commands::FormatParameters) -> Result<bool> {
        self.ir = None;

        match mu::format(&self.source, params) {
            Ok(((ast, source), diags)) => {
                let changed = self.source.code() != source.code();
                self.source = source;
                self.ast = Some(Ok((ast, diags)));
                Ok(changed)
            }
            Err(diags) => {
                self.ast = Some(Err(diags));
                Ok(false)
            }
        }
    }
}

impl mu::commands::Sync for SourceFile {
    fn sync(&self) -> Result {
        match &self.source.path() {
            Some(path) => self.source.save(path).into_diagnostic(),
            None => Err(miette::miette!("This source does not have a file path")),
        }
    }
}

impl mu::commands::compile::Parse for SourceFile {
    fn parse(&mut self) -> Result {
        self.ir = None;
        self.ast = Some(mu::parse(&self.source));
        Ok(())
    }
}

impl mu::commands::compile::Lower for SourceFile {
    fn lower(&mut self) -> Result {
        match &self.ast {
            Some(Ok((ast, _))) => {
                self.ir = Some(mu::lower(&self.source, ast));
                Ok(())
            }
            _ => Err(SourceError::InvalidState.into()),
        }
    }
}

impl mu::commands::Compile for SourceFile {}

impl mu::commands::PrintDiagnostics for SourceFile {
    fn print_diagnostics(
        &self,
        _f: &mut dyn std::fmt::Write,
        _options: &DiagRenderOptions,
    ) -> std::fmt::Result {
        todo!()
    }
}
