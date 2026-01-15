# Operators

> [!NOTE]
> Do not confuse *operators* with [*operations*](../structure/workbenches/types/operations.md)

There are several operators which can be used to combine *expressions* with
each other:

| Operator |  Type  |      Input types       |      Result Type       | Description      |
| :------: | :----: | :--------------------: | :--------------------: | ---------------- |
|   `-`    | unary  | Integer, Scalar, Array | Integer, Scalar, Array | Negation         |
|   `+`    | binary | Integer, Scalar, Array | Integer, Scalar, Array | Addition         |
|   `-`    | binary | Integer, Scalar, Array | Integer, Scalar, Array | Subtraction      |
|   `*`    | binary | Integer, Scalar, Array | Integer, Scalar, Array | Multiplication   |
|   `/`    | binary | Integer, Scalar, Array | Integer, Scalar, Array | Division         |
|   `^`    | binary |    Integer, Scalar     |    Integer, Scalar     | Power            |
|   `&`    | binary |        Boolean         |        Boolean         | Logical AND      |
|   `\|`   | binary |        Boolean         |        Boolean         | Logical OR       |
|   `>`    | binary |    Integer, Scalar     |        Boolean         | Greater than     |
|   `>=`   | binary |    Integer, Scalar     |        Boolean         | Greater or equal |
|   `<`    | binary |    Integer, Scalar     |        Boolean         | Less than        |
|   `<=`   | binary |    Integer, Scalar     |        Boolean         | Less or equal    |
|   `==`   | binary |    Integer, Scalar     |        Boolean         | Equal            |
|   `!=`   | binary |    Integer, Scalar     |        Boolean         | Not equal        |

Here are some examples of each operator:

[![test](.test/operator_examples.svg)](.test/operator_examples.log)

```µcad,operator_examples
use std::debug::assert_eq; // used for testing

assert_eq([ -5,     0 - 5               ]); // Negation
assert_eq([ 5 + 6,  11                  ]); // Addition
assert_eq([ 5 - 6,  -1                  ]); // Subtraction
assert_eq([ 5 * 6,  30                  ]); // Multiplication
assert_eq([ 5 / 6,  0.83333333333333333 ]); // Division
assert_eq([ 5 ^ 6,  15625               ]); // Power
assert_eq([ true &  false, false        ]); // Logical AND
assert_eq([ true |  false, true         ]); // Logical OR
assert_eq([ 5 > 6,  false               ]); // Greater than
assert_eq([ 5 >= 6, false               ]); // Greater or equal
assert_eq([ 5 < 6,  true                ]); // Less than
assert_eq([ 5 <= 6, true                ]); // Less or equal
assert_eq([ 5 == 6, false               ]); // Equal
assert_eq([ 5 != 6, true                ]); // Not equal
```

## Operators & Arrays

Some of the operators listed above can be used with arrays too.
There result then is a new array with each value processed with the operator
and the second operand.

[![test](.test/operator_array.svg)](.test/operator_array.log)

```µcad,operator_array
use std::debug::assert_eq; // used for testing

assert_eq([ -[1, 2, 3, 4]           , [-1, -2, -3, -4]     ]); // Negation
assert_eq([ [1, 2, 3, 4] + 5        , [6, 7, 8, 9]         ]); // Addition
assert_eq([ [1, 2, 3, 4] - 5        , [-4, -3, -2, -1]     ]); // Subtraction
assert_eq([ [1, 2, 3, 4] * 5        , [5, 10, 15, 20]      ]); // Multiplication
assert_eq([ [1.0, 2.0, 3.0, 4.0] / 5, [0.2, 0.4, 0.6, 0.8] ]); // Division
```
