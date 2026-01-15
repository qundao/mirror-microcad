# Expressions

An *expression* defines a value simply by a [literal](literals.md) or by
combining multiple other expressions.
For example, we can multiply a quantity of 4 millimeters with a factor 5 and
assign it to a constant `v`:

[![test](.test/expression_multiply.svg)](.test/expression_multiply.log)

```µcad,expression_multiply
v = 5 * 4.0mm;

std::debug::assert_eq([ v, 20mm ]);
```

The result of this expression would be `20mm` like the test (see `assert_eq`)
proves.
In this form, the computed value is not yet used for anything, so the examples
above will not produce any visible output or effect.
