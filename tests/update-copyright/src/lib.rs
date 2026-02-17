// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use regex::Regex;
use scan_dir::ScanDir;
use std::{fs, process::Command};

/// Search a path for specific files and update copyright notice
/// # Arguments
/// `search_path`: Path which will be recursively searched
/// `has_extensions`: List of extensions
pub fn update_copyrights(
    search_path: impl AsRef<std::path::Path>,
    extensions: &[(&str, &[&str])],
    exclusions: &[&str],
    check_only: bool,
) -> std::io::Result<bool> {
    let mut check_failed = false;

    // convert exclusions into patterns
    let exclusions: Vec<_> = exclusions
        .iter()
        .map(|pattern| glob::Pattern::new(pattern).expect("bad exclusion pattern"))
        .collect();

    let re = Regex::new(r"Copyright © (\d{4}(-\d{4})?)").unwrap();

    let files = ScanDir::files()
        .walk(&search_path, |iter| {
            iter.filter(|(entry, _)| {
                !exclusions
                    .iter()
                    .any(|exclusion| exclusion.matches(&entry.path().to_string_lossy()))
            })
            .filter(|(_, name)| {
                extensions.iter().any(|extensions| {
                    extensions
                        .1
                        .iter()
                        .any(|extension| name.ends_with(extension))
                })
            })
            .map(|(ref entry, _)| entry.path())
            .collect::<Vec<_>>()
        })
        .expect("scan_path failed");

    for path in files {
        if let Some(prefix) = extensions.iter().find_map(|(prefix, extensions)| {
            if let Some(ext) = path.extension() {
                extensions
                    .contains(&ext.to_string_lossy().to_string().as_str())
                    .then_some(*prefix)
            } else {
                None
            }
        }) {
            let mut content = fs::read_to_string(&path)?;
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < 2
                || !lines[0].starts_with(&format!("{prefix} Copyright"))
                || !lines[1].starts_with(&format!("{prefix} SPDX-License-Identifier"))
            {
                let copyright_notice = format!(
                    "{prefix} Copyright © 0000 The µcad authors <info@ucad.xyz>\n{prefix} SPDX-License-Identifier: AGPL-3.0-or-later\n\n"
                );

                let mut new_content = String::new();
                new_content.push_str(&copyright_notice);
                new_content.push_str(&content);
                println!("cargo:warning=new: {path:?}");
                content = new_content;
            }

            let git_log = Command::new("git")
                .arg("log")
                .arg("--follow")
                .arg("--format=%ad")
                .arg("--date=short")
                .arg(path.to_str().unwrap())
                .output()?;

            let git_log = String::from_utf8(git_log.stdout).unwrap();
            if !git_log.is_empty() {
                let years: Vec<&str> = git_log
                    .lines()
                    .map(|s| s.split('-').next().unwrap())
                    .collect();
                let min_year = years.iter().min().unwrap();
                let max_year = years.iter().max().unwrap();

                let year_range = if min_year == max_year {
                    min_year.to_string()
                } else {
                    format!("{min_year}-{max_year}")
                };

                let new_content = re.replace(&content, format!("Copyright © {year_range}"));

                if new_content != content {
                    println!("cargo:warning=update: {path:?} -> {year_range}");
                    if check_only {
                        check_failed = true;
                    } else {
                        fs::write(&path, new_content.to_string())?;
                    }
                }
            } else {
                eprintln!("untracked file: {path:?}");
            }
        }
    }

    Ok(check_failed)
}
