# If Statement

```µcad,if_value#todo
fn f(a:Scalar) {
    x = if a > 5 { a } else { a*2 };
    return x;
}

use std::debug::*;
assert_eq([ f(4), 8 ]);
assert_eq([ f(6), 6 ]);
```

```µcad,if_model#todo
use std::geo2d::*;
use std::debug::*;

fn f(a:Length) {
    m = if a > 5mm { Circle(a) } else { Circle(a*2) };
    return m;
}

assert_eq([ f(4mm), Circle(8mm) ]);
assert_eq([ f(6mm), Circle(6mm) ]);
```
