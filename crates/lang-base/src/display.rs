// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub trait DisplayOneLine: std::fmt::Display {
    fn fmt_one_line(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut adapter = OneLineWriter::new(f);
        use std::fmt::Write;
        write!(adapter, "{self}")
    }

    fn to_string_one_line(&self) -> String {
        let full = self.to_string();
        let line = full.lines().next().unwrap_or("");
        shorten(line, 80)
    }
}

/// Helper writer that stops at the first newline to avoid extra work.
struct OneLineWriter<'a, 'b> {
    formatter: &'a mut std::fmt::Formatter<'b>,
    stopped: bool,
}

impl<'a, 'b> OneLineWriter<'a, 'b> {
    fn new(formatter: &'a mut std::fmt::Formatter<'b>) -> Self {
        Self {
            formatter,
            stopped: false,
        }
    }
}

impl std::fmt::Write for OneLineWriter<'_, '_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.stopped {
            return Ok(());
        }
        if let Some(first_line) = s.lines().next() {
            self.formatter.write_str(first_line)?;
            if s.contains('\n') {
                self.stopped = true;
            }
        }
        Ok(())
    }
}

/// Shortens given string to it's first line and to `max_chars` characters.
pub fn shorten(s: &str, max_chars: usize) -> String {
    let s = s.trim_end();
    if s.chars().count() > max_chars {
        format!(
            "{}…",
            s.chars()
                .take(max_chars.saturating_sub(1))
                .collect::<String>()
        )
    } else {
        s.to_string()
    }
}
