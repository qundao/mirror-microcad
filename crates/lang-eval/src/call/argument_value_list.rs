// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! *Argument value list* evaluation entity.

use derive_more::{Deref, DerefMut};
use microcad_lang_base::SrcRef;
use microcad_lang_proc_macros::SrcReferrer;

use crate::{ArgumentValue, EvalError, EvalResult};

/// Collection of *argument values* (e.g. `( x=1, y=2 )`).
///
/// Also provides methods to find a matching call
/// between it and a given *parameter list*.
#[derive(Clone, Debug, Default, Deref, DerefMut, SrcReferrer)]
pub struct ArgumentValueList {
    #[deref]
    #[deref_mut]
    pub args: Vec<ArgumentValue>,
    pub src_ref: SrcRef,
}

impl ArgumentValueList {
    /// Return a single argument.
    ///
    /// Returns error if there is no or more than one argument available.
    pub fn get_single(&self) -> EvalResult<&ArgumentValue> {
        if self.args.len() == 1 {
            if let Some(a) = self.args.first() {
                return Ok(&a);
            }
        }

        Err(EvalError::ArgumentCountMismatch {
            args: self.to_string(),
            expected: 1,
            found: self.args.len(),
        }
        .into())
    }
}

impl std::fmt::Display for ArgumentValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        })
    }
}

impl<Arg> FromIterator<Arg> for ArgumentValueList
where
    Arg: Into<ArgumentValue>,
{
    fn from_iter<T: IntoIterator<Item = Arg>>(iter: T) -> Self {
        let args: Vec<_> = iter.into_iter().map(|a| a.into()).collect();
        Self {
            src_ref: SrcRef::none(),
            args,
        }
    }
}
