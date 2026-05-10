// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod builtin;
mod markdown;
mod mdbook;
mod source;

use std::{cell::RefCell, rc::Rc};

use derive_more::From;
use microcad_builtin::Symbol;
use microcad_lang_base::{
    DiagRenderOptions, Diagnostics, MICROCAD_EXTENSIONS, RcMut, ResourceLocation,
};
use url::Url;

use crate::{Config, commands};

pub type Result<T = ()> = std::result::Result<T, RcMut<Diagnostics>>;

/// A document asset with a state and diagnostics
pub struct Asset<S: Default> {
    /// Each asset must have a unique URL.
    url: Url,
    /// Each document item keeps its [Diagnostics]
    diagnostics: RcMut<Diagnostics>,
    /// Each document has a state.
    state: RefCell<S>,
}
/// Return a symbol
pub trait GetAssetSymbol {
    fn get_symbol(&self) -> Result<Symbol>;
}

impl<S: Default> Asset<S> {
    /// Create a new container
    fn new(url: Url) -> Rc<Self> {
        Rc::new(Self {
            url,
            diagnostics: RcMut::new(Default::default()),
            state: Default::default(),
        })
    }

    /// Generic transitioner to move the pipeline forward
    fn transition<F>(&self, f: F) -> Result
    where
        F: FnOnce(S) -> std::result::Result<S, Diagnostics>,
    {
        let mut state = self.state.borrow_mut();
        let current = std::mem::take(&mut *state);
        match f(current) {
            Ok(new_state) => {
                *state = new_state;
                Ok(())
            }
            Err(diag) => {
                self.diagnostics.replace(diag);
                Err(self.diagnostics.clone())
            }
        }
    }
}

impl<S: Default> ResourceLocation for Asset<S> {
    fn url(&self) -> &Url {
        &self.url
    }
}

pub type SourceAsset = Asset<source::State>;
pub type MarkdownAsset = Asset<markdown::State>;
pub type MdBookAsset = Asset<mdbook::State>;
pub type BuiltinAsset = Asset<builtin::State>;

/// A document containing µcad code.
#[derive(From)]
pub enum Document {
    /// A single source file
    Source(Rc<SourceAsset>),

    /// A markdown file containing source code snippets
    Markdown(Rc<MarkdownAsset>),

    /// An `book.toml` of a markdown book
    MdBook(Rc<MdBookAsset>),

    /// A builtin symbol
    Builtin(Rc<BuiltinAsset>),
}

impl Document {
    /// Create a new document
    ///
    /// If the URL ends with:
    /// * `.µcad`/`.mcad`/`.ucad`: Create a source file
    /// * `.md`: Create a markdown
    /// * `book.toml`: Create an MdBook
    pub fn new(url: Url) -> miette::Result<Self> {
        let path = std::path::Path::new(url.path());
        let file_name = path.file_name().and_then(|os| os.to_str()).unwrap_or("");
        let extension = path.extension().and_then(|os| os.to_str()).unwrap_or("");

        match file_name {
            "book.toml" => Ok(MdBookAsset::new(url).into()),
            _ => match extension {
                "md" => Ok(MarkdownAsset::new(url).into()),
                extension => {
                    if MICROCAD_EXTENSIONS.contains(&extension) {
                        Ok(SourceAsset::new(url).into())
                    } else {
                        Err(miette::miette!("Invalid document type: {extension}"))
                    }
                }
            },
        }
    }

    pub fn from_file_path(path: impl AsRef<std::path::Path>) -> miette::Result<Self> {
        let url = Url::from_file_path(path.as_ref())
            .map_err(|_| miette::miette!("URL does not contain a file path!"))?;
        Self::new(url)
    }

    pub fn diagnostics(&self) -> RcMut<Diagnostics> {
        match self {
            Document::Source(i) => i.diagnostics.clone(),
            Document::Markdown(i) => i.diagnostics.clone(),
            Document::MdBook(i) => i.diagnostics.clone(),
            Document::Builtin(i) => i.diagnostics.clone(),
        }
    }
}

impl ResourceLocation for Document {
    fn url(&self) -> &Url {
        match self {
            Document::Source(u) => &u.url,
            Document::Markdown(u) => &u.url,
            Document::MdBook(u) => &u.url,
            Document::Builtin(u) => &u.url,
        }
    }
}

impl commands::LoadFromFile for Document {
    fn load_from_file(&self) -> Result {
        match &self {
            Document::Source(item) => item.load_from_file(),
            Document::Markdown(item) => item.load_from_file(),
            Document::MdBook(item) => item.load_from_file(),
            _ => unimplemented!(),
        }
    }
}

impl commands::Format for Document {
    fn format(&self, params: &commands::FormatParameters) -> Result<bool> {
        match &self {
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

impl commands::Check for Document {
    fn check(&self, config: &Config) -> Result<bool> {
        match &self {
            Document::Source(asset) => asset.check(config),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(_) => todo!(),
        }
    }
}

impl GetAssetSymbol for Document {
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
    fn doc_gen(&self, params: &commands::DocGenParameters) -> self::Result {
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
