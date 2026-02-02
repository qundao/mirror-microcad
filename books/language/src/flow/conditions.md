# Conditionals

Conditions lead to different executions paths for different cases.

## If statements in workbenches

Inside a workbench block, an if statement can be used to select different shapes or constructions depending on input parameters.

[![test](.test/if_statement.svg)](.test/if_statement.log)

```µcad,if_statement
use std::ops::*;
use std::math::*;
use std::geo2d::*;

sketch MySketch(a: Integer) {
    if a == 1 {
        Circle(r = 1cm)
    } else {
        Rect(s = 1cm)
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
        Circle(r = 1cm)
    } else if a == 2 {
        Rect(s = 1cm)
    } else {
        Hexagon(r = 1cm)
    }
}

MySketch([1,2,3]).align(X, 1cm);
```

![output](.test/chained_if_statement-out.svg)

## If Statement for functions

[![test](.test/if_functions.svg)](.test/if_functions.log)

```µcad,if_functions
fn f(x: Integer) {
    if x == 5 or x == 4 {
        std::print("match");
    } else if x > 0 and x < 4 {
        std::print("no match");
    } else {
        std::print("invalid");
    }
}

f(5);  // prints "match"
f(1);  // prints "no match"
f(-1); // prints "invalid"
f(6);  // prints "invalid"
```
