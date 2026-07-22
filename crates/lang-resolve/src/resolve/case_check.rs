// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::{SrcReferrer, element::Case};

use crate::{ResolveResult, resolve::ResolveError, rst};

/// Check case
pub trait CaseCheck {
    fn expected_case(&self) -> Option<Case>;

    fn case_check(&self, id: &rst::Identifier) -> ResolveResult<()> {
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
                    })
                }
            }
            None => Ok(()),
        }
    }
}

impl CaseCheck for rst::ResolvedSymbolDef {
    fn expected_case(&self) -> Option<Case> {
        use rst::ResolvedSymbolDef::*;
        match &self {
            SourceFile(_) | InlineModule | Function(_) => Some(Case::LowerSnake),
            Workbench(_) => Some(Case::Pascal),
            Constant(_) => Some(Case::UpperSnake),
            Builtin(_) | Alias(_) | Wildcard(_) => None,
            _ => None,
        }
    }
}
