// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! External files register

use crate::{lower::*, resolve::*};
use derive_more::Deref;

use microcad_core::hash::HashMap;
use microcad_lang_base::MICROCAD_EXTENSIONS;

/// External files register.
///
/// A map of *qualified name* -> *source file path* which is generated at creation
/// by scanning in the given `search_paths`.
#[derive(Default, Deref)]
pub struct Externals(HashMap<ir::QualifiedName, std::path::PathBuf>);

impl Externals {
    /// Creates externals list.
    ///
    /// Recursively scans given `search_paths` for µcad files but files will not be loaded.
    /// # Arguments
    /// - `search_paths`: Paths to search for any external files.
    pub fn new(search_paths: &[impl AsRef<std::path::Path>]) -> ResolveResult<Self> {
        if search_paths.is_empty() {
            log::info!("No external search paths were given");
            Ok(Externals::default())
        } else {
            let new = Self(Self::search_externals(search_paths)?);
            if new.is_empty() {
                log::warn!("Did not find any externals in any search path");
            } else {
                log::info!("Found {} external module(s): {new}", new.len());
                log::trace!("Externals:\n{new:?}");
            }
            Ok(new)
        }
    }

    /// Searches for external source code files (*external modules*) in given *search paths*.
    fn search_externals(
        search_paths: &[impl AsRef<std::path::Path>],
    ) -> ResolveResult<HashMap<ir::QualifiedName, std::path::PathBuf>> {
        search_paths
            .iter()
            .inspect(|p| log::trace!("Searching externals in: {:?}", p.as_ref()))
            .map(|search_path| {
                scan_dir::ScanDir::all()
                    .read(search_path.as_ref(), |iter| {
                        iter.map(|(entry, _)| entry.path())
                            .map(find_external_mod)
                            // catch eny errors here
                            .collect::<Result<Vec<_>, _>>()?
                            .into_iter()
                            .flatten()
                            .map(|file| {
                                let name = make_symbol_name(
                                    file.strip_prefix(search_path)
                                        .expect("cannot strip search path from file name"),
                                );
                                let path = file.canonicalize().expect("path not found");
                                log::trace!("Found external: {name} {path:?}");
                                Ok((name, path))
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()
                    .map(|v| v.into_iter().flatten())
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.into_iter().flatten().collect())
    }
}

impl std::fmt::Display for Externals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0.iter().collect::<Vec<_>>();
        // sort for better readability
        v.sort();
        write!(
            f,
            "{}",
            v.iter()
                .map(|file| file.0.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl std::fmt::Debug for Externals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0.iter().collect::<Vec<_>>();
        // sort for better readability
        v.sort();
        v.iter()
            .try_for_each(|file| writeln!(f, "{} => {}", file.0, file.1.to_string_lossy()))
    }
}

fn make_symbol_name(relative_path: impl AsRef<std::path::Path>) -> ir::QualifiedName {
    let path = relative_path.as_ref();
    let stem = path.file_stem().map(|s| s.to_string_lossy().to_string());
    let name = if stem == Some("mod".into()) {
        path.parent().expect("mod file without parent folder")
    } else {
        path
    };
    name.iter()
        .map(|id| ir::Identifier::no_ref(id.to_string_lossy().as_ref()))
        .collect()
}

fn search_mod_dir_file(
    path: impl AsRef<std::path::Path>,
) -> ResolveResult<Option<std::path::PathBuf>> {
    log::trace!("search_mod_dir_file: {:?}", path.as_ref());
    let files = scan_dir::ScanDir::files().read(path, |iter| {
        iter.map(|(ref entry, _)| entry.path())
            .filter(|p| is_mod_file(p))
            .collect::<Vec<_>>()
    })?;
    if let Some(file) = files.first() {
        match files.len() {
            1 => Ok(Some(file.clone())),
            _ => Err(ResolveError::AmbiguousExternals(files)),
        }
    } else {
        Ok(None)
    }
}

/// Return `true` if given path has a valid microcad extension
pub fn is_microcad_file(p: impl AsRef<std::path::Path>) -> bool {
    p.as_ref().is_file()
        && p.as_ref()
            .extension()
            .map(|ext| {
                MICROCAD_EXTENSIONS
                    .iter()
                    .any(|extension| *extension == ext)
            })
            .unwrap_or(false)
}

/// Return `true` if given path is a file called `mod` plus a valid microcad extension
fn is_mod_file(p: impl AsRef<std::path::Path>) -> bool {
    let p = p.as_ref();
    is_microcad_file(p)
        && p.file_stem()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s == "mod")
}

/// Find a module file by path and id.
///
/// Module files might be on of the following:
///
/// - \<path>`/`\<id>`.`*ext*
/// - \<path>`/`\<id>`/mod.`*ext*
///
/// *ext* = any valid microcad file extension.
pub fn find_mod_file_by_id(
    path: impl AsRef<std::path::Path>,
    id: &ir::Identifier,
) -> ResolveResult<std::path::PathBuf> {
    let path = path.as_ref();
    log::trace!("find_mod_file_by_id: {path:?} {id:?}");
    match (
        search_mod_file_by_id(path, id),
        search_mod_dir_file(path.join(id.to_string())),
    ) {
        (Ok(file), Ok(Some(dir))) => Err(ResolveError::AmbiguousExternals(vec![file, dir])),
        (Ok(file), Err(_) | Ok(None)) | (Err(_), Ok(Some(file))) => Ok(file),
        (Err(err), _) => Err(err),
    }
}

fn find_external_mod(
    path: impl AsRef<std::path::Path>,
) -> ResolveResult<Option<std::path::PathBuf>> {
    log::trace!("find mod file ex: {:?}", path.as_ref());
    let path = path.as_ref().to_path_buf();
    if path.is_dir() {
        return search_mod_dir_file(&path);
    }
    if is_microcad_file(&path) {
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

fn search_mod_file_by_id(
    path: impl AsRef<std::path::Path>,
    id: &ir::Identifier,
) -> ResolveResult<std::path::PathBuf> {
    let path = path.as_ref();

    // Patch path if we are in a test environment
    let path = if std::fs::exists(path.join(".test")).expect("file access failure") {
        path.join(".test")
    } else {
        path.into()
    };

    log::trace!("search_mod_file_by_id: {path:?} {id}");
    if let Some(path) = scan_dir::ScanDir::files().read(&path, |iter| {
        iter.map(|(entry, _)| entry.path())
            .filter(|p| is_microcad_file(p))
            .find(|p| {
                p.file_stem()
                    .map(|stem| *stem == *id.to_string())
                    .unwrap_or(false)
            })
    })? {
        Ok(path)
    } else {
        Err(ResolveError::SourceFileNotFound(
            id.clone(),
            path.to_path_buf(),
        ))
    }
}
