// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod builtin;
mod markdown;
mod mdbook;
mod source;

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use derive_more::From;
use microcad_lang_base::{DiagRenderOptions, Diagnostics, MICROCAD_EXTENSIONS};
use url::Url;

use crate::{Config, document};

/// A container for a document with a state and diagnostics
pub struct Item<S: Default> {
    /// Each container must a URL.
    url: Url,
    /// Each document can have its own config
    config: Rc<Config>,
    /// Each document item keeps its [Diagnostics]
    diagnostics: RefCell<Diagnostics>,
    /// Each document has a state.
    state: RefCell<S>,
}

pub type DiagResult<'a, T = ()> = Result<T, Ref<'a, Diagnostics>>;

impl<S: Default> Item<S> {
    /// Create a new container
    fn new(url: Url, config: Rc<Config>) -> Rc<Self> {
        Rc::new(Self {
            url,
            config,
            diagnostics: Default::default(),
            state: Default::default(),
        })
    }

    /// Generic transitioner to move the pipeline forward
    fn transition<F>(&self, f: F) -> DiagResult
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
                Err(self.diagnostics.borrow())
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

pub type SourceItem = Item<document::source::State>;
pub type MarkdownItem = Item<document::markdown::State>;
pub type MdBookItem = Item<document::mdbook::State>;
pub type BuiltinItem = Item<document::builtin::State>;

#[derive(From)]
pub enum Document {
    /// A single source file
    Source(Rc<SourceItem>),

    /// A markdown file containing source code snippets
    Markdown(Rc<MarkdownItem>),

    /// An `book.toml` of a markdown book
    MdBook(Rc<MdBookItem>),

    /// A builtin symbol
    Builtin(Rc<BuiltinItem>),
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
            "book.toml" => Ok(MdBookItem::new(url, config).into()),
            _ => match extension {
                "md" => Ok(MarkdownItem::new(url, config).into()),
                extension => {
                    if MICROCAD_EXTENSIONS.contains(&extension) {
                        Ok(SourceItem::new(url, config).into())
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

    pub fn load_from_file(&'_ self) -> DiagResult<'_> {
        match &self {
            Document::Source(item) => item.load_from_file(),
            Document::Markdown(item) => item.load_from_file(),
            Document::MdBook(item) => item.load_from_file(),
            _ => unimplemented!(),
        }
    }

    pub fn format(&'_ self) -> DiagResult<'_, bool> {
        match &self {
            Document::Source(item) => item.format(),
            Document::Markdown(item) => item.format(),
            Document::MdBook(item) => item.format(),
            _ => unimplemented!(),
        }
    }

    /// Write to disk
    pub fn sync(&'_ self) -> DiagResult<'_> {
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

    pub fn diagnostics(&'_ self) -> std::cell::Ref<'_, Diagnostics> {
        match self {
            Document::Source(i) => i.diagnostics.borrow(),
            Document::Markdown(i) => i.diagnostics.borrow(),
            Document::MdBook(i) => i.diagnostics.borrow(),
            Document::Builtin(i) => i.diagnostics.borrow(),
        }
    }

    pub fn pretty_print(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        match self {
            Document::Source(i) => i.pretty_print(f),
            Document::Markdown(u) => todo!(),
            Document::MdBook(u) => todo!(),
            Document::Builtin(u) => todo!(),
        }
    }

    pub fn diagnostics_string(&self) -> String {
        let mut buffer = String::new();
        match self.pretty_print(&mut buffer) {
            Ok(_) | Err(_) => buffer,
        }
    }
}
