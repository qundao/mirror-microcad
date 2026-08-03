// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::From;
use microcad_lang_base::{BuiltinId, Identifier, SingleIdentifier, SrcRef, SrcReferrer};
use miette::SourceSpan;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum SymbolPath {
    Builtin(BuiltinId),
    Path {
        is_absolute: bool,
        parts: Box<[Identifier]>,
        src_ref: SrcRef,
    },
}

impl crate::ir::NameSpec for SymbolPath {}

impl SrcReferrer for SymbolPath {
    fn src_ref(&self) -> SrcRef {
        match self {
            SymbolPath::Builtin(_) => SrcRef::none(),
            SymbolPath::Path { src_ref, .. } => *src_ref,
        }
    }
}

impl SingleIdentifier for SymbolPath {
    fn single_identifier(&self) -> Option<&Identifier> {
        match self {
            SymbolPath::Path {
                is_absolute,
                parts,
                src_ref,
            } if self.is_single_identifier() => parts.first(),
            _ => None,
        }
    }

    fn is_single_identifier(&self) -> bool {
        match self {
            SymbolPath::Builtin(_) => false,
            SymbolPath::Path {
                is_absolute, parts, ..
            } => !is_absolute && parts.len() == 1,
        }
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
        let (is_absolute, s) = if s.starts_with("::") {
            (true, s.strip_prefix("::").unwrap())
        } else {
            (false, s)
        };

        Self::Path {
            is_absolute,
            parts: s.split("::").map(Identifier::from).collect(),
            src_ref: SrcRef::none(),
        }
    }
}

impl From<Identifier> for SymbolPath {
    fn from(id: Identifier) -> Self {
        let src_ref = id.src_ref();
        Self::Path {
            is_absolute: false,
            parts: vec![id].into_boxed_slice(),
            src_ref,
        }
    }
}

impl std::fmt::Display for SymbolPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SymbolPath::Builtin(builtin_id) => builtin_id.fmt(f),
            SymbolPath::Path {
                is_absolute, parts, ..
            } => {
                if *is_absolute {
                    write!(f, "::")?;
                }
                write!(
                    f,
                    "{}",
                    parts
                        .iter()
                        .map(|id| format!("{id}"))
                        .collect::<Vec<_>>()
                        .join("::")
                )
            }
        }
    }
}
