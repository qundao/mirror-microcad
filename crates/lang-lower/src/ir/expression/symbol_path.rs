// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::From;
use microcad_lang_base::{BuiltinId, Identifier, SingleIdentifier, SrcRef, SrcReferrer};
use miette::SourceSpan;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum Name {
    Builtin(BuiltinId),
    Path {
        is_absolute: bool,
        parts: Box<[Identifier]>,
        src_ref: SrcRef,
    },
}

impl crate::ir::NameSpec for Name {}

impl SrcReferrer for Name {
    fn src_ref(&self) -> SrcRef {
        match self {
            Name::Builtin(_) => SrcRef::none(),
            Name::Path { src_ref, .. } => *src_ref,
        }
    }
}

impl SingleIdentifier for Name {
    fn single_identifier(&self) -> Option<&Identifier> {
        match self {
            Name::Path {
                is_absolute,
                parts,
                src_ref,
            } if self.is_single_identifier() => parts.first(),
            _ => None,
        }
    }

    fn is_single_identifier(&self) -> bool {
        match self {
            Name::Builtin(_) => false,
            Name::Path {
                is_absolute, parts, ..
            } => !is_absolute && parts.len() == 1,
        }
    }
}

impl From<Name> for SourceSpan {
    fn from(value: Name) -> Self {
        value.src_ref().into()
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Name {
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

impl From<Identifier> for Name {
    fn from(id: Identifier) -> Self {
        let src_ref = id.src_ref();
        Self::Path {
            is_absolute: false,
            parts: vec![id].into_boxed_slice(),
            src_ref,
        }
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Name::Builtin(builtin_id) => builtin_id.fmt(f),
            Name::Path {
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
