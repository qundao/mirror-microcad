# Model Expressions

Things change when an expression consists of *models* instead of just *values*.
We call this a *model expression*:

[![test](.test/expression_model.svg)](.test/expression_model.log)

```µcad,expression_model
std::geo2d::Rect(1cm) - std::geo2d::Circle(3mm);
```

This is an expression too — one that contains an element, specifically a
[call](../../flow/calls/workbench_calls.md) to `Rect`, which draws a rectangle
that will actually be rendered into this:

Output
  :![output](.test/expression_model-out.svg)

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

Output
  :![output](.test/if_expression-out.svg)
