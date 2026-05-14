// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod builtin;
mod markdown;
mod mdbook;
mod source;

use derive_more::From;
use microcad_builtin::Symbol;

use microcad_lang_base::{DiagRenderOptions, Diagnostics, RcMut, ResourceLocation, Url};
pub use source::Source;

use crate::{Model, Result, commands};

/// Return a symbol
pub trait GetSymbol {
    fn get_symbol(&self) -> Result<Symbol>;
}

pub trait TryFilePath: ResourceLocation {
    fn try_file_path(&self) -> Result<std::path::PathBuf> {
        match self.to_file_path() {
            Some(path) => Ok(path),
            None => Err(miette::miette!("No local path: {}", self.url())),
        }
    }
}

pub trait CaptureDiags {
    fn diags(&self) -> RcMut<Diagnostics>;

    /// Internal helper to "capture" errors into the local diagnostics collection.
    fn capture_diags<T, E>(&self, diags: std::result::Result<T, E>) -> Option<T>
    where
        E: Into<Diagnostics>,
    {
        match diags {
            Ok(val) => Some(val),
            Err(err_rc) => {
                self.diags().replace(err_rc.into());
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
    Source(Source),

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
    pub fn new(url: Url) -> Result<Self> {
        let path = url.path();
        if path.ends_with("/book.toml") {
            Ok(MdBook::new(url).into())
        } else if path.ends_with(".md") {
            Ok(Markdown::new(url).into())
        } else if url.scheme() == "builtin" {
            Ok(Builtin::new().into())
        } else if url.scheme() == "file" {
            Ok(Source::new(url).into())
        } else {
            Err(miette::miette!("Invalid document type: {}", url.path()))
        }
    }

    /// Load a document from a url.
    pub fn load(url: Url) -> Result<Self> {
        use commands::LoadFromFile;
        let mut document = Self::new(url)?;
        document.load_from_file()?;
        Ok(document)
    }

    /// Open a document from a location str.
    pub fn open(location: impl AsRef<str>) -> Result<Self> {
        Self::load(crate::locate::to_url(location.as_ref())?)
    }

    /// Load a document from file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        use commands::LoadFromFile;
        use miette::IntoDiagnostic;
        let absolute_path = std::fs::canonicalize(&path).into_diagnostic()?;
        let url = Url::from_file_path(&absolute_path).map_err(|_| {
            miette::miette!(
                "URL {path} does not contain a file path!",
                path = path.as_ref().display()
            )
        })?;
        let mut document = Self::new(url)?;
        document.load_from_file()?;
        Ok(document)
    }
}

impl CaptureDiags for Document {
    fn diags(&self) -> RcMut<Diagnostics> {
        match self {
            Document::Source(i) => i.diags(),
            Document::Markdown(i) => i.diags(),
            Document::MdBook(i) => i.diags(),
            Document::Builtin(i) => i.diags(),
        }
    }
}

impl ResourceLocation for Document {
    fn url(&self) -> &Url {
        match self {
            Document::Source(u) => &u.url(),
            Document::Markdown(u) => &u.url(),
            Document::MdBook(u) => &u.url(),
            Document::Builtin(u) => &u.url(),
        }
    }
}

impl commands::LoadFromFile for Document {
    fn load_from_file(&mut self) -> Result {
        match self {
            Document::Source(item) => item.load_from_file(),
            Document::Markdown(item) => item.load_from_file(),
            Document::MdBook(item) => item.load_from_file(),
            _ => unimplemented!(),
        }
    }
}

impl commands::compile::Parse for Document {
    fn parse(&mut self) -> Result {
        match self {
            Document::Source(source) => source.parse(),
            _ => unimplemented!(),
        }
    }
}

impl commands::compile::Lower for Document {
    fn lower(&mut self) -> Result {
        match self {
            Document::Source(source) => source.lower(),
            _ => unimplemented!(),
        }
    }
}

impl commands::compile::Resolve for Document {
    fn resolve(&mut self, parameters: impl Into<commands::compile::ResolveParameters>) -> Result {
        match self {
            Document::Source(source) => source.resolve(parameters),
            _ => unimplemented!(),
        }
    }
}

impl commands::compile::Eval for Document {
    fn eval(&mut self) -> Result {
        match self {
            Document::Source(source) => source.eval(),
            _ => unimplemented!(),
        }
    }
}

impl commands::compile::Render for Document {
    fn render(&mut self, params: impl Into<commands::compile::RenderParameters>) -> Result<Model> {
        match self {
            Document::Source(source) => source.render(params),
            _ => unimplemented!(),
        }
    }
}

impl commands::Compile for Document {}

impl commands::Export for Document {
    fn get_export_targets(
        &self,
        params: &commands::ExportParameters,
    ) -> Result<commands::ExportTargets> {
        match self {
            Document::Source(source) => source.get_export_targets(params),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(_) => todo!(),
        }
    }
}

impl commands::Format for Document {
    fn format(&mut self, params: &commands::FormatParameters) -> Result<bool> {
        match self {
            Document::Source(item) => item.format(params),
            Document::Markdown(item) => item.format(params),
            Document::MdBook(item) => item.format(params),
            _ => unimplemented!(),
        }
    }
}

impl commands::Sync for Document {
    fn sync(&self) -> Result {
        match &self {
            Document::Source(source) => source.sync(),
            Document::Markdown(markdown) => markdown.sync(),
            Document::MdBook(mdbook) => mdbook.sync(),
            _ => unimplemented!(),
        }
    }
}

impl GetSymbol for Document {
    fn get_symbol(&self) -> Result<Symbol> {
        match &self {
            Document::Source(asset) => asset.get_symbol(),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(asset) => asset.get_symbol(),
        }
    }
}

impl commands::DocGen for Document {
    fn doc_gen(&self, params: &commands::DocGenParameters) -> Result {
        match &self {
            Document::Source(asset) => asset.doc_gen(params),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(asset) => asset.doc_gen(params),
        }
    }
}

impl commands::PrintDiagnostics for Document {
    fn print_diagnostics(
        &self,
        f: &mut dyn std::fmt::Write,
        options: &DiagRenderOptions,
    ) -> std::fmt::Result {
        match self {
            Document::Source(i) => i.print_diagnostics(f, options),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(_) => todo!(),
        }
    }
}
