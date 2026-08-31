mod common;

use microcad_lang_base::Artifact;
use microcad_lang_lower::ir;
use microcad_lang_lower::ir::visitor::{
    ConstantVisitor, FnVisitor, Visitor, WorkbenchStatementVisitor, WorkbenchVisitor,
};
use microcad_macros::__mu;
use std::collections::HashSet;

/// A visitor that collects references to all `ir::Path` instances found in the IR tree.
#[derive(Debug, Default)]
pub struct PathCollector {
    pub paths: Vec<ir::Path>,
}

impl PathCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenience helper to collect all unique paths as a HashSet.
    pub fn collect_unique(mut self, tree: &ir::Tree) -> HashSet<ir::Path> {
        self.visit_tree(tree);
        self.paths.into_iter().collect()
    }
}

// 1. Core leaf visitor overriding path collection
impl ConstantVisitor for PathCollector {
    fn visit_path(&mut self, path: &ir::Path) {
        self.paths.push(path.clone());
    }
}

// 2. Sub-trait implementations (inherit default traversal behavior)
impl WorkbenchStatementVisitor for PathCollector {}
impl WorkbenchVisitor for PathCollector {}
impl FnVisitor for PathCollector {}
impl Visitor for PathCollector {}

#[test]
fn path_collector() {
    let source = common::source_from_test_file("circle");
    let (ir, _) = common::ir_from_source(&source).expect("No errors");

    // Collect all paths from a full IR tree
    let mut collector = PathCollector::new();
    collector.visit_tree(&ir.tree);
    println!("Found {} paths", collector.paths.len());

    let paths = collector.collect_unique(&ir.tree);
    for path in &paths {
        println!("{path}");
    }

    assert!(
        paths.contains(&ir::Path::Resolved(microcad_lang_base::SymbolId::Builtin(
            __mu!(geo2d::Circle),
        )))
    );

    assert!(
        paths.contains(&ir::Path::Resolved(microcad_lang_base::SymbolId::Builtin(
            __mu!(core::member_access),
        )))
    );

    println!("{}", ir.to_ron().expect("No error"))
}
