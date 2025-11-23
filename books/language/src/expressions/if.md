# If statements

## If statements in workbenches

Inside a workbench block, an if statement can be used to select different shapes or constructions depending on input parameters.

[![test](.test/if_statement.svg)](.test/if_statement.log)

```µcad,if_statement
use std::ops::*;
use std::math::*;
use std::geo2d::*;

sketch MySketch(a: Integer) {
    if a == 1 {
        Circle(1cm)
    } else {
        Rect(1cm)
    }
}

MySketch([1,2]).align(X, 1cm);
```

Multiple conditions can be chained, allowing more than two alternatives:

![output](.test/if_statement-out.svg)

[![test](.test/chained_if_statement.svg)](.test/chained_if_statement.log)

```µcad,chained_if_statement
use std::ops::*;
use std::math::*;
use std::geo2d::*;

sketch MySketch(a: Integer) {
    if a == 1 {
        Circle(1cm)
    } else if a == 2 {
        Rect(1cm)
    } else {
        Hexagon(1cm)
    }
}

MySketch([1,2,3]).align(X, 1cm);
```

![output](.test/chained_if_statement-out.svg)
