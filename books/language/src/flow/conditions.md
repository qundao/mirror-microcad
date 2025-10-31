# Conditionals

Conditions lead to different executions paths for different cases.

## If Statement for models

[![test](.test/if_models.svg)](.test/if_models.log)

```µcad,if_models
sketch MySketch(a: Integer) {
    if a == 1 {
        std::geo2d::Circle(radius = 4mm)
    } else {
        std::geo2d::Circle(radius = 10mm)
    }
}

MySketch(2) - MySketch(1);
```

![test](.test/if_models-out.svg)

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
