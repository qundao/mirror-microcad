// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::resolve::*;
use derive_more::{Deref, DerefMut};

/// Grant Context
#[derive(Deref, DerefMut)]
pub(crate) struct GrantContext<'a> {
    /// Context from resolve.
    #[deref]
    #[deref_mut]
    context: &'a mut ResolveContext,
    /// Scope stack.
    stack: Vec<Scope>,
}

impl<'a> GrantContext<'a> {
    pub(crate) fn new(context: &'a mut ResolveContext) -> Self {
        Self {
            context,
            stack: Default::default(),
        }
    }
    pub(super) fn scope<T, F: FnOnce(&mut Self) -> DiagResult<T>>(
        &mut self,
        scope: Scope,
        f: F,
    ) -> DiagResult<T> {
        self.stack.push(scope);
        let r = f(self);
        self.stack.pop();
        r
    }

    pub(super) fn parent(&self) -> Scope {
        self.stack.last().expect("rootless scope stack").clone()
    }

    pub(super) fn find<F: FnMut(&Scope) -> bool>(&self, f: F) -> bool {
        self.stack.iter().any(f)
    }
}
