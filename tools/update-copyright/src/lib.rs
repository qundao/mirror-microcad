// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use regex::Regex;
use scan_dir::ScanDir;
use std::{fs, process::Command};

fn is_excluded(path: impl AsRef<std::path::Path>, excluded_patterns: &[&str]) -> bool {
    let path_str = path.as_ref().to_str().unwrap_or_default();

    excluded_patterns.iter().any(|pattern| {
        // Convert wildcard '*' to regex '.*'
        let regex_pattern = format!("^{}$", regex::escape(pattern).replace(r"\*", ".*"));
        Regex::new(&regex_pattern)
            .map(|re| re.is_match(path_str))
            .unwrap_or(false)
    })
}

pub fn update_copyrights(
    search_path: impl AsRef<std::path::Path>,
    hash_extensions: &[&str],
    slash_extension: &[&str],
    exclude_patterns: &[&str],
) -> std::io::Result<()> {
    let mut extensions = vec![];
    extensions.extend_from_slice(
        &hash_extensions
            .iter()
            .map(|s| format!(".{s}"))
            .collect::<Vec<_>>(),
    );
    extensions.extend_from_slice(
        &slash_extension
            .iter()
            .map(|s| format!(".{s}"))
            .collect::<Vec<_>>(),
    );

    eprintln!("Searching for extensions: {}", extensions.join(", "));
    eprintln!("Excluding: {}", exclude_patterns.join(", "));

    let re = Regex::new(r"Copyright © (\d{4}(-\d{4})?)").unwrap();

    let files = ScanDir::files()
        .walk(search_path, |iter| {
            iter.filter(|(entry, name)| {
                extensions.iter().any(|extension| {
                    name.ends_with(extension) && !is_excluded(entry.path(), exclude_patterns)
                })
            })
            .map(|(ref entry, _)| entry.path())
            .collect::<Vec<_>>()
        })
        .expect("scan_path failed");

    for path in files {
        let ext = path
            .extension()
            .expect("extension")
            .to_string_lossy()
            .to_string();

        let prefix = if slash_extension.contains(&ext.as_str()) {
            "//"
        } else if hash_extensions.contains(&ext.as_str()) {
            "#"
        } else {
            panic!("unexpected extension")
        };

        let content = fs::read_to_string(&path)?;
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() < 2
            || !lines[0].starts_with(&format!("{prefix} Copyright"))
            || !lines[1].starts_with(&format!("{prefix} SPDX-License-Identifier"))
        {
            let copyright_notice = format!(
                "{prefix} Copyright © 2025 The µcad authors <info@ucad.xyz>\n{prefix} SPDX-License-Identifier: AGPL-3.0-or-later\n\n"
            );

            let mut new_content = String::new();
            new_content.push_str(&copyright_notice);
            new_content.push_str(&content);

            println!("{path:?}");
            fs::write(&path, new_content)?;
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
                fs::write(&path, new_content.to_string())?;
            }
        } else {
            eprintln!("untracked file: {path:?}");
        }
    }

    Ok(())
}
