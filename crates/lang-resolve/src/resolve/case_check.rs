// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{SrcReferrer, element::Case};

use crate::library::symbol;
use crate::{ResolveResult, resolve::ResolveError};

/// Check case
pub trait CaseCheck {
    fn expected_case(&self) -> Option<Case>;

    fn case_check(&self, id: &symbol::Identifier) -> ResolveResult<()> {
        let actual = id.detect_case();
        match self.expected_case() {
            Some(expected) => {
                if actual == expected {
                    Ok(())
                } else {
                    Err(ResolveError::WrongCase {
                        expected,
                        actual,
                        src_ref: id.src_ref(),
                    }
                    .into())
                }
            }
            None => Ok(()),
        }
    }
}

impl CaseCheck for symbol::SymbolDef {
    fn expected_case(&self) -> Option<Case> {
        use symbol::SymbolDef::*;
        match &self {
            Source(_) | InlineModule(_) | FileModule(_) | Function(_) => Some(Case::LowerSnake),
            Workbench(_) => Some(Case::Pascal),
            Constant(_) => Some(Case::UpperSnake),
            Alias(_) | Wildcard(_) | Root(_) => None,
        }
    }
}
