// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod builtin;
mod markdown;
mod mdbook;
mod source_file;
mod stdin;

use derive_more::From;

use microcad_lang_base::{DiagRenderOptions, Diagnostics, SourceKind, Url};
pub use source_file::SourceFile;
pub use stdin::Stdin;

use crate::prelude as mu;

/// Return the symbol for document
pub trait GetSymbol {
    fn get_symbol(
        &mut self,
        parameters: impl Into<mu::ResolveParameters>,
    ) -> mu::Result<mu::Symbol>;
}

pub trait CaptureDiags {
    fn diags(&self) -> &Diagnostics;

    fn diags_mut(&mut self) -> &mut Diagnostics;

    /// Internal helper to "capture" errors into the local diagnostics collection.
    fn capture_diags<T, E>(&mut self, diags: std::result::Result<T, E>) -> Option<T>
    where
        E: Into<Diagnostics>,
    {
        match diags {
            Ok(val) => Some(val),
            Err(diags) => {
                self.diags_mut().append(diags.into());
                None
            }
        }
    }
}

pub type Markdown = markdown::MarkdownDocument;
pub type MdBook = mdbook::MdBookDocument;
pub type Builtin = builtin::Builtin;

/// A document containing µcad code.
#[derive(From)]
pub enum Document {
    /// A single source file
    SourceFile(Box<SourceFile>),

    /// A markdown file containing source code snippets
    Markdown(Markdown),

    /// An `book.toml` of a markdown book
    MdBook(MdBook),

    /// A builtin symbol
    Builtin(Builtin),
}

impl Document {
    /// Create a new document
    ///
    /// If the URL ends with:
    /// * `.µcad`/`.mcad`/`.ucad`: Create a source file
    /// * `.md`: Create a markdown
    /// * `book.toml`: Create an MdBook
    ///
    /// If the URLs scheme is `builtin`, create a built-in document.
    pub fn new(url: Url) -> mu::Result<Self> {
        let path = url.path();
        if path.ends_with("/book.toml") {
            Ok(MdBook::new(url)?.into())
        } else if path.ends_with(".md") {
            Ok(Markdown::new(SourceKind::from(url))?.into())
        } else if url.scheme() == "builtin" {
            Ok(Builtin::new().into())
        } else if url.scheme() == "file" {
            Ok(Box::new(SourceFile::load_from_file(url, 0)?).into())
        } else {
            Err(miette::miette!("Invalid document type: {}", url.path()))
        }
    }

    /// Open a document from a location str.
    pub fn open(location: impl AsRef<str>) -> mu::Result<Self> {
        Self::new(crate::locate::to_url(location.as_ref())?)
    }
}

impl mu::commands::GetCode for Document {
    fn get_code(&self) -> Option<&str> {
        match self {
            Self::SourceFile(source_file) => source_file.get_code(),
            Self::Markdown(_) | Self::MdBook(_) | Self::Builtin(_) => None,
        }
    }
}

impl mu::commands::SetCode for Document {
    fn set_code(&mut self, new_code: String) -> Option<&str> {
        match self {
            Self::SourceFile(source_file) => source_file.set_code(new_code),
            Self::Markdown(_) | Self::MdBook(_) | Self::Builtin(_) => None,
        }
    }
}

impl CaptureDiags for Document {
    fn diags(&self) -> &Diagnostics {
        match self {
            Document::SourceFile(i) => i.diags(),
            Document::Markdown(i) => i.diags(),
            Document::MdBook(i) => i.diags(),
            Document::Builtin(i) => i.diags(),
        }
    }

    fn diags_mut(&mut self) -> &mut Diagnostics {
        match self {
            Document::SourceFile(i) => i.diags_mut(),
            Document::Markdown(i) => i.diags_mut(),
            Document::MdBook(i) => i.diags_mut(),
            Document::Builtin(i) => i.diags_mut(),
        }
    }
}

impl mu::commands::compile::Parse for Document {
    fn parse(&mut self) -> mu::Result {
        match self {
            Document::SourceFile(source) => source.parse(),
            _ => unimplemented!(),
        }
    }
}

impl mu::commands::compile::Lower for Document {
    fn lower(&mut self) -> mu::Result {
        match self {
            Document::SourceFile(source) => source.lower(),
            _ => unimplemented!(),
        }
    }
}

impl mu::commands::compile::Resolve for Document {
    fn resolve(
        &mut self,
        parameters: impl Into<mu::commands::compile::ResolveParameters>,
    ) -> mu::Result<mu::Symbol> {
        match self {
            Document::SourceFile(source) => source.resolve(parameters),
            _ => unimplemented!(),
        }
    }
}

impl mu::commands::compile::Eval for Document {
    fn eval(&mut self) -> mu::Result<mu::Model> {
        match self {
            Document::SourceFile(source) => source.eval(),
            _ => unimplemented!(),
        }
    }
}

impl mu::commands::Render for Document {
    fn render(
        &mut self,
        params: impl Into<mu::commands::RenderParameters>,
    ) -> mu::Result<mu::Model> {
        match self {
            Document::SourceFile(source) => source.render(params),
            _ => unimplemented!(),
        }
    }
}

impl mu::commands::Compile for Document {}

impl mu::commands::Export for Document {
    fn get_export_targets(
        &self,
        params: impl Into<mu::commands::ExportParameters>,
    ) -> mu::Result<mu::commands::ExportTargets> {
        match self {
            Document::SourceFile(source) => source.get_export_targets(params),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(_) => todo!(),
        }
    }
}

impl mu::commands::Format for Document {
    fn format(&mut self, params: &mu::commands::FormatParameters) -> mu::Result<bool> {
        match self {
            Document::SourceFile(item) => item.format(params),
            Document::Markdown(item) => item.format(params),
            Document::MdBook(item) => item.format(params),
            _ => unimplemented!(),
        }
    }
}

impl mu::commands::Sync for Document {
    fn sync(&self) -> mu::Result {
        match &self {
            Document::SourceFile(source) => source.sync(),
            Document::Markdown(markdown) => markdown.sync(),
            Document::MdBook(mdbook) => mdbook.sync(),
            _ => unimplemented!(),
        }
    }
}

impl GetSymbol for Document {
    fn get_symbol(
        &mut self,
        params: impl Into<mu::commands::compile::ResolveParameters>,
    ) -> mu::Result<mu::Symbol> {
        match self {
            Document::SourceFile(asset) => asset.get_symbol(params),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(asset) => asset.get_symbol(params),
        }
    }
}

impl mu::commands::DocGen for Document {
    fn doc_gen(&mut self, params: impl Into<mu::commands::DocGenParameters>) -> mu::Result {
        match self {
            Document::SourceFile(asset) => asset.doc_gen(params),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(asset) => asset.doc_gen(params),
        }
    }
}

impl mu::commands::PrintDiagnostics for Document {
    fn print_diagnostics(
        &self,
        f: &mut dyn std::fmt::Write,
        options: &DiagRenderOptions,
    ) -> std::fmt::Result {
        match self {
            Document::SourceFile(i) => i.print_diagnostics(f, options),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(_) => todo!(),
        }
    }
}
