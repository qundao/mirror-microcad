// Copyright © 2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Microcad micro markdown parser and writer

use microcad_lang::{
    builtin::{BuiltinKind, BuiltinWorkbenchKind},
    doc::Doc,
    resolve::*,
    syntax::{
        FunctionDefinition, Identifiable, InitDefinition, Initialized, ModuleDefinition,
        ParameterList, SourceFile, Visibility, WorkbenchDefinition,
    },
};

use crate::md::{self, Markdown, Section};

/// Add an extra `#` to each heading line.
fn indent_header_lines(lines: Vec<String>) -> Vec<String> {
    lines
        .into_iter()
        .map(|s| {
            if s.starts_with("#") {
                format!("#{s}")
            } else {
                s
            }
        })
        .collect()
}

/// Fetch documentation as string with indented headers. (Markdown hack)
fn fetch_doc(doc: &impl Doc) -> String {
    indent_header_lines(doc.doc().fetch_lines()).join("\n")
}

/// Trait to fetch markdown.
pub trait ToMd {
    fn to_md(&self) -> md::Markdown;
}

impl ToMd for InitDefinition {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!("# {}\n{}", self.signature(), fetch_doc(self)))
    }
}

impl ToMd for SourceFile {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!("# {}\n{}", self.id(), fetch_doc(self)))
    }
}

impl ToMd for FunctionDefinition {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!("# {}\n{}", self.id(), fetch_doc(self)))
    }
}

impl ToMd for ModuleDefinition {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!("# {}\n{}", self.id(), fetch_doc(self)))
    }
}

impl ToMd for ParameterList {
    fn to_md(&self) -> md::Markdown {
        if self.is_empty() {
            md::Markdown::default()
        } else {
            md::Markdown::new(&format!(
                "# Parameters\n{}",
                self.iter()
                    .map(|param| format!("- {}", param.to_string()))
                    .collect::<Vec<String>>()
                    .join("\n")
            ))
        }
    }
}

impl ToMd for WorkbenchDefinition {
    fn to_md(&self) -> md::Markdown {
        let mut md = md::Markdown::new(&format!("# {}\n{}", self.id(), fetch_doc(self)));
        md.nest(self.plan.to_md(), 1);

        self.inits().for_each(|init| {
            md.nest(init.to_md(), 1);
        });

        md
    }
}

impl ToMd for microcad_lang::builtin::Builtin {
    fn to_md(&self) -> md::Markdown {
        md::Markdown::new(&format!("# {}\n{}", self.id(), fetch_doc(self)))
    }
}

impl ToMd for SymbolDef {
    fn to_md(&self) -> md::Markdown {
        match &self {
            SymbolDef::SourceFile(source_file) => source_file.to_md(),
            SymbolDef::Module(module_definition) => module_definition.to_md(),
            SymbolDef::Workbench(workbench_definition) => workbench_definition.to_md(),
            SymbolDef::Function(function_definition) => function_definition.to_md(),
            SymbolDef::Builtin(builtin) => builtin.to_md(),
            _ => md::Markdown::default(),
        }
    }
}

impl ToMd for Symbol {
    fn to_md(&self) -> md::Markdown {
        // Print one line description of a workbench
        fn symbol_one_line_item(symbol: &Symbol) -> String {
            let id = symbol.id();
            let link = format!(
                "- [`{id}`]({filename})",
                filename = symbol.with_def(|def| match def {
                    SymbolDef::Module(_) | SymbolDef::SourceFile(_) => format!("./{id}"),
                    _ => format!("./{id}.md"),
                })
            );
            match symbol.doc().fetch_lines().first() {
                Some(line) => format!("{link}: {line}"),
                None => link,
            }
        }

        use microcad_lang::syntax::WorkbenchKind;
        fn symbol_list<P>(symbol: &Symbol, md: &mut Markdown, heading: &str, p: P)
        where
            P: FnMut(&Symbol) -> bool,
        {
            let symbols: Vec<_> = symbol
                .iter()
                .filter(|symbol| symbol.is_public())
                .filter(p)
                .collect();
            if !symbols.is_empty() {
                md.add_section(Section {
                    heading: heading.to_string(),
                    level: 2,
                    content: symbols.iter().map(symbol_one_line_item).collect(),
                });
            }
        }

        let mut md = self.with_def(|def| def.to_md());

        {
            // Generate list of sub-modules
            symbol_list(self, &mut md, "Sub-modules", |symbol| {
                symbol
                    .with_def(|def| matches!(def, SymbolDef::Module(_) | SymbolDef::SourceFile(_)))
            });

            // Generate list of sketches
            symbol_list(self, &mut md, "Sketches", |symbol| {
                symbol.with_def(|def| match def {
                    SymbolDef::Workbench(workbench_definition) => {
                        matches!(&workbench_definition.kind.value, WorkbenchKind::Sketch)
                    }
                    _ => false,
                })
            });

            // Parts
            symbol_list(self, &mut md, "Parts", |symbol| {
                symbol.with_def(|def| match def {
                    SymbolDef::Workbench(workbench_definition) => {
                        matches!(&workbench_definition.kind.value, WorkbenchKind::Part)
                    }
                    _ => false,
                })
            });

            // Operations
            symbol_list(self, &mut md, "Operations", |symbol| {
                symbol.with_def(|def| match def {
                    SymbolDef::Workbench(workbench_definition) => {
                        matches!(&workbench_definition.kind.value, WorkbenchKind::Operation)
                    }
                    _ => false,
                })
            });

            // Built-in 2D primitives
            symbol_list(self, &mut md, "Built-in 2D primitives", |symbol| {
                symbol.with_def(|def| -> bool {
                    match def {
                        SymbolDef::Builtin(builtin) => matches!(
                            &builtin.kind,
                            BuiltinKind::Workbench(BuiltinWorkbenchKind::Primitive2D)
                        ),
                        _ => false,
                    }
                })
            });

            // Built-in 3D primitives
            symbol_list(self, &mut md, "Built-in 3D primitives", |symbol| {
                symbol.with_def(|def| -> bool {
                    match def {
                        SymbolDef::Builtin(builtin) => matches!(
                            &builtin.kind,
                            BuiltinKind::Workbench(BuiltinWorkbenchKind::Primitive3D)
                        ),
                        _ => false,
                    }
                })
            });

            // Built-in operations
            symbol_list(self, &mut md, "Built-in operations", |symbol| {
                symbol.with_def(|def| -> bool {
                    match def {
                        SymbolDef::Builtin(builtin) => matches!(
                            &builtin.kind,
                            BuiltinKind::Workbench(BuiltinWorkbenchKind::Operation)
                        ),
                        _ => false,
                    }
                })
            });

            // Built-in transformations
            symbol_list(self, &mut md, "Built-in transformations", |symbol| {
                symbol.with_def(|def| -> bool {
                    match def {
                        SymbolDef::Builtin(builtin) => matches!(
                            &builtin.kind,
                            BuiltinKind::Workbench(BuiltinWorkbenchKind::Transform)
                        ),
                        _ => false,
                    }
                })
            });

            fn inline_symbol_md<P>(symbol: &Symbol, md: &mut Markdown, heading: &str, p: P)
            where
                P: FnMut(&Symbol) -> bool,
            {
                let symbols: Vec<_> = symbol
                    .iter()
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
            inline_symbol_md(self, &mut md, "Functions", |symbol| {
                symbol.with_def(|def| matches!(def, SymbolDef::Function(_)))
            });

            // Built-in functions
            inline_symbol_md(self, &mut md, "Built-in functions", |symbol| {
                symbol.with_def(|def| match def {
                    SymbolDef::Builtin(builtin) => matches!(builtin.kind, BuiltinKind::Function),
                    _ => false,
                })
            });

            // Constants
            {
                let constants: Vec<_> = self
                    .iter()
                    .filter_map(|symbol| {
                        symbol.with_def(|def| match def {
                            SymbolDef::Constant(Visibility::Public, identifier, value) => {
                                Some((identifier.clone(), value.clone()))
                            }
                            _ => None,
                        })
                    })
                    .collect();

                if !constants.is_empty() {
                    md.add_section(Section {
                        heading: "Constants".to_string(),
                        level: 2,
                        content: constants
                            .into_iter()
                            .map(|(identifier, value)| format!("- `{identifier}` = `{value}`"))
                            .collect(),
                    });
                }
            }

            // Aliases
            {
                let aliases: Vec<_> = self
                    .iter()
                    .filter_map(|symbol| {
                        symbol.with_def(|def| match def {
                            SymbolDef::Alias(Visibility::Public, identifier, name) => {
                                Some((identifier.clone(), name.clone()))
                            }
                            _ => None,
                        })
                    })
                    .collect();

                if !aliases.is_empty() {
                    md.add_section(Section {
                        heading: "Aliases".to_string(),
                        level: 2,
                        content: aliases
                            .into_iter()
                            .map(|(identifier, name)| format!("- `{identifier}` => `{name}`"))
                            .collect(),
                    });
                }
            }
        }

        md
    }
}
