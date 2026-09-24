// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) file writer

use microcad_core::*;

use crate::svg::{SvgTagAttributes, canvas::Canvas};

use std::io::Write;

/// SVG writer.
pub struct SvgWriter<W: Write> {
    /// The writer (e.g. a file).
    writer: W,
    /// Indentation level.
    level: usize,
    /// The canvas.
    canvas: Canvas,

    attr_stack: Vec<SvgTagAttributes>,

    /// Tracks if the closing `</svg>` tag was explicitly written.
    finished: bool,

    /// Tracks if the header was written.
    header_written: bool,
}

impl<W: Write> SvgWriter<W> {
    /// Create new SvgWriter
    /// # Arguments
    /// - `w`: Output writer
    /// - `size`: Size of the canvas.
    /// - `scale`: Scale of the output
    pub fn new(
        mut writer: W,
        size: Option<Size2>,
        content_rect: Rect,
        scale: Option<Scalar>,
    ) -> Self {
        let size = size.unwrap_or_else(|| Size2 {
            width: content_rect.width(),
            height: content_rect.height(),
        });

        let canvas = Canvas::new_centered_content(size, content_rect, scale);

        Self {
            writer,
            level: 1,
            canvas,
            attr_stack: vec![SvgTagAttributes::default()],
            finished: false,
            header_written: false,
        }
    }

    /// Ensures the XML preamble and `<svg>` header are emitted exactly once before writing body elements.
    fn ensure_header(&mut self) -> std::io::Result<()> {
        if self.header_written {
            return Ok(());
        }

        let size = &self.canvas.size;
        let (x, y) = (0, 0);
        let (w, h) = (size.width, size.height);

        writeln!(self.writer, "<?xml version='1.0' encoding='UTF-8'?>")?;
        writeln!(
            self.writer,
            r#"<svg version='1.1' xmlns='http://www.w3.org/2000/svg' viewBox='{x} {y} {w} {h}' width='{w}mm' height='{h}mm'>"#
        )?;
        writeln!(
            self.writer,
            r#"  <defs>
        <marker id="arrow" viewBox="0 0 16 16" refX="8" refY="8" markerWidth="9" markerHeight="9" orient="auto-start-reverse">
          <path d="M 0 0 L 16 8 L 0 16 z" stroke="none" fill="context-fill" />
        </marker>
      </defs>"#
        )?;

        self.level = 1;
        self.header_written = true;
        Ok(())
    }

    /// Add attributes to the write
    pub fn with_attributes(mut self, attr: SvgTagAttributes) -> Self {
        self.attr_stack = vec![attr];
        self
    }
}

impl<W: Write> SvgWriter<W> {
    /// Pushes attributes onto the stack, executes the closure `f`,
    /// and ensures the attributes are popped when the closure completes.
    pub fn with_attr<F, R>(&mut self, attr: impl Into<SvgTagAttributes>, f: F) -> std::io::Result<R>
    where
        F: FnOnce(&mut Self) -> std::io::Result<R>,
    {
        // 1. Push new attributes onto the stack
        self.attr_stack.push(attr.into());

        // 2. Execute inner rendering work
        let result = f(self);

        // 3. Guaranteed cleanup: pop attributes regardless of Ok or Err
        self.attr_stack.pop();

        result
    }

    /// Return reference to canvas.
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn attr(&self) -> &SvgTagAttributes {
        self.attr_stack.last().unwrap()
    }

    fn tag_inner(tag: &str, attr: &SvgTagAttributes) -> String {
        format!(
            "{tag}{attr}",
            attr = if attr.is_empty() {
                String::new()
            } else {
                format!(" {attr}")
            }
        )
    }

    /// Write something into the SVG and consider indentation.
    pub fn with_indent(&mut self, s: &str) -> std::io::Result<()> {
        self.ensure_header()?;
        writeln!(self.writer, "{:indent$}{s}", "", indent = 2 * self.level)
    }

    /// Write a single tag `<tag>`.
    pub fn tag(&mut self, tag: &str) -> std::io::Result<()> {
        self.with_indent(&format!(
            "<{tag_inner}/>",
            tag_inner = Self::tag_inner(tag, self.attr())
        ))
    }

    /// Open a tag `<tag>`
    pub fn open_tag(&mut self, tag: &str) -> std::io::Result<()> {
        self.with_indent(&format!(
            "<{tag_inner}>",
            tag_inner = Self::tag_inner(tag, self.attr())
        ))?;

        self.level += 1;
        Ok(())
    }

    /// Close a tag `</tag>`
    pub fn close_tag(&mut self, tag: &str) -> std::io::Result<()> {
        self.level -= 1;
        self.with_indent(format!("</{tag}>").as_str())
    }

    /// Begin a new group `<g>`.
    pub fn begin_group(&mut self, attr: impl Into<SvgTagAttributes>) -> std::io::Result<()> {
        self.attr_stack.push(attr.into());
        self.open_tag("g")
    }

    /// End a group `</g>`.
    pub fn end_group(&mut self) -> std::io::Result<()> {
        self.close_tag("g")?;
        self.attr_stack.pop();
        Ok(())
    }

    /// Defs tag.
    pub fn defs(&mut self, inner: &str) -> std::io::Result<()> {
        self.open_tag("defs")?;
        self.with_indent(inner)?;
        self.close_tag("defs")
    }

    /// Style tag.
    pub fn style(&mut self, inner: &str) -> std::io::Result<()> {
        self.open_tag("style")?;
        self.with_indent(inner)?;
        self.close_tag("style")
    }

    /// Finish this SVG. This method is also called in the Drop trait implementation.
    ///
    /// Writes trailing closing tags (e.g. `</svg>`) and returns the underlying writer.
    pub fn finish(mut self) -> std::io::Result<W> {
        self.ensure_header()?;

        writeln!(self.writer, "</svg>")?;
        self.writer.flush()?;
        Ok(self.writer)
    }
}
