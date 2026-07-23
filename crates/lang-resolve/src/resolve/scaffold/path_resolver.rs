// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Identifier, MICROCAD_EXTENSION, Source, SrcReferrer};

/// The path resolver gets the file paths of file modules to be eventually resolved.
///
/// Similar to [`mod scaffold`], the PathResolver only operates only a single source file.
pub trait PathResolver {
    /*fn file_module_path(
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
    }*/
}
