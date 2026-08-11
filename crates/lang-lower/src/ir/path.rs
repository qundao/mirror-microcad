// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use derive_more::{Display, From};
use microcad_lang_base::{
    BuiltinId, Identifier, SingleIdentifier, SrcRef, SrcReferrer, SymbolId, SymbolName,
    ToCompactString, Unresolve,
};
use miette::SourceSpan;

use crate::ir;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, From, Hash, PartialEq, Serialize, Deserialize)]
pub struct UnresolvedPath {
    pub is_absolute: bool,
    pub parts: Box<[Identifier]>,
    pub src_ref: SrcRef,
}

impl SingleIdentifier for UnresolvedPath {
    fn single_identifier(&self) -> Option<&Identifier> {
        if self.is_single_identifier() {
            self.parts.first()
        } else {
            None
        }
    }

    fn is_single_identifier(&self) -> bool {
        self.is_absolute && self.parts.len() == 1
    }
}

impl std::fmt::Display for UnresolvedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_absolute {
            write!(f, "::")?;
        }
        write!(
            f,
            "{}",
            self.parts
                .iter()
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
                .join("::")
        )
    }
}

#[derive(Clone, Debug, Display, From, Hash, PartialEq, Serialize, Deserialize)]
pub enum Path {
    /// A Path that starts with a `__mu` prefix.
    Builtin(BuiltinId),
    /// A path that still needs to be resolved by the symbol resolver.
    UnresolvedPath(UnresolvedPath),
}

impl From<Path> for SymbolId {
    fn from(path: Path) -> Self {
        match path {
            Path::Builtin(builtin_id) => SymbolId::Builtin(builtin_id),
            Path::UnresolvedPath(unresolved_path) => {
                SymbolId::Unresolved(unresolved_path.to_compact_string())
            }
        }
    }
}

pub trait PathSpec: Serialize + SrcReferrer + SingleIdentifier {}

impl ir::PathSpec for Path {}

impl PathSpec for SymbolName {}

impl Unresolve<SymbolName> for Path {
    fn unresolve_symbols<U: microcad_lang_base::Unresolver>(
        self,
        unresolver: &mut U,
    ) -> SymbolName {
        match self {
            Path::Builtin(builtin_id) => unresolver.unresolve(builtin_id),
            Path::UnresolvedPath(path) => {
                let full_name = path.to_string().to_compact_string();
                SymbolName {
                    symbol_id: SymbolId::Unresolved(full_name.clone()),
                    full_name,
                }
            }
        }
    }
}

impl SrcReferrer for Path {
    fn src_ref(&self) -> SrcRef {
        match self {
            Path::Builtin(_) => SrcRef::none(),
            Path::UnresolvedPath(path) => path.src_ref,
        }
    }
}

impl SingleIdentifier for Path {
    fn single_identifier(&self) -> Option<&Identifier> {
        match self {
            Path::UnresolvedPath(path) => path.single_identifier(),
            _ => None,
        }
    }

    fn is_single_identifier(&self) -> bool {
        match self {
            Path::Builtin(_) => false,
            Path::UnresolvedPath(path) => path.is_single_identifier(),
        }
    }
}

impl From<Path> for SourceSpan {
    fn from(value: Path) -> Self {
        value.src_ref().into()
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        let (is_absolute, s) = if s.starts_with("::") {
            (true, s.strip_prefix("::").unwrap())
        } else {
            (false, s)
        };

        Self::UnresolvedPath(UnresolvedPath {
            is_absolute,
            parts: s.split("::").map(Identifier::from).collect(),
            src_ref: SrcRef::none(),
        })
    }
}

impl From<Identifier> for Path {
    fn from(id: Identifier) -> Self {
        let src_ref = id.src_ref();
        Self::UnresolvedPath(UnresolvedPath {
            is_absolute: false,
            parts: vec![id].into_boxed_slice(),
            src_ref,
        })
    }
}
