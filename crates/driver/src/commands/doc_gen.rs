// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_docgen::{Md, MdBook};

use crate::commands::CommandResult;

pub struct DocGenSettings {
    generator: Option<String>,
    output_path: Option<std::path::PathBuf>,
}

impl DocGenSettings {
    fn generator(&self) -> miette::Result<Box<dyn microcad_docgen::DocGen>> {
        let name = self.generator.clone().unwrap_or("md".to_string());
        use microcad_docgen::*;
        match name.as_str() {
            "md" => Ok(Box::new(Md {
                output_path: self.output_path.clone(),
            })),
            "mdbook" => Ok(Box::new(MdBook {
                path: self.output_path.clone().unwrap_or_default(),
            })),
            _ => Err(miette::miette!("No generator with name `{name}`")),
        }
    }
}

pub trait DocGen {
    fn doc_gen(&self, settings: &DocGenSettings) -> CommandResult<()>;
}
