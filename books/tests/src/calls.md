# Calls

[![test](.test/call_same_parameter_name.svg)](.test/call_same_parameter_name.log)

```µcad,call_same_parameter_name#fail
use std::geo2d::*;
use std::ops::*;

Rect(w = 10mm, w = 10mm); // error: same parameter name
```

[![test](.test/recursion.svg)](.test/recursion.log)

```µcad,recursion
fn pow( x: Scalar, n: Integer ) -> Scalar {
    if n > 1 {
        x * pow(x, n-1)
    } else if n < 1 {
        1.0 / x / pow(x, -n)
    } else {
        1.0
    }
}

std::debug::assert_eq([ pow(8.0,2), 64.0 ]);
std::debug::assert_eq([ pow(8.0,-2), 0.015625 ]);
```
