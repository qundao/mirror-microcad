// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate tests out of examples

use miette::{IntoDiagnostic, Result};
use std::{io::Write, path::Path};

/// create examples book from examples folder
pub fn generate_example_book(
    input_folder: impl AsRef<Path>,
    book_src: impl AsRef<Path>,
) -> Result<()> {
    let mut examples = Vec::new();
    std::fs::remove_dir_all(book_src.as_ref()).expect("test error");
    std::fs::create_dir(book_src.as_ref()).expect("test error");
    for entry in std::fs::read_dir(input_folder).into_diagnostic()?.flatten() {
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
        // get file type
        if let Ok(file_type) = entry.file_type() {
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
                let folder_name = file_name;
                let mut code = format!("# Example: {name}\n\n");
                for entry in std::fs::read_dir(entry.path()).into_diagnostic()?.flatten() {
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

                    if let Ok(file_type) = entry.file_type() {
                        // check if directory or file
                        if file_type.is_file() {
                            if let Some(ext) = path.extension() {
                                if ["µcad", "mcad", "ucad"]
                                    .contains(&ext.to_string_lossy().to_string().as_str())
                                {
                                    let content =
                                        std::fs::read_to_string(path.clone()).into_diagnostic()?;
                                    code.extend(
                                        format!(
                                            "## Module: {name}

[![test](.test/{folder_name}_{name}.svg)](.test/{folder_name}_{name}.log)

```µcad,{folder_name}_{name}
// file: {file_name}
{content}
```

![test](.test/{folder_name}_{name}-out.svg)

![test](.test/{folder_name}_{name}-out.stl)

"
                                        )
                                        .chars(),
                                    );
                                }
                            }
                        }
                    }
                }
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
                examples.push((folder_name, md_name));
            }
        }
    }

    let summary = book_src.as_ref().join("SUMMARY.md");
    let code = format!(
        "# Examples

{}",
        {
            let mut examples = examples
                .iter()
                .map(|(name, md)| format!("- [{name}]({})", md))
                .collect::<Vec<_>>();
            examples.sort();
            examples.join("\n")
        }
    );
    std::fs::File::create(summary)
        .expect("file access error")
        .write_all(code.as_bytes())
        .expect("write error");

    Ok(())
}
