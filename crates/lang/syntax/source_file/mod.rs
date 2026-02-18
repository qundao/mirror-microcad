// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad source file representation

use miette::{MietteError, MietteSpanContents, SourceCode, SourceSpan, SpanContents};
use crate::{src_ref::*, syntax::*};

/// µcad source file
#[derive(Clone, Default)]
pub struct SourceFile {
    /// Documentation.
    pub doc: Option<DocBlock>,
    /// Qualified name of the file if loaded from externals
    pub name: QualifiedName,
    /// Root code body.
    pub statements: StatementList,
    /// Name of loaded file.
    pub filename: Option<std::path::PathBuf>,
    /// Source file string, TODO: might be a &'a str in the future
    pub source: String,

    /// Hash of the source file
    ///
    /// This hash is calculated from the source code itself
    /// This is used to map `SrcRef` -> `SourceFile`
    pub hash: u64,
}

impl SourceFile {
    /// Create new source file from existing source.
    pub fn new(
        doc: Option<DocBlock>,
        statements: StatementList,
        source: String,
        hash: u64,
    ) -> Self {
        Self {
            doc,
            statements,
            source,
            hash,
            ..Default::default()
        }
    }
    /// Return filename of loaded file or `<no file>`
    pub fn filename(&self) -> std::path::PathBuf {
        self.filename
            .clone()
            .unwrap_or(std::path::PathBuf::from(crate::invalid_no_ansi!(SOURCE)))
    }

    /// Return filename of loaded file or `<no file>`
    pub fn set_filename(&mut self, path: impl AsRef<std::path::Path>) {
        assert!(self.filename.is_none());
        self.filename = Some(
            path.as_ref()
                .canonicalize()
                .unwrap_or(path.as_ref().to_path_buf()),
        )
    }

    /// Return filename of loaded file or `<no file>`
    pub fn filename_as_str(&self) -> &str {
        self.filename
            .as_ref()
            .map(|f| f.to_str().expect("File name error {filename:?}"))
            .unwrap_or(crate::invalid!(SOURCE))
    }

    /// Return the module name from the file name
    pub fn id(&self) -> Identifier {
        self.name.last().unwrap_or(&Identifier::none()).clone()
    }

    /// get a specific line
    ///
    /// - `line`: line number beginning at `0`
    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.source.lines().nth(line)
    }

    /// get a specific line
    ///
    /// - `line`: line number beginning at `0`
    pub fn get_code(&self, src_ref: &SrcRef) -> &str {
        let range = &src_ref.as_ref().expect("source reference empty").range;
        &self.source[range.start..range.end]
    }

    /// return number of source code lines
    pub fn num_lines(&self) -> usize {
        self.source.lines().count()
    }

    /// Set file name.
    pub fn set_name(&mut self, name: QualifiedName) {
        self.name = name
    }
}

impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.statements.iter().try_for_each(|s| writeln!(f, "{s}"))
    }
}

impl std::fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.statements
            .iter()
            .try_for_each(|s| writeln!(f, "{s:?}"))
    }
}

impl TreeDisplay for SourceFile {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}SourceFile '{:?}' ({}):",
            "",
            self.id(),
            self.filename_as_str()
        )?;
        depth.indent();
        if let Some(doc) = &self.doc {
            doc.tree_print(f, depth)?;
        }
        self.statements
            .iter()
            .try_for_each(|s| s.tree_print(f, depth))
    }
}

impl SrcReferrer for SourceFile {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        SrcRef::new(0..self.num_lines(), 0, 0, self.hash)
    }
}

impl Doc for SourceFile {
    fn doc(&self) -> Option<DocBlock> {
        self.doc.clone()
    }
}

#[test]
fn load_source_file_wrong_location() {
    let source_file = SourceFile::load("I do not exist.µcad");
    if let Err(err) = source_file {
        log::info!("{err}");
        //assert_eq!(format!("{err}"), "Cannot load source file");
    } else {
        panic!("Does file exist?");
    }
}

/// A compatibility layer for using SourceFile with miette
pub struct MietteSourceFile<'a> {
    source: &'a str,
    name: String,
    line_offset: usize,
}

impl MietteSourceFile<'static> {
    /// Create an invalid source file for when we can't load the source
    pub fn invalid() -> Self {
        MietteSourceFile {
            source: crate::invalid_no_ansi!(FILE),
            name: crate::invalid_no_ansi!(FILE).into(),
            line_offset: 0,
        }
    }
}

impl SourceFile {
    /// Get a miette source adapter for the SourceFile
    pub fn miette_source<'a>(&'a self, path: String, line_offset: usize) -> MietteSourceFile<'a> {
        MietteSourceFile {
            source: &self.source,
            name: path,
            line_offset,
        }
    }
}

impl SourceCode for MietteSourceFile<'_> {
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn SpanContents<'a> + 'a>, MietteError> {
        let inner_contents = self.source.read_span(span, context_lines_before, context_lines_after)?;
        let contents = MietteSpanContents::new_named(
            self.name.clone(),
            inner_contents.data(),
            *inner_contents.span(),
            inner_contents.line() + self.line_offset,
            inner_contents.column(),
            inner_contents.line_count(),
        ).with_language("µcad");
        Ok(Box::new(contents))
    }
}
