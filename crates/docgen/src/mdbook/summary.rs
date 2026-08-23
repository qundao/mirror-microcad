// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Summary markdown for an MdBook

use derive_more::Display;
use microcad_lang_markdown::WriteToFile;

#[derive(Display, Debug)]
#[display("{:indent$}- [`{name}`]({path})", "", indent = 2 * self.level, path = self.path.display())]
pub struct SummaryEntry {
    level: usize,
    name: String,
    path: std::path::PathBuf,
}

impl SummaryEntry {
    pub fn new(level: usize, name: impl Into<String>, path: impl AsRef<std::path::Path>) -> Self {
        Self {
            level,
            name: name.into(),
            path: path.as_ref().to_path_buf(),
        }
    }
}

#[derive(Debug)]
pub struct Summary {
    entries: Vec<SummaryEntry>,
}

impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# Summary\n")?;
        self.entries
            .iter()
            .try_for_each(|entry| writeln!(f, "{entry}"))
    }
}

impl FromIterator<SummaryEntry> for Summary {
    fn from_iter<T: IntoIterator<Item = SummaryEntry>>(iter: T) -> Self {
        Self {
            entries: iter.into_iter().collect(),
        }
    }
}

impl WriteToFile for Summary {}
