// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scaffolding builds Unresolved Symbol Tree as mid-level intermediate represenation from IR.

mod scaffoldable;

use crate::{LowerContext, ir};

pub trait Scaffold {
    fn scaffold(&self, context: &mut LowerContext) -> ir::IrNodeId;
}

impl Scaffold for ir::desugared::FileModule {
    fn scaffold(&self, _context: &mut LowerContext) -> ir::IrNodeId {
        todo!()
    }
}

impl Scaffold for ir::desugared::ExplicitAlias {
    fn scaffold(&self, _context: &mut LowerContext) -> ir::IrNodeId {
        todo!()
    }
}

impl Scaffold for ir::desugared::WildcardAlias {
    fn scaffold(&self, _context: &mut LowerContext) -> ir::IrNodeId {
        todo!()
    }
}

impl Scaffold for ir::desugared::Constant {
    fn scaffold(&self, _context: &mut LowerContext) -> ir::IrNodeId {
        todo!()
    }
}

impl Scaffold for ir::desugared::InlineModule {
    fn scaffold(&self, _context: &mut LowerContext) -> ir::IrNodeId {
        todo!()
    }
}

impl Scaffold for ir::desugared::Function {
    fn scaffold(&self, _context: &mut LowerContext) -> ir::IrNodeId {
        todo!()
    }
}

impl Scaffold for ir::desugared::Workbench {
    fn scaffold(&self, _context: &mut LowerContext) -> ir::IrNodeId {
        todo!()
    }
}

impl Scaffold for ir::desugared::Source {
    fn scaffold(&self, _context: &mut LowerContext) -> ir::IrNodeId {
        todo!()
    }
}
