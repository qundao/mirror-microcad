// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Initialization definition syntax element

use crate::ir;

use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

/// Workbench *initializer* definition
///
/// Example:
///
/// ```uCAD
/// part A(a: Length) {
///     init(b: Length) { a = 2.0*b; } // The init definition
/// }
/// ```
#[derive(Clone, Debug, SrcReferrer)]
pub struct InitDefinition {
    /// SrcRef of the `init` keyword
    pub keyword_ref: SrcRef,
    /// Documentation.
    pub doc: ir::DocBlock,
    /// Parameter list for this init definition
    pub parameters: ir::ParameterList,
    /// Body if the init definition
    pub body: ir::Body,
    /// Source reference
    pub src_ref: SrcRef,
}

impl InitDefinition {
    /// Return signature with parameters if this init.
    pub fn signature(&self) -> String {
        format!("init({parameters}) ", parameters = self.parameters)
    }
}

impl std::fmt::Display for InitDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.signature())?;
        write!(f, "{body}", body = self.body)
    }
}

/// Iterator over part's *initializers*.
pub struct Inits<'a>(std::slice::Iter<'a, ir::Statement>);

impl<'a> Inits<'a> {
    /// Create new init for a part.
    pub fn new(def: &'a impl crate::Initialized<'a>) -> Self {
        Self(def.statements())
    }
}

impl<'a> Iterator for Inits<'a> {
    type Item = &'a InitDefinition;

    fn next(&mut self) -> Option<Self::Item> {
        for statement in self.0.by_ref() {
            match statement {
                ir::Statement::Init(init_definition) => {
                    return Some(init_definition);
                }
                _ => continue,
            }
        }

        None
    }
}
