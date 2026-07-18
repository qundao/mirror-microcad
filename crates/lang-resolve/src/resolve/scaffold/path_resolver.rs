// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Identifier, MICROCAD_EXTENSION, Source, SrcReferrer};

use crate::scaffold::ScaffoldError;

/// The path resolver gets the file paths of file modules to be eventually resolved.
///
/// Similar to [`mod scaffold`], the PathResolver only operates only a single source file.
pub trait PathResolver<'source> {
    fn source(&self) -> &'source Source;

    fn source_path(&self) -> Result<std::path::PathBuf, ScaffoldError> {
        let source = self.source();
        match source.path() {
            Some(path) => Ok(path),
            None => Err(ScaffoldError::SourceHasNoPath {
                loc: source.location.clone(),
                src_ref: source.src_ref(),
            }),
        }
    }

    fn module_name_from_source(&self) -> Result<Identifier, ScaffoldError> {
        let source_path = self.source_path()?;

        // Extract `foo` from `file/to/foo.mu`
        let stem = source_path.file_stem().unwrap().to_str().unwrap(); // TODO Remove unwrap here.
        Ok(Identifier::from(stem))
    }

    fn file_module_path(
        &self,
        file_module_name: &Identifier,
    ) -> Result<std::path::PathBuf, ScaffoldError> {
        Ok(self
            .source_path()?
            .with_extension("")
            .join(file_module_name.id().to_string())
            .with_extension(MICROCAD_EXTENSION))
    }

    fn file_module_path_as_string(
        &self,
        file_module_name: &Identifier,
    ) -> Result<String, ScaffoldError> {
        let path = self.file_module_path(file_module_name)?;

        Ok(path.to_string_lossy().into_owned())
    }
}

/// The [`DefaultPathResolver`] does perform any file system operations.
pub struct DefaultPathResolver<'source> {
    pub(crate) source: &'source Source,
}

impl<'source> From<&'source Source> for DefaultPathResolver<'source> {
    fn from(source: &'source Source) -> Self {
        Self { source }
    }
}

impl<'source> PathResolver<'source> for DefaultPathResolver<'source> {
    fn source(&self) -> &'source Source {
        self.source
    }
}

#[cfg(feature = "io")]
pub struct FilePathResolve<'source> {
    pub(crate) source: &'source Source,
}
