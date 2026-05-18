// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_driver::prelude as mu;

use miette::IntoDiagnostic;

#[test]
fn test_builtin_resolution() {
    let result = mu::locate::to_url("__builtin").unwrap();
    assert_eq!(result.scheme(), "builtin");
    assert_eq!(result.path(), "/builtin");
}

#[test]
fn test_external_url_pass_through() {
    let https_url = "https://example.com/design.mcad";
    let result = mu::locate::to_url(https_url).unwrap();
    assert_eq!(result.as_str(), https_url);
}

#[test]
fn test_filesystem_resolution_with_extension() -> mu::Result {
    let dir = tempfile::tempdir().into_diagnostic()?;
    let file_path = dir.path().join("my_design.µcad");
    std::fs::write(&file_path, "test content").into_diagnostic()?;

    // Test raw string path
    let input = file_path.to_str().unwrap();
    let result = mu::locate::to_url(input)?;

    assert_eq!(result.scheme(), "file");
    // Canonicalize returns the absolute path, so we compare URLs
    assert_eq!(
        result,
        mu::Url::from_file_path(std::fs::canonicalize(&file_path).unwrap()).unwrap()
    );
    Ok(())
}

#[test]
fn test_filesystem_resolution_missing_extension() -> mu::Result {
    let dir = tempfile::tempdir().into_diagnostic()?;
    let file_path = dir.path().join("logic.mcad"); // Using one of the extensions
    std::fs::write(&file_path, "test content").into_diagnostic()?;

    // Input WITHOUT extension
    let input = dir.path().join("logic").to_str().unwrap().to_string();
    let result = mu::locate::to_url(&input)?;

    assert!(result.as_str().ends_with("logic.mcad"));
    Ok(())
}

#[test]
fn test_directory_mod_resolution() -> mu::Result {
    let dir = tempfile::tempdir().into_diagnostic()?;
    let sub_dir = dir.path().join("my_module");
    std::fs::create_dir(&sub_dir).into_diagnostic()?;

    let mod_file = sub_dir.join("mod.µcad");
    std::fs::write(&mod_file, "module content").into_diagnostic()?;

    // Input is the directory path
    let input = sub_dir.to_str().unwrap();
    let result = mu::locate::to_url(input)?;

    assert!(
        result
            .to_file_path()
            .unwrap()
            .ends_with("my_module/mod.µcad"),
        "{result}"
    );
    Ok(())
}

#[test]
fn test_invalid_path_fails() {
    let result = mu::locate::to_url("/non/existent/path/at/all/ever");
    assert!(result.is_err());
}

#[test]
fn test_file_scheme_input() -> mu::Result {
    let dir = tempfile::tempdir().into_diagnostic()?;
    let file_path = dir.path().join("direct.µcad");
    std::fs::write(&file_path, "content").into_diagnostic()?;

    let file_url = mu::Url::from_file_path(&file_path).unwrap();
    let result = mu::locate::to_url(file_url.as_str())?;

    assert_eq!(result, file_url);
    Ok(())
}
