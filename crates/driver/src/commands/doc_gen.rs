// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Result, commands::compile::ResolveParameters, document};

pub struct DocGenParameters {
    pub generator_id: Option<String>,
    pub output_path: Option<std::path::PathBuf>,
    pub resolve_parameters: ResolveParameters,
}

impl DocGenParameters {
    fn generator(&self) -> Result<Box<dyn microcad_docgen::DocGen>> {
        let name = self.generator_id.clone().unwrap_or("md".to_string());
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

pub trait DocGen: document::GetSymbol {
    fn doc_gen(&mut self, params: impl Into<DocGenParameters>) -> Result {
        let p = params.into();
        let generator = p.generator()?;
        let symbol = self.get_symbol(p.resolve_parameters)?;
        generator
            .doc_gen(&symbol)
            .map_err(|err| miette::miette!("{err}"))
    }
}
