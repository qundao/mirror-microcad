use microcad_builtin::mu::*;

use microcad_builtin_proc_macros::test_builtin_fn;

#[test]
#[test_builtin_fn(core::greater_than(lhs = 3, rhs = 5) => false)]
#[test_builtin_fn(core::greater_than(lhs = 5, rhs = 3) => true)]
fn greater_than() {}
