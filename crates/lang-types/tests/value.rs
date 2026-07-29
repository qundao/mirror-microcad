use microcad_lang_types::{Integer, Scalar, Value, ValueResult};

fn integer(value: i64) -> Value {
    Integer::from_num(value).into()
}

fn scalar(value: f64) -> Value {
    Scalar::from_num(value).into()
}

fn check(result: ValueResult, value: Value) {
    let result = result.expect("error result");
    assert_eq!(result, value);
}

#[test]
fn test_value_integer() {
    let u = || integer(2);
    let v = || integer(5);
    let w = || scalar(5.0);

    // symmetric operations
    check(u() + v(), integer(2 + 5));
    check(u() - v(), integer(2 - 5));
    check(u() * v(), integer(2 * 5));
    check(u() / v(), scalar(2.0 / 5.0));
    check(-u(), integer(-2));

    // asymmetric operations
    check(u() + w(), scalar(2.0 + 5.0));
    check(u() - w(), scalar(2.0 - 5.0));
    check(u() * w(), scalar(2.0 * 5.0));
    check(u() / w(), scalar(2.0 / 5.0));
}

#[test]
fn test_value_scalar() {
    let u = || scalar(2.0);
    let v = || scalar(5.0);
    let w = || integer(5);

    // symmetric operations
    check(u() + v(), scalar(2.0 + 5.0));
    check(u() - v(), scalar(2.0 - 5.0));
    check(u() * v(), scalar(2.0 * 5.0));
    check(u() / v(), scalar(2.0 / 5.0));
    check(-u(), scalar(-2.0));

    // asymmetric operations
    check(u() + w(), scalar(2.0 + 5.0));
    check(u() - w(), scalar(2.0 - 5.0));
    check(u() * w(), scalar(2.0 * 5.0));
    check(u() / w(), scalar(2.0 / 5.0));
}
