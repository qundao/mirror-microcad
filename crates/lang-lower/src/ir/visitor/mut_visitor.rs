// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! IR tree visitor API.

use crate::ir;

pub trait PathVisitorMut: Sized {
    fn visit_path(&mut self, _path: &mut ir::Path) {}
}

pub trait WorkbenchVisitorMut: PathVisitorMut {
    fn visit_workbench(&mut self, workbench: &mut ir::workbench::Workbench) {
        todo!()
    }
    fn visit_workbench_expr(&mut self, expr: &mut ir::workbench::WorkbenchExpression) {
        todo!()
    }
    fn visit_workbench_init(&mut self, init: &mut ir::workbench::Init) {
        todo!()
    }
    fn visit_workbench_stmt(&mut self, stmt: &mut ir::workbench::WorkbenchStatement) {
        todo!()
    }
    fn visit_workbench_call(&mut self, call: &mut ir::workbench::WorkbenchCall) {
        todo!()
    }
    fn visit_workbench_if(&mut self, call: &mut ir::workbench::WorkbenchIf) {
        todo!()
    }
    fn visit_workbench_group(&mut self, call: &mut ir::workbench::Group) {
        todo!()
    }
}

pub trait FnVisitorMut: PathVisitorMut {
    fn visit_fn(&mut self, workbench: &mut ir::workbench::Workbench) {
        todo!()
    }
    fn visit_fn_expr(&mut self, expr: &mut ir::workbench::WorkbenchExpression) {
        todo!()
    }

    fn visit_workbench_stmt(&mut self, stmt: &mut ir::workbench::WorkbenchStatement) {
        todo!()
    }
    fn visit_workbench_call(&mut self, call: &mut ir::workbench::WorkbenchCall) {
        todo!()
    }
    fn visit_workbench_if(&mut self, call: &mut ir::workbench::WorkbenchIf) {
        todo!()
    }
    fn visit_workbench_group(&mut self, call: &mut ir::workbench::Group) {
        todo!()
    }
}

pub trait ConstantVisitorMut: PathVisitorMut {
    fn visit_constant(&mut self, constant: &mut ir::Constant);
}

pub trait VisitorMut: FnVisitorMut + WorkbenchVisitorMut + ConstantVisitorMut {}
