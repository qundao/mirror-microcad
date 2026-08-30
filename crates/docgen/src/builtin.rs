// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate documentation as markdown book for µcad compiler built-ins.

use microcad_builtin::{
    BuiltinConstant, BuiltinFunction, BuiltinItem, BuiltinModule, BuiltinOperation,
    BuiltinPrimitive, BuiltinRegistry,
};
use microcad_lang_markdown as md;
use microcad_lang_markdown::{
    WriteToFile,
    table_builder::{Column, TableBuilder},
};

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
            name = self.info.item_name(),
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
            name = self.info.item_name(),
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

impl ToMd for BuiltinItem {
    fn to_md(&self) -> microcad_lang_markdown::Markdown {
        match &self {
            BuiltinItem::Constant(builtin_constant) => builtin_constant.to_md(),
            BuiltinItem::Function(builtin_function) => builtin_function.to_md(),
            BuiltinItem::Primitive(builtin_primitive) => builtin_primitive.to_md(),
            BuiltinItem::Operation(builtin_operation) => builtin_operation.to_md(),
            BuiltinItem::Module(builtin_module) => builtin_module.to_md(),
        }
    }
}

impl ToMd for BuiltinModule {
    fn to_md(&self) -> md::Markdown {
        md::md!(
            r#"# {name}

            {doc}

            {constants}
            {functions}
            {primitives}
            {operations}
            "#,
            name = self.info.name,
            doc = self.info.doc.map(|doc| doc.to_string()).unwrap_or_default(),
            constants = item_table(
                "Constants",
                "Constant",
                self.constants().map(|item| {
                    [
                        BuiltinMdbook::item_link(item.clone()),
                        item.info.doc_summary().to_string(),
                    ]
                })
            ),
            functions = item_table(
                "Functions",
                "Function",
                self.functions().map(|item| {
                    [
                        BuiltinMdbook::item_link(item.clone()),
                        item.info.doc_summary().to_string(),
                    ]
                })
            ),
            primitives = item_table(
                "Primitives",
                "Primitive",
                self.primitives().map(|item| {
                    [
                        BuiltinMdbook::item_link(item.clone()),
                        item.info.doc_summary().to_string(),
                    ]
                })
            ),
            operations = item_table(
                "Operations",
                "Operation",
                self.operations().map(|item| {
                    [
                        BuiltinMdbook::item_link(item.clone()),
                        item.info.doc_summary().to_string(),
                    ]
                })
            )
        )
    }
}

fn item_table<I, R, S>(header: &str, item_kind: &str, items: I) -> String
where
    I: IntoIterator<Item = R>,
    R: IntoIterator<Item = S>,
    S: Into<String>,
{
    let table = TableBuilder::new()
        .columns([Column::left(item_kind), Column::left("Summary")])
        .rows(items);

    if table.is_empty() {
        String::new()
    } else {
        format!("## {header}\n{table}")
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

        let md = md::md!(
            r#"
            # `__mu`: µcad Compiler built-in library

            {doc}

            {modules}
            "#,
            doc = microcad_builtin::mu::__MU,
            modules = item_table(
                "Modules",
                "Module",
                self.registry.modules().map(|module| {
                    [
                        Self::mod_link(module),
                        module.info.doc_summary().to_string(),
                    ]
                })
            )
        );

        self.registry.modules().for_each(|module| {
            let mod_name = module.info.item_name();
            let md_path = src_path.join(mod_name);
            mdbook.add_md(md_path.join("README.md"), module.to_md());

            module.items.iter().for_each(|item| {
                let item_name = item.info().item_name();
                mdbook.add_md(md_path.join(format!("{item_name}.md")), item.to_md());
            })
        });

        mdbook.add_md(src_path.join("README.md"), md);

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

    fn mod_link(module: &BuiltinModule) -> String {
        let name = module.info.item_name();
        format!("[{name}]({name}/README.md)")
    }

    fn item_link(item: impl Into<BuiltinItem>) -> String {
        let item = item.into();
        let name = item.info().item_name();
        format!("[{name}]({name}.md)")
    }

    fn summary(&self) -> Summary {
        let mut entries = Vec::new();
        use crate::mdbook::SummaryEntry;
        entries.push(SummaryEntry::new(0, "__mu", "README.md"));

        self.registry.modules().for_each(|module| {
            let mod_name = module.info.item_name();
            let md_path = format!("{mod_name}/README.md");
            entries.push(SummaryEntry::new(1, mod_name, &md_path));

            module.items.iter().for_each(|item| {
                let item_name = item.info().item_name();
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
