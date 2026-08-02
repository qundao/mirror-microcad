// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{Identifier, SingleIdentifier, SrcRef, SrcReferrer};
use microcad_lang_proc_macros::SrcReferrer;
use miette::SourceSpan;

use serde::{Deserialize, Serialize};

/// A *qualified name* consists of a list of *identifiers*, separated by `::`,
/// e.g. `a::b::c`
#[derive(Default, Clone, Debug, Hash, PartialEq, SrcReferrer, Serialize, Deserialize)]
pub struct SymbolPath {
    pub prefix: Option<SrcRef>,
    pub parts: Box<[Identifier]>,
    pub src_ref: SrcRef,
}

impl crate::ir::NameKind for SymbolPath {}

impl SingleIdentifier for SymbolPath {
    fn single_identifier(&self) -> Option<&Identifier> {
        if self.is_single_identifier() {
            self.parts.first()
        } else {
            None
        }
    }

    fn is_single_identifier(&self) -> bool {
        self.prefix.is_none() && self.parts.len() == 1
    }
}

impl From<SymbolPath> for SourceSpan {
    fn from(value: SymbolPath) -> Self {
        value.src_ref().into()
    }
}

impl From<String> for SymbolPath {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for SymbolPath {
    fn from(s: &str) -> Self {
        let (prefix, s) = if s.starts_with("::") {
            (Some(SrcRef::none()), s.strip_prefix("::").unwrap())
        } else {
            (None, s)
        };

        Self {
            prefix,
            parts: s.split("::").map(Identifier::from).collect(),
            src_ref: SrcRef::none(),
        }
    }
}

impl From<Identifier> for SymbolPath {
    fn from(id: Identifier) -> Self {
        let src_ref = id.src_ref();
        Self {
            prefix: None,
            parts: vec![id].into_boxed_slice(),
            src_ref,
        }
    }
}

impl std::fmt::Display for SymbolPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.prefix {
                Some(_) => "::",
                None => "",
            }
        )?;
        write!(
            f,
            "{}",
            self.parts
                .iter()
                .map(|id| format!("{id}"))
                .collect::<Vec<_>>()
                .join("::")
        )
    }
}
