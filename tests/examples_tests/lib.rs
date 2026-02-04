// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate tests out of examples

use miette::{IntoDiagnostic, Result};
use std::{io::Write, path::Path};

pub fn generate_example_book(
    input_folder: impl AsRef<Path>,
    book_src: impl AsRef<Path>,
) -> Result<()> {
    let mut examples = Vec::new();
    for entry in std::fs::read_dir(input_folder).into_diagnostic()?.flatten() {
        // get file type
        if let Ok(file_type) = entry.file_type() {
            let path = entry.path();
            let file_name = path
                .file_name()
                .expect("test error")
                .to_string_lossy()
                .to_string();
            let name = path
                .file_stem()
                .expect("test error")
                .to_string_lossy()
                .to_string();
            // check if directory or file
            if file_type.is_file() {
                if let Some(ext) = path.extension() {
                    if ["µcad", "mcad", "ucad"]
                        .contains(&ext.to_string_lossy().to_string().as_str())
                    {
                        let content = std::fs::read_to_string(path.clone()).into_diagnostic()?;
                        let code = format!(
                            "# Example: {name}

[![test](.test/{name}.svg)](.test/{name}.log)

```µcad,{name}
{content}
```

![test](.test/{name}-out.svg)

![test](.test/{name}-out.stl)
"
                        );

                        eprintln!("{code}");

                        let dest = book_src.as_ref().join(format!("{name}.md"));
                        std::fs::File::create(dest.clone())
                            .expect("file access error")
                            .write_all(code.as_bytes())
                            .expect("write error");
                        let md_name = dest
                            .file_name()
                            .expect("test error")
                            .to_string_lossy()
                            .to_string();
                        examples.push((file_name, md_name));
                    }
                }
            } else if file_type.is_dir() {
            }
        }
    }

    let summary = book_src.as_ref().join("SUMMARY.md");
    let code = format!(
        "# Examples

{}",
        examples
            .iter()
            .map(|(name, md)| format!("- [{name}]({})", md))
            .collect::<Vec<_>>()
            .join("\n")
    );
    std::fs::File::create(summary)
        .expect("file access error")
        .write_all(code.as_bytes())
        .expect("write error");

    Ok(())
}
