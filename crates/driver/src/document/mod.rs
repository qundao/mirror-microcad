// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod builtin;
mod markdown;
mod mdbook;
mod source;

use std::{cell::RefCell, rc::Rc};

use derive_more::From;
use microcad_lang_base::{Diagnostics, MICROCAD_EXTENSIONS, RcMut};
use url::Url;

use crate::{Config, commands::CommandResult, document};

/// A container for a document with a state and diagnostics
pub struct Asset<S: Default> {
    /// Each container must a URL.
    url: Url,
    /// Each document can have its own config
    config: Rc<Config>,
    /// Each document item keeps its [Diagnostics]
    diagnostics: RcMut<Diagnostics>,
    /// Each document has a state.
    state: RefCell<S>,
}

impl<S: Default> Asset<S> {
    /// Create a new container
    fn new(url: Url, config: Rc<Config>) -> Rc<Self> {
        Rc::new(Self {
            url,
            config,
            diagnostics: RcMut::new(Default::default()),
            state: Default::default(),
        })
    }

    /// Generic transitioner to move the pipeline forward
    fn transition<F>(&self, f: F) -> CommandResult
    where
        F: FnOnce(S) -> Result<S, Diagnostics>,
    {
        let mut state = self.state.borrow_mut();
        let current = std::mem::take(&mut *state);
        match f(current) {
            Ok(new_state) => {
                *state = new_state.into();
                Ok(())
            }
            Err(diag) => {
                self.diagnostics.replace(diag);
                Err(self.diagnostics.clone())
            }
        }
    }

    /// Returns the local file path if the URL is a "file://" scheme.
    pub fn file_path(&self) -> Option<std::path::PathBuf> {
        // url::to_file_path() returns Result<PathBuf, ()>
        // We convert it to Option for a cleaner API
        self.url.to_file_path().ok()
    }
}

pub type SourceAsset = Asset<document::source::State>;
pub type MarkdownAsset = Asset<document::markdown::State>;
pub type MdBookAsset = Asset<document::mdbook::State>;
pub type BuiltinAsset = Asset<document::builtin::State>;

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
    pub fn new(url: Url, config: Rc<Config>) -> miette::Result<Self> {
        let path = std::path::Path::new(url.path());
        let file_name = path.file_name().and_then(|os| os.to_str()).unwrap_or("");
        let extension = path.extension().and_then(|os| os.to_str()).unwrap_or("");

        match file_name {
            "book.toml" => Ok(MdBookAsset::new(url, config).into()),
            _ => match extension {
                "md" => Ok(MarkdownAsset::new(url, config).into()),
                extension => {
                    if MICROCAD_EXTENSIONS.contains(&extension) {
                        Ok(SourceAsset::new(url, config).into())
                    } else {
                        Err(miette::miette!("Invalid document type: {extension}"))
                    }
                }
            },
        }
    }

    pub fn from_file_path(
        path: impl AsRef<std::path::Path>,
        config: Rc<Config>,
    ) -> miette::Result<Self> {
        let url = Url::from_file_path(path.as_ref()).expect("No error");
        Self::new(url, config)
    }

    pub fn load_from_file(&self) -> CommandResult {
        match &self {
            Document::Source(item) => item.load_from_file(),
            Document::Markdown(item) => item.load_from_file(),
            Document::MdBook(item) => item.load_from_file(),
            _ => unimplemented!(),
        }
    }

    pub fn format(&self) -> CommandResult<bool> {
        match &self {
            Document::Source(item) => item.format(),
            Document::Markdown(item) => item.format(),
            Document::MdBook(item) => item.format(),
            _ => unimplemented!(),
        }
    }

    /// Write to disk
    pub fn sync(&self) -> CommandResult {
        match &self {
            Document::Source(source) => source.sync(),
            Document::Markdown(markdown) => markdown.sync(),
            Document::MdBook(mdbook) => mdbook.sync(),
            _ => unimplemented!(),
        }
    }

    pub fn url(&self) -> &Url {
        match self {
            Document::Source(u) => &u.url,
            Document::Markdown(u) => &u.url,
            Document::MdBook(u) => &u.url,
            Document::Builtin(u) => &u.url,
        }
    }

    pub fn file_path(&self) -> Option<std::path::PathBuf> {
        self.url().to_file_path().ok()
    }

    pub fn diagnostics(&self) -> RcMut<Diagnostics> {
        match self {
            Document::Source(i) => i.diagnostics.clone(),
            Document::Markdown(i) => i.diagnostics.clone(),
            Document::MdBook(i) => i.diagnostics.clone(),
            Document::Builtin(i) => i.diagnostics.clone(),
        }
    }

    pub fn pretty_print(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        match self {
            Document::Source(i) => i.pretty_print(f),
            Document::Markdown(_) => todo!(),
            Document::MdBook(_) => todo!(),
            Document::Builtin(_) => todo!(),
        }
    }

    pub fn diagnostics_string(&self) -> String {
        let mut buffer = String::new();
        match self.pretty_print(&mut buffer) {
            Ok(_) | Err(_) => buffer,
        }
    }
}
