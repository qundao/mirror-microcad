// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scaffolding builds the tree from desugared IR items.

use crate::{LowerContext, ir};

pub trait Scaffold: Sized {
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId;
}

impl<T> Scaffold for Box<[T]>
where
    T: Scaffold,
{
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId {
        self.into_iter().for_each(|item| {
            context
                .top_node()
                .append(item.scaffold(context), &mut context.arena)
        });
        *context.top_node()
    }
}

impl Scaffold for ir::desugared::FileModule {
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId {
        context.scaffold_item(ir::IrItem {
            meta: self.meta,
            def: ir::FileModule { attr: self.attr }.into(),
        })
    }
}

impl Scaffold for ir::desugared::Alias {
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId {
        context.scaffold_item(ir::IrItem {
            meta: self.meta,
            def: ir::Alias {
                attr: self.attr,
                path: self.path,
            }
            .into(),
        })
    }
}

impl Scaffold for ir::desugared::Wildcard {
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId {
        context.scaffold_item(ir::IrItem {
            meta: self.meta,
            def: ir::Wildcard {
                attr: self.attr,
                path: self.path,
            }
            .into(),
        })
    }
}

impl Scaffold for ir::desugared::Constant {
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId {
        context.scaffold_item(ir::IrItem {
            meta: self.meta,
            def: ir::Constant {
                attr: self.attr,
                ty: self.ty,
                expr: self.expr,
            }
            .into(),
        })
    }
}

impl Scaffold for ir::desugared::InlineModule {
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId {
        context.scaffold_item_with_children(
            ir::IrItem {
                meta: self.meta,
                def: ir::InlineModule { attr: self.attr }.into(),
            },
            self.items,
        )
    }
}

impl Scaffold for ir::desugared::Function {
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId {
        context.scaffold_item_with_children(
            ir::IrItem {
                meta: self.meta,
                def: ir::Function {
                    attr: self.attr,
                    statements: self.statements,
                    signature: self.signature,
                }
                .into(),
            },
            self.items,
        )
    }
}

impl Scaffold for ir::desugared::Workbench {
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId {
        context.scaffold_item_with_children(
            ir::IrItem {
                meta: self.meta,
                def: ir::Workbench {
                    attr: self.attr,
                    statements: self.statements,
                    kind: self.kind,
                    parameters: self.parameters,
                    inits: self.inits,
                }
                .into(),
            },
            self.items,
        )
    }
}

impl Scaffold for ir::desugared::Source {
    fn scaffold(self, context: &mut LowerContext) -> ir::NodeId {
        context.scaffold_item_with_children(
            ir::IrItem {
                meta: self.meta,
                def: ir::Source {
                    attr: self.attr,
                    statements: self.statements,
                }
                .into(),
            },
            self.items,
        )
    }
}
