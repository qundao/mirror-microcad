use insta::assert_debug_snapshot;
use microcad_syntax::parser::parser;
use microcad_syntax::tokens::{lex, SpannedToken, Token};
use test_case::test_case;
use chumsky::prelude::*;

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
#[test_case("formatted expr string", r#""string {formated - expression:03.5} expression""#)]
#[test_case("function", "fn(a: Length) -> Length {a * 2}")]
#[test_case("comment", "a = 1; // comment")]
#[test_case("multi line comment", r#"a = 1; /** multi
    line
    comment
    */
    b = 2;"#)]
#[test_case("doc comment", r#"/// Doc comment
    part Foo() {
        Cylinder(height = 10mm, radius = 5mm);
    }"#)]
fn test_parser(name: &str, input: &str) {
    let tokens = lex(input).unwrap();
    let input = tokens
        .as_slice()
        .map(2..2, |spanned: &SpannedToken<Token>| {
            (&spanned.token, &spanned.span)
        });
    assert_debug_snapshot!(format!("parser_{name}"), parser().parse(input).into_result());
}
