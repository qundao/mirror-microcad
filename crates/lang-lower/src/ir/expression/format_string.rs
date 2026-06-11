// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad format expression syntax elements

use crate::ir;

use microcad_lang_base::{Refer, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::SrcReferrer;
use serde::Serialize;

/// Format string item.
#[derive(Debug, Serialize)]
pub enum FormatStringInner {
    /// String literal.
    String(Refer<String>),
    /// Format expression.
    FormatExpression(Box<FormatExpression>),
}

impl SrcReferrer for FormatStringInner {
    fn src_ref(&self) -> SrcRef {
        match self {
            FormatStringInner::String(s) => s.src_ref(),
            FormatStringInner::FormatExpression(e) => e.src_ref(),
        }
    }
}

/// Format string.
#[derive(Default, Debug, SrcReferrer, Serialize)]
pub struct FormatString(pub Refer<Vec<FormatStringInner>>);

impl FormatString {
    /// Insert a string.
    pub fn push_string(&mut self, s: String, src_ref: SrcRef) {
        self.0
            .push(FormatStringInner::String(Refer::new(s, src_ref)));
    }

    /// Insert a format expression
    pub fn push_format_expr(&mut self, expr: FormatExpression) {
        self.0
            .push(FormatStringInner::FormatExpression(Box::new(expr)));
    }

    /// Return the number of sections (inserted elements)
    pub fn section_count(&self) -> usize {
        self.0.len()
    }
}

impl From<Refer<String>> for FormatString {
    fn from(value: Refer<String>) -> Self {
        FormatString(Refer {
            src_ref: value.src_ref.clone(),
            value: vec![FormatStringInner::String(value)],
        })
    }
}

impl std::fmt::Display for FormatString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, r#"""#)?;
        for elem in &*self.0 {
            match elem {
                FormatStringInner::String(s) => write!(f, "{}", s.value)?,
                FormatStringInner::FormatExpression(expr) => write!(f, "{expr}")?,
            }
        }
        write!(f, r#"""#)?;
        Ok(())
    }
}

/// Format expression including format specification.
#[derive(Debug, SrcReferrer, Serialize)]
pub struct FormatExpression {
    /// Format specifier
    pub spec: Option<ir::FormatSpec>,
    /// Expression to format
    pub expression: ir::ConstantExpression,
    /// Source code reference
    src_ref: SrcRef,
}

impl FormatExpression {
    /// Create new format expression.
    pub fn new(
        spec: Option<ir::FormatSpec>,
        expression: ir::ConstantExpression,
        src_ref: SrcRef,
    ) -> Self {
        Self {
            src_ref,
            spec,
            expression,
        }
    }
}

impl std::fmt::Display for FormatExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(spec) = &self.spec {
            write!(f, "{{{}:{}}}", spec, self.expression)
        } else {
            write!(f, "{{{}}}", self.expression)
        }
    }
}

/// Format specification.
#[derive(Debug, Default, PartialEq, SrcReferrer, Serialize)]
pub struct FormatSpec {
    /// Precision for number formatting.
    pub precision: Option<u32>,
    /// Alignment width (leading zeros).
    pub width: Option<u32>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl std::fmt::Display for FormatSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.width, self.precision) {
            (Some(width), Some(precision)) => write!(f, "0{width}.{precision}"),
            (None, Some(precision)) => write!(f, ".{precision}"),
            (Some(width), None) => write!(f, "0{width}"),
            _ => Ok(()),
        }
    }
}
