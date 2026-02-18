// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to search for files
    #[arg(default_value = ".")]
    search_path: std::path::PathBuf,
    /// File extensions to include (using "# " as comment prefix)
    #[arg(short = 'H', long)]
    hash_extensions: Vec<String>,
    /// File extensions to include (using "// " as comment prefix)
    #[arg(short = 'S', long)]
    slash_extension: Vec<String>,
    /// patterns to exclude (e.g. "./target/*")
    #[arg(short, long)]
    exclude_patterns: Vec<String>,
}

use regex::Regex;

fn main() -> std::io::Result<()> {}
