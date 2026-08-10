use microcad_builtin::{BuiltinEvalContext, mu::*};
use microcad_builtin_proc_macros::test_builtin_fn;

use microcad_lang_types::{Value, arguments, array, tuple};

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
// --- Data Structure Access & Construction ---

#[test]
#[test_builtin_fn(core::array_access(lhs = array![10, 20, 30], index = 1) => 20)]
#[test_builtin_fn(core::array_access(lhs = array![10, 20, 30], index = 0) => 10)]
fn array_access() {}

#[test]
#[test_builtin_fn(core::member_access(lhs = tuple!(a = 42, b = "hello"), name = "a") => 42)]
#[test_builtin_fn(core::member_access(lhs = tuple!(a = 42, b = "hello"), name = "b") => "hello")]
fn member_access() {}

#[test]
//#[test_builtin_fn(core::list(1, 2, 3) => list(1, 2, 3))]
fn array() {
    let mut ctx = BuiltinEvalContext::default();
    match core::array(arguments!(1, 2, 3, 4), &mut ctx) {
        Ok(Value::Array(a)) => assert_eq!(a, array![1, 2, 3, 4]),
        _ => panic!("Formatting failed"),
    }
}

#[test]
#[test_builtin_fn(core::tuple(x = 10, y = 20) => tuple!(x = 10, y = 20))]
fn tuple() {}

#[test]
#[test_builtin_fn(core::range(start = 1, end = 4) => array![1, 2, 3, 4])]
#[test_builtin_fn(core::range(start = 0, end = 0) => array![0])]
fn range() {}

// --- String Formatting ---

#[test]
fn format() {
    let mut ctx = BuiltinEvalContext::default();
    match core::format(arguments!("Hello, ", "World!", " ", 2026), &mut ctx) {
        Ok(Value::String(s)) => assert_eq!(s.as_str(), "Hello, World! 2026"),
        _ => panic!("Formatting failed"),
    }
}

#[test]
#[test_builtin_fn(core::format_spec(expr = 3.14159, width = 8, precision = 2) => "    3.14")]
#[test_builtin_fn(core::format_spec(expr = 42, width = 5, precision = -1) => "   42")]
#[test_builtin_fn(core::format_spec(expr = 3.14159, width = -1, precision = 3) => "3.142")]
fn format_spec() {}

// --- Debug & Diagnostic Functions ---

#[test]
#[test_builtin_fn(debug::assert(cond = true, message = "ok") => ())]
fn assert_pass() {}

#[test]
#[test_builtin_fn(debug::expect(cond = true, message = "ok") => ())]
#[test_builtin_fn(debug::expect(cond = false, message = "warning") => ())]
fn expect() {}

#[test]
#[test_builtin_fn(debug::error(message = "custom error") => ())]
fn debug_error() {}

#[test]
#[test_builtin_fn(debug::warning(message = "custom warning") => ())]
fn debug_warning() {}

#[test]
#[test_builtin_fn(debug::info(message = "custom info") => ())]
fn debug_info() {}
