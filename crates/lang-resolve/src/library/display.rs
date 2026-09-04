// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{DisplayOneLine, VersionAnnotation};

use crate::{
    Library, Manifest, SymbolDef,
    library::{FileModule, LibraryRoot, Symbol},
};

impl std::fmt::Display for Manifest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", toml::to_string_pretty(&self).unwrap_or_default())
    }
}

impl DisplayOneLine for Manifest {
    fn to_string_one_line(&self) -> String {
        format!(
            "{name} {version}",
            name = self.library.name,
            version = self.library.version
        )
    }
}

impl std::fmt::Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root())
    }
}

impl DisplayOneLine for Library {}

impl std::fmt::Display for LibraryRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.manifest {
            Some(manifest) => {
                writeln!(f, "{}", manifest.to_string_one_line())?;
            }
            None => {}
        }

        Ok(())
    }
}

impl DisplayOneLine for LibraryRoot {}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let doc = self.doc.to_string_one_line();
        if !doc.is_empty() {
            writeln!(f, "{doc}")?;
        }
        let ver = self.ver.to_string_one_line();
        if self.ver != VersionAnnotation::default() && !ver.is_empty() {
            writeln!(f, "{ver}")?;
        }
        let meta = self.meta.to_string();
        if !meta.is_empty() {
            write!(f, "{meta} ")?;
        }
        writeln!(f, "{def}", def = self.def.to_string_one_line())
    }
}

impl DisplayOneLine for Symbol {
    fn to_string_one_line(&self) -> String {
        format!(
            "{meta}{def} {ver}",
            meta = {
                let meta = self.meta.to_string();
                if meta.is_empty() {
                    String::new()
                } else {
                    format!("{meta} ")
                }
            },
            def = self.def.to_string_one_line(),
            ver = {
                let ver = self.ver.to_string_one_line();
                if self.ver != VersionAnnotation::default() && !ver.is_empty() {
                    format!(" [{ver}]")
                } else {
                    String::new()
                }
            }
        )
    }
}

impl std::fmt::Display for FileModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_one_line())
    }
}

impl DisplayOneLine for FileModule {
    fn to_string_one_line(&self) -> String {
        match self {
            FileModule::NotLoaded => format!("NotLoaded"),
            FileModule::Loaded { path, .. } => format!("Loaded({})", path.display()),
        }
    }
}

impl std::fmt::Display for SymbolDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &'static str = self.into(); // "Function"
        write!(f, "{name}")?;

        write!(
            f,
            "{}",
            match self {
                SymbolDef::Root(library_root) => library_root.to_string_one_line(),
                SymbolDef::FileModule(file_module) => file_module.to_string_one_line(),
                SymbolDef::Workbench(workbench) => workbench.to_string_one_line(),
                SymbolDef::Function(function) => function.to_string_one_line(),
                SymbolDef::Constant(constant) => constant.to_string_one_line(),
                SymbolDef::Alias(alias) => alias.to_string_one_line(),
                SymbolDef::Wildcard(wildcard) => wildcard.to_string_one_line(),
                SymbolDef::Source(_) | SymbolDef::InlineModule(_) => String::new(),
            }
        )
    }
}

impl DisplayOneLine for SymbolDef {}
