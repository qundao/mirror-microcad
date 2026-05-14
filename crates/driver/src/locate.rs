// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function to locate microcad documents.

use crate::{Result, base::*};

/// Return `true` if given path has a valid microcad extension
pub fn is_microcad_file(path: impl AsRef<std::path::Path>) -> bool {
    let path = path.as_ref();
    path.is_file()
        && path
            .extension()
            .map(|ext| {
                MICROCAD_EXTENSIONS
                    .iter()
                    .any(|extension| *extension == ext)
            })
            .unwrap_or(false)
}

/// Retrieve actual path of µcad file, even if this path does not have an extension or is a folder.
///
/// It is agnostic about file extension and will always return the file with the extension that actually exists.
/// If the path is a directory, it will be checked if the directory contains a `mod` file with µcad extension.
///
/// - my/library/my_design.µcad -> my/library/my_design.µcad
/// - my/library/my_design -> my/library/my_design.µcad # In case the extension of the existing file is `.µcad`.
/// - my/library/my_design -> my/library/my_design.mcad # In case the extension of the existing file is `.mcad`.
/// - my/library/my_design -> my/library/my_design.ucad # In case the extension of the existing file is `.ucad`.
/// - my/library/my_design -> my/library/my_design/mod.µcad # `my_design` is directory and a module.
pub fn resolved_path(path: impl AsRef<std::path::Path>) -> Result<std::path::PathBuf> {
    let path = path.as_ref();

    // If the path already has a supported extension, check if it exists.
    if is_microcad_file(path) {
        return Ok(path.to_path_buf());
    }

    // If not, try all supported extensions.
    for ext in MICROCAD_EXTENSIONS {
        let mut with_ext = path.to_path_buf();
        with_ext.set_extension(ext);
        if with_ext.exists() {
            return Ok(with_ext);
        }
    }

    // If the path is a directory, look for a `mod` file with any supported extension.
    if path.is_dir() {
        for ext in MICROCAD_EXTENSIONS {
            let mut mod_path = path.to_path_buf();
            mod_path.push(format!("mod.{ext}"));
            if mod_path.exists() {
                return Ok(mod_path);
            }
        }
    }

    Err(miette::miette!("No µcad file found at: {}", path.display()))
}

/// Convert an input (e.g. from command line) into a valid and unique URL to be used for any source.
pub fn to_url(input: &str) -> Result<Url> {
    use miette::IntoDiagnostic;

    // 1. Handle the special __builtin case
    if input == "__builtin" {
        return Ok(Url::parse("builtin:///builtin").into_diagnostic()?);
    }

    // Try to parse as a formal URL (e.g., https://, file://, mcad://)
    match Url::parse(input) {
        Ok(url) => match url.scheme() {
            // If we have a file scheme, try to resolve the file.
            "file" => {
                let path = url
                    .to_file_path()
                    .map_err(|_| miette::miette!("Invalid file URL path: {url}"))?;
                let resolved = resolved_path(path)?;
                Ok(Url::from_file_path(resolved)
                    .map_err(|_| miette::miette!("Failed to convert path back to URL"))?)
            }
            _ => Ok(url),
        },

        Err(_) => {
            // 3. Fallback: Treat input as a raw filesystem path
            let local_path = std::path::PathBuf::from(input);
            let resolved = resolved_path(local_path)?;

            // Canonicalize to absolute path to ensure the URL is valid
            let absolute_path =
                std::fs::canonicalize(&resolved)
                    .into_diagnostic()
                    .map_err(|err| {
                        miette::miette!(
                            "{err}: Failed to find absolute path for: {}",
                            resolved.display()
                        )
                    })?;

            Url::from_file_path(absolute_path)
                .map_err(|_| miette::miette!("Converted path is not a valid file URL"))
        }
    }
}
