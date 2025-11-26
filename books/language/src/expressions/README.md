# Expressions

An *expression* defines a value by combining multiple other expressions.
The simplest form of an expression are [literals](literals.md):

[![test](.test/expression_literals.svg)](.test/expression_literals.log)

```µcad,expression_literals
5;        // Scalar literal
4.0mm;    // Quantity literal
"Hello";  // String literal
```

However — as mentioned before — literals can be combined into larger
expressions.
For example, we can multiply the value `5` by the quantity `4.0mm`:

[![test](.test/expression_multiply.svg)](.test/expression_multiply.log)

```µcad,expression_multiply
5 * 4.0mm;
```

The result of this expression would be `20mm`.  
In this form, the computed value is not yet used for anything, so the examples
above will not produce any visible output or effect.

Things change when an expression generates a *model* instead of just a *value*.
We call this a *model expression*:

[![test](.test/expression_model.svg)](.test/expression_model.log)

```µcad,expression_model
std::geo2d::Rect(1cm);
```

This too is an expression — one that contains an element, specifically a
[call](../../flow/calls/workbench_calls.md) to `Rect`, which draws a rectangle
that will actually be rendered into this:

![output](.test/expression_model-out.svg)

If statements can also be used as an expression, evaluating to the value from
the chosen branch.

[![test](.test/if_expression.svg)](.test/if_expression.log)

```µcad,if_expression
use std::ops::*;
use std::math::*;
use std::geo2d::*;

sketch MySketch(a: Integer) {
    outer = if a == 1 {
        Circle(1cm)
    } else if a == 2 {
        Rect(1cm)
    } else {
        Hexagon(1cm)
    };
    outer - Circle(3mm)
}

MySketch([1,2,3]).align(X, 1cm);
```

![output](.test/if_expression-out.svg)