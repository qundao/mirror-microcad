use insta::assert_debug_snapshot;
use microcad_syntax::parser::parse;
use microcad_syntax::tokens::lex;
use test_case::test_case;

#[test_case("single int", "1")]
#[test_case("int overflow", "9999999999999999999")]
#[test_case("single float", "1.2")]
#[test_case("quantity int", "1mm")]
#[test_case("quantity float", ".2mm")]
#[test_case("basic addition", "1 + 1")]
#[test_case("basic addition, no space", "1+1")]
#[test_case("addition identifier", "length + 1")]
#[test_case("array range", "[1..10]")]
#[test_case("array list", "[1,2,3,4]")]
#[test_case("multiple binary operator", "(1+2)*3+1")]
#[test_case("assignment", "a = b * 2;")]
#[test_case("typed assignment", "a: Length = b * 2mm;")]
#[test_case("block assignment", "a = {a = 1 + 2; a * 3};")]
#[test_case("plain string", r#""plain string""#)]
#[test_case("plain string in expr", r#"a = "plain string int expression" + 1;"#)]
#[test_case("escaped bracket string", r#""string {{ with }} escaped bracket""#)]
#[test_case("escaped quote string", r#""string \" with \" escaped \ quotes""#)]
#[test_case("basic expr string", r#""string {with} expression""#)]
#[test_case("expr string", r#""string {more + complex} expression""#)]
#[test_case(
    "formatted expr string width",
    r#""string {formatted - expression:03} expression""#
)]
#[test_case(
    "formatted expr string accuracy",
    r#""string {formatted - expression:.5} expression""#
)]
#[test_case(
    "formatted expr string both",
    r#""string {formatted - expression:03.5} expression""#
)]
#[test_case("function", "fn foo(a: Length) -> Length {a * 2}")]
#[test_case("function with return", "fn foo(a: Length) -> Length {return a * 2;}")]
#[test_case("function with default", "fn foo(a: Length, b = 3mm) {b / a}")]
#[test_case("call empty args", "foo()")]
#[test_case("call positional args", "foo(1, 2)")]
#[test_case("call named args", "foo(a = 1, b = 2)")]
#[test_case("call partially named args", "foo(a, b = 2, 3)")]
#[test_case("comment", "a = 1; // comment")]
#[test_case(
    "multi line comment",
    r#"a = 1; /** multi
    line
    comment
    */
    b = 2;"#
)]
#[test_case(
    "doc comment",
    r#"/// Doc comment
    part Foo() {
        Cylinder(height = 10mm, radius = 5mm);
    }"#
)]
#[test_case("tuple", "(1, 1 + 1)")]
#[test_case("one-tuple", "(1,)")]
#[test_case("one-bracketed", "(1)")]
#[test_case("named tuple", "(length = 1, width = 1 + 1)")]
#[test_case("named one-tuple, trailing", "(length = 1,)")]
#[test_case("named one-tuple", "(length = 1)")]
#[test_case("partially named tuple", "(\"a\", length = 1)")]
#[test_case("empty tuple", "()")]
#[test_case("qualified name", "foo::bar")]
#[test_case("marker", "@input")]
#[test_case("unary", "!input")]
#[test_case("if", "if a > 1 { foo(); }")]
#[test_case("if-else", "if a > 1 { 3 } else { 4 }")]
#[test_case("else-if", "if a > 1 { 3 } else if a < -1 { 1 }")]
#[test_case("else-if-else", "if a > 1 { 3 } else if a < -1 { 1 } else { 0 }")]
#[test_case("sketch", "sketch Wheel(radius: Length) {std::geo2d::Circle(radius);}")]
#[test_case(
    "pub-part",
    "pub part Wheel(radius: Length, height = 1mm) {std::geo3d::Cylinder(radius, height);}"
)]
#[test_case(
    "sketch-with-init",
    "sketch Wheel(radius: Length) {
    init(diameter: Length) {
        radius = diameter / 2;
    }
    std::geo2d::Circle(radius);
}"
)]
#[test_case("mod", "mod foo { fn bar(){} }")]
#[test_case("mod pub", "pub mod foo { fn bar(){} }")]
#[test_case("mod extern", "mod foo;")]
#[test_case("use", "use foo;")]
#[test_case("use glob", "use foo::bar::*;")]
#[test_case("use as", "pub use foo::bar as foobar;")]
#[test_case("array list units", "a = [1, 2]mm;")]
#[test_case("array range units", "a = [1..3]mm;")]
fn test_parser(name: &str, input: &str) {
    let tokens = lex(input);
    assert_debug_snapshot!(format!("parser_{name}"), parse(tokens.as_slice()));
}
