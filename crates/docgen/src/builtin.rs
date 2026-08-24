// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate documentation as markdown book for µcad compiler built-ins.

use microcad_builtin::{
    Builtin, BuiltinConstant, BuiltinFunction, BuiltinModule, BuiltinOperation, BuiltinPrimitive,
    BuiltinRegistry,
};
use microcad_lang_markdown::{self as md, WriteToFile};

use crate::{md::ToMd, mdbook::Summary};

#[derive(Debug, Default)]
pub struct BuiltinMdbook {
    /// Registered builtins,
    registry: BuiltinRegistry,
}

impl ToMd for BuiltinConstant {
    fn to_md(&self) -> md::Markdown {
        md::md!(
            "# {name}\n{doc}",
            name = self.info.item_name().unwrap(),
            doc = self
                .info
                .doc
                .map(|doc| format!(
                    "**{name} = {value}**\n{doc}",
                    name = self.info.name,
                    value = self.value(),
                ))
                .unwrap_or_default()
        )
    }
}

impl ToMd for BuiltinFunction {
    fn to_md(&self) -> md::Markdown {
        md::md!(
            "# Function {name} {{#{name}}} \n{doc}",
            name = self.info.item_name().unwrap(),
            doc = self
                .info
                .doc
                .map(|doc| format!(
                    "> `{name}{ty}`\n\n{doc}",
                    name = self.info.name,
                    ty = self.ty()
                ))
                .unwrap_or_default()
        )
    }
}

impl ToMd for BuiltinPrimitive {
    fn to_md(&self) -> md::Markdown {
        md::md!(
            "# {name}\n{doc}",
            name = self.info.name,
            doc = self.info.doc.map(|doc| doc.to_string()).unwrap_or_default()
        )
    }
}

impl ToMd for BuiltinOperation {
    fn to_md(&self) -> md::Markdown {
        md::md!(
            "# {name}\n{doc}",
            name = self.info.name,
            doc = self.info.doc.map(|doc| doc.to_string()).unwrap_or_default()
        )
    }
}

impl ToMd for Builtin {
    fn to_md(&self) -> microcad_lang_markdown::Markdown {
        match &self {
            Builtin::Constant(builtin_constant) => builtin_constant.to_md(),
            Builtin::Function(builtin_function) => builtin_function.to_md(),
            Builtin::Primitive(builtin_primitive) => builtin_primitive.to_md(),
            Builtin::Operation(builtin_operation) => builtin_operation.to_md(),
            Builtin::Module(builtin_module) => builtin_module.to_md(),
        }
    }
}

impl ToMd for BuiltinModule {
    fn to_md(&self) -> md::Markdown {
        let mut md = md::md!(
            "# {name}\n{doc}",
            name = self.info.name,
            doc = self.info.doc.map(|doc| doc.to_string()).unwrap_or_default()
        );

        self.items.iter().for_each(|item| md.nest(item.to_md(), 1));

        md
    }
}

impl BuiltinMdbook {
    pub fn new() -> Self {
        Self {
            registry: BuiltinRegistry::new(),
        }
    }

    pub fn to_mdbook(&self) -> md::MdBook {
        let mut mdbook = md::MdBook::new("__mu");
        let src_path = std::path::PathBuf::from("src");

        mdbook.add_md(
            src_path.join("README.md"),
            md::md!(
                "# `__mu`: µcad Compiler built-in library\n{doc}",
                doc = microcad_builtin::mu::__MU
            ),
        );

        self.registry.modules().for_each(|module| {
            let mod_name = module.info.item_name().unwrap();
            let md_path = src_path.join(mod_name);
            mdbook.add_md(md_path.join("README.md"), module.to_md());

            module.items.iter().for_each(|item| {
                let item_name = item.info().item_name().unwrap();
                mdbook.add_md(md_path.join(format!("{item_name}.md")), item.to_md());
            })
        });

        mdbook
    }

    pub fn write(&self, path: impl AsRef<std::path::Path>) -> Result<(), md::MdBookError> {
        let path = path.as_ref();
        std::fs::create_dir_all(&path)?;
        crate::mdbook::Config.write_to_file(path.join("book.toml"))?;

        let mdbook = self.to_mdbook();
        self.summary()
            .write_to_file(path.join(&mdbook.src_path).join("SUMMARY.md"))?;

        mdbook.save_all(path)
    }

    fn summary(&self) -> Summary {
        let mut entries = Vec::new();
        use crate::mdbook::SummaryEntry;

        entries.push(SummaryEntry::new(0, "__mu", "README.md"));

        self.registry.modules().for_each(|module| {
            let mod_name = module.info.item_name().unwrap();
            let md_path = format!("{mod_name}/README.md");
            entries.push(SummaryEntry::new(1, mod_name, &md_path));

            module.items.iter().for_each(|item| {
                let item_name = item.info().item_name().unwrap();
                entries.push(SummaryEntry::new(
                    2,
                    item_name,
                    format!("{mod_name}/{item_name}.md"),
                ));
            });
        });

        Summary::from_iter(entries)
    }
}
