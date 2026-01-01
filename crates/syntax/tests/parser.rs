use chumsky::prelude::*;
use insta::assert_debug_snapshot;
use microcad_syntax::parser::{map_token_input, parse};
use microcad_syntax::tokens::{NormalToken, SpannedToken, lex};
use test_case::test_case;

#[test_case("single int", "1")]
#[test_case("basic addition", "1 + 1")]
#[test_case("basic addition, no space", "1+1")]
#[test_case("addition identifier", "length + 1")]
#[test_case("array range", "[1..10]")]
#[test_case("array list", "[1,2,3,4]")]
#[test_case("multiple binary operator", "(1+2)*3+1")]
#[test_case("assignment", "a = b * 2;")]
#[test_case("block assignment", "a = {a = 1 + 2; a * 3};")]
#[test_case("plain string", r#""plain string""#)]
#[test_case("plain string in expr", r#"a = "plain string int expression" + 1;"#)]
#[test_case("escaped bracket string", r#""string {{ with }} escaped bracket""#)]
#[test_case("escaped quote string", r#""string \" with \" escaped \ quotes""#)]
#[test_case("basic expr string", r#""string {with} expression""#)]
#[test_case("expr string", r#""string {more + complex} expression""#)]
#[test_case(
    "formatted expr string width",
    r#""string {formated - expression:03} expression""#
)]
#[test_case(
    "formatted expr string accuracy",
    r#""string {formated - expression:.5} expression""#
)]
#[test_case(
    "formatted expr string both",
    r#""string {formated - expression:03.5} expression""#
)]
#[test_case("function", "fn(a: Length) -> Length {a * 2}")]
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
#[test_case("qualified name", "foo::bar")]
fn test_parser(name: &str, input: &str) {
    let tokens = lex(input).unwrap();
    assert_debug_snapshot!(
        format!("parser_{name}"),
        parse(tokens.as_slice())
    );
}
