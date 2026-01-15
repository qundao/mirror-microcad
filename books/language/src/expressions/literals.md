# Literals

Literals are the simplest form of expressions.
In this section, we will look at some literals.

[![test](.test/literal.svg)](.test/literal.log)

```µcad,expression_literals
5;        // Scalar literal
4.0mm;    // Quantity literal
"Hello";  // String literal
```

There are several types of literals:

| Name                               | Encoding               | Description                            |
| ---------------------------------- | ---------------------- | -------------------------------------- |
| [*Integer*](#integer-literals)     | 64 bit[^bits]  integer | signed integer                         |
| [*Scalar*](#scalar-literals)       | 64 bit[^bits] float    | signed floating point                  |
| [*Boolean*](#boolean-literals)     | 1 bit bool             | boolean                                |
| [*String*](#string-literals)       | UTF-8                  | Text                                   |
| [*Quantities*](#quantity-literals) | 64 bit[^bits] float    | signed floating point (including type) |

[^bits]: On 64 bit systems.

## Integer Literals

Integer literals contain a whole number with a sign (but without a unit).
Here are a few examples:

[![test](.test/integer_literal.svg)](.test/integer_literal.log)

```µcad,integer_literal
50;
1350;
-6
```

## Scalar Literals

Scalar literals contain a
[floating point number](https://en.wikipedia.org/wiki/Double-precision_floating-point_format)
with a sign (but without a unit).

[![test](.test/scalar_literal.svg)](.test/scalar_literal.log)

```µcad,scalar_literal
1.23;
0.3252;
.4534;
1.;
-1200.0;
12.0E+12;
50%    // = 0.5
```

## Boolean Literals

Booleans contain either the value `true` or `false`:

[![test](.test/boolean_literal.svg)](.test/boolean_literal.log)

```µcad,boolean_literal
true;
false
```

## String Literals

Strings are texts enclosed in quotation marks:

[![test](.test/string_literal.svg)](.test/string_literal.log)

```µcad,string_literal
"Hello, World!"
```

## Quantity Literals

Quantities are like scalars but with a unit and are widely used in microcad if
you wanna draw something.

[![test](.test/quantity_literal.svg)](.test/quantity_literal.log)

```µcad,quantity_literal
4.0mm;   // 4 millimeters
5l;      // 5 liters
4m²;     // 4 square meters
4m2;     // also 4 square meters
10°;     // 10 degree angle
10deg    // also 10 degree angle
```

You will find more details about quantities and units in [this section](../../types/quantities.md#quantity-literals).
