use microcad_builtin::mu::*;
use microcad_builtin_proc_macros::test_builtin_fn;

// --- Arithmetic Operators ---

#[test]
#[test_builtin_fn(core::add(lhs = 5, rhs = 3) => 8)]
#[test_builtin_fn(core::add(lhs = -2, rhs = 4) => 2)]
fn add() {}

#[test]
#[test_builtin_fn(core::sub(lhs = 10, rhs = 4) => 6)]
#[test_builtin_fn(core::sub(lhs = 2, rhs = 5) => -3)]
fn sub() {}

#[test]
#[test_builtin_fn(core::mul(lhs = 4, rhs = 3) => 12)]
#[test_builtin_fn(core::mul(lhs = -3, rhs = 3) => -9)]
fn mul() {}

#[test]
#[test_builtin_fn(core::div(lhs = 12, rhs = 3) => 4.0)]
#[test_builtin_fn(core::div(lhs = 7, rhs = 2) => 3.5)]
fn div() {}

// --- Unary Operators ---

#[test]
#[test_builtin_fn(core::neg(rhs = 5) => -5)]
#[test_builtin_fn(core::neg(rhs = -3) => 3)]
fn neg() {}

#[test]
#[test_builtin_fn(core::plus(rhs = 5) => 5)]
#[test_builtin_fn(core::plus(rhs = -3) => -3)]
fn plus() {}

#[test]
#[test_builtin_fn(core::not(rhs = true) => false)]
#[test_builtin_fn(core::not(rhs = false) => true)]
fn not() {}

// --- Comparison Operators ---

#[test]
#[test_builtin_fn(core::gt(lhs = 3, rhs = 5) => false)]
#[test_builtin_fn(core::gt(lhs = 5, rhs = 3) => true)]
#[test_builtin_fn(core::gt(lhs = 4, rhs = 4) => false)]
fn gt() {}

#[test]
#[test_builtin_fn(core::lt(lhs = 3, rhs = 5) => true)]
#[test_builtin_fn(core::lt(lhs = 5, rhs = 3) => false)]
#[test_builtin_fn(core::lt(lhs = 4, rhs = 4) => false)]
fn lt() {}

#[test]
#[test_builtin_fn(core::ge(lhs = 5, rhs = 3) => true)]
#[test_builtin_fn(core::ge(lhs = 4, rhs = 4) => true)]
#[test_builtin_fn(core::ge(lhs = 3, rhs = 5) => false)]
fn ge() {}

#[test]
#[test_builtin_fn(core::le(lhs = 3, rhs = 5) => true)]
#[test_builtin_fn(core::le(lhs = 4, rhs = 4) => true)]
#[test_builtin_fn(core::le(lhs = 5, rhs = 3) => false)]
fn le() {}

#[test]
#[test_builtin_fn(core::eq(lhs = 4, rhs = 4) => true)]
#[test_builtin_fn(core::eq(lhs = 4, rhs = 5) => false)]
fn eq() {}

#[test]
#[test_builtin_fn(core::not_equal(lhs = 4, rhs = 5) => true)]
#[test_builtin_fn(core::not_equal(lhs = 4, rhs = 4) => false)]
fn not_equal() {}

// --- Logical / Bitwise Operators ---

#[test]
#[test_builtin_fn(core::and(lhs = true, rhs = true) => true)]
#[test_builtin_fn(core::and(lhs = true, rhs = false) => false)]
fn and() {}

#[test]
#[test_builtin_fn(core::or(lhs = true, rhs = false) => true)]
#[test_builtin_fn(core::or(lhs = false, rhs = false) => false)]
fn or() {}

// --- Data Structure Access ---
