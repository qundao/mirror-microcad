# Operators

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
-5;             // Negation
5 + 6;          // Addition (result: 11)
5 - 6;          // Subtraction (result: -1)
5 * 6;          // Multiplication (result: 30)
5 / 6;          // Division (result: 0.833333333)
5 ^ 6;          // Power (result: 15625)
true & false;   // Logical AND (result: false)
true | false;   // Logical OR (result: true)
5 > 6;          // Greater than (result: false)
5 >= 6;         // Greater or equal (result: false)
5 < 6;          // Less than (result: true)
5 <= 6;         // Less or equal (result: true)
5 == 6;         // Equal (result: false)
5 != 6          // Not equal (result: true)
```

## Operators & Arrays

Some of the operators listed above can be used with arrays too.
There result then is a new array with each value processed with the operator
and the second operand.

[![test](.test/operator_array.svg)](.test/operator_array.log)

```µcad,operator_array
-[1, 2, 3, 4];      // Negation (result: [-1, -2, -3, -4])
[1, 2, 3, 4] + 5;   // Addition (result: [6, 7, 8, 9])
[1, 2, 3, 4] - 5;   // Subtraction (result: [-4, -3, -2, -1])
[1, 2, 3, 4] * 5;   // Multiplication (result: [5, 10, 15, 20])
[1, 2, 3, 4] / 5;   // Division (result: [0.2, 0.4, 0.6, 0.8])
```
