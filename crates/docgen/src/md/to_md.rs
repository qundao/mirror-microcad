// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Microcad micro markdown parser and writer

use microcad_lang_markdown::{Markdown, Paragraph, Section};
use microcad_package::{
    SymbolDef, SymbolNodeRef,
    symbol::{self, SymbolNodeExt},
};

/// Helper function to parse markdown from a string, but any occurring parse error will lead to a panic.
///
/// It is assumed that if the generated markdown is incorrect, it is an error in the generator, not on content side.
fn parse(input: String) -> Markdown {
    microcad_lang_markdown::parse(&input)
        .expect("Internal Logic Error: Expected valid generated markdown.")
}

/// Trait to fetch markdown from a symbol definition.
pub(crate) trait ToMd {
    fn to_md(&self) -> Markdown;
}

impl ToMd for symbol::ParameterList {
    fn to_md(&self) -> Markdown {
        if self.is_empty() {
            Markdown::default()
        } else {
            parse(format!(
                "# Parameters\n{}",
                self.iter()
                    .map(|param| format!(
                        "## {}\n{}",
                        param.id,
                        match &param.doc {
                            Some(doc) => doc,
                            None => "",
                        }
                    ))
                    .collect::<Vec<String>>()
                    .join("\n")
            ))
        }
    }
}

impl<'a> ToMd for SymbolNodeRef<'a> {
    fn to_md(&self) -> Markdown {
        // Print one line description of a workbench
        fn symbol_one_line_item<'a>(symbol: SymbolNodeRef<'a>) -> Option<String> {
            if let Some(id) = symbol.id() {
                let link = format!(
                    "- [`{id}`]({filename})",
                    filename = match symbol.def {
                        SymbolDef::InlineModule(_) | SymbolDef::Source(_) => format!("./{id}"),
                        _ => format!("./{id}.md"),
                    }
                );
                symbol
                    .doc()
                    .and_then(|doc| doc.lines().next())
                    .map(|line| format!("{link}: {line}"))
            } else {
                None
            }
        }

        use microcad_package::symbol::WorkbenchKind;
        fn symbol_list<'a, P>(symbol: SymbolNodeRef<'a>, md: &mut Markdown, heading: &str, p: P)
        where
            P: FnMut(&SymbolNodeRef<'a>) -> bool,
        {
            let symbols: Vec<_> = symbol
                .children()
                .filter(|symbol| symbol.is_public())
                .filter(p)
                .collect();
            if !symbols.is_empty() {
                md.add_section(Section {
                    heading: heading.to_string(),
                    level: 2,
                    content: vec![Paragraph::Text(
                        symbols
                            .into_iter()
                            .filter_map(symbol_one_line_item)
                            .collect::<Vec<_>>()
                            .join("\n"),
                    )],
                });
            }
        }

        let mut md = Markdown::default();

        match (self.id(), self.doc()) {
            (Some(id), Some(doc)) => {
                md = parse(format!("# {id}\n{doc}"));
            }
            _ => {}
        };

        {
            // Generate list of sub-modules
            symbol_list(*self, &mut md, "Sub-modules", |symbol| {
                matches!(
                    symbol.def(),
                    SymbolDef::InlineModule(_) | SymbolDef::Source(_)
                )
            });

            // Generate list of sketches
            symbol_list(*self, &mut md, "Sketches", |symbol| {
                matches!(
                    symbol.def(),
                    SymbolDef::Workbench(workbench_definition) if
                        matches!(&workbench_definition.kind.value, WorkbenchKind::Sketch)
                )
            });

            // Parts
            symbol_list(*self, &mut md, "Parts", |symbol| {
                matches!(
                    symbol.def(),
                    SymbolDef::Workbench(workbench_definition) if
                        matches!(&workbench_definition.kind.value, WorkbenchKind::Part)
                )
            });

            // Operations
            symbol_list(*self, &mut md, "Operations", |symbol| {
                matches!(
                    symbol.def(),
                    SymbolDef::Workbench(workbench_definition) if
                        matches!(&workbench_definition.kind.value, WorkbenchKind::Op)
                )
            });

            fn inline_symbol_md<'a, P>(
                symbol: SymbolNodeRef<'a>,
                md: &mut Markdown,
                heading: &str,
                p: P,
            ) where
                P: FnMut(&SymbolNodeRef<'a>) -> bool,
            {
                let symbols: Vec<_> = symbol
                    .children()
                    .filter(|symbol| symbol.is_public())
                    .filter(p)
                    .collect();
                if !symbols.is_empty() {
                    md.add_section(Section {
                        heading: heading.to_string(),
                        level: 2,
                        content: vec![],
                    });
                    symbols.iter().for_each(|symbol| md.nest(symbol.to_md(), 2));
                }
            }

            // Functions
            inline_symbol_md(*self, &mut md, "Functions", |symbol| {
                matches!(symbol.def(), SymbolDef::Function(_))
            });

            // Constants
            {
                let constants: Vec<_> = self
                    .children()
                    .filter_map(|symbol| match (symbol.id(), symbol.def()) {
                        (Some(id), SymbolDef::Constant(constant)) => {
                            Some((id.clone(), constant.value.value.clone()))
                        }
                        _ => None,
                    })
                    .collect();

                if !constants.is_empty() {
                    md.add_section(Section {
                        heading: "Constants".to_string(),
                        level: 2,
                        content: vec![Paragraph::Text(
                            constants
                                .into_iter()
                                .map(|(identifier, value)| format!("- `{identifier}` = `{value}`"))
                                .collect::<Vec<_>>()
                                .join("\n"),
                        )],
                    });
                }
            }

            // Aliases
            {
                let aliases: Vec<_> = self
                    .children()
                    .filter_map(|symbol| match symbol.def() {
                        SymbolDef::Alias(alias) if symbol.id().is_some() => {
                            Some((symbol.id().cloned().unwrap(), alias.0.clone()))
                        }
                        _ => None,
                    })
                    .collect();

                if !aliases.is_empty() {
                    md.add_section(Section {
                        heading: "Aliases".to_string(),
                        level: 2,
                        content: vec![Paragraph::Text(
                            aliases
                                .into_iter()
                                .map(|(identifier, name)| format!("- `{identifier}` => `{name}`"))
                                .collect::<Vec<_>>()
                                .join("\n"),
                        )],
                    });
                }
            }
        }

        md
    }
}
