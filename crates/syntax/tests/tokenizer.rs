use insta::assert_debug_snapshot;
use microcad_syntax::tokens::lex;
use test_case::test_case;

#[test_case("single int", "1")]
#[test_case("basic addition", "1 + 1")]
#[test_case("basic addition, no space", "1+1")]
#[test_case("assignment", "a = b * 2")]
#[test_case("plain string", r#""plain string""#)]
#[test_case("plain string in expr", r#"a = "plain string int expression" + 1"#)]
#[test_case("escaped bracket string", r#""string {{ with }} escaped bracket""#)]
#[test_case("escaped quote string", r#""string \" with \" escaped \ quotes""#)]
#[test_case("basic expr string", r#""string {with} expression""#)]
#[test_case("expr string", r#""string {more + complex} expression""#)]
#[test_case(
    "formatted expr string",
    r#""string {formated - expression:03.5} expression""#
)]
#[test_case("function", "fn(a: Length) -> Length {a * 2}")]
#[test_case("comment", "a = 1 // comment")]
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
fn test_lexer(name: &str, input: &str) {
    assert_debug_snapshot!(format!("lexer_{name}"), lex(input));
}
