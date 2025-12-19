use insta::assert_debug_snapshot;
use microcad_syntax::tokens::lex;

#[test]
fn test_basic() {
    assert_debug_snapshot!(lex("1"));
    assert_debug_snapshot!(lex("1 + 1"));
    assert_debug_snapshot!(lex("1+ 1"));
    assert_debug_snapshot!(lex("a = b * 2"));
}

#[test]
fn test_string() {
    assert_debug_snapshot!(lex(r#""plain string""#));
    assert_debug_snapshot!(lex(r#"a = "plain string int expression" + 1"#));
    assert_debug_snapshot!(lex(r#""string {{ with }} escaped bracket""#));
    assert_debug_snapshot!(lex(r#""string \" with \" escaped \ quotes""#));
    assert_debug_snapshot!(lex(r#""string {with} expression""#));
    assert_debug_snapshot!(lex(r#""string {more + complex} expression""#));
    assert_debug_snapshot!(lex(r#""string {formated - expression:03.5} expression""#));
}

#[test]
fn test_larger() {
    assert_debug_snapshot!(lex("fn(a: Length) -> Length {a * 2}"));
    assert_debug_snapshot!(lex("a = 1 // comment"));
    assert_debug_snapshot!(lex(r#"a = 1; /** multi
    line
    comment
    */
    b = 2;"#));
    assert_debug_snapshot!(lex(r#"/// Doc comment
    part Foo() {
        Cylinder(height = 10mm, radius = 5mm);
    }"#));
}