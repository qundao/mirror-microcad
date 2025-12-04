# Functions

[![test](.test/function.svg)](.test/function.log)

```µcad,function
// function definition
fn f() {}
// call
f();
```

[![test](.test/function_value.svg)](.test/function_value.log)

```µcad,function_value
// parameter
fn f(n: Scalar) {
    // result value
    n+1
}
use std::debug::*;
assert_eq([ f(1), 2 ]);
assert_eq([ f(4), 5 ]);
```

[![test](.test/function_return.svg)](.test/function_return.log)

```µcad,function_return
fn f(n: Scalar) {
    return n+1;
}
use std::debug::*;
assert_eq([ f(1), 2 ]);
assert_eq([ f(4), 5 ]);
```

[![test](.test/function_if.svg)](.test/function_if.log)

```µcad,function_if
fn f(n: Scalar) {
    if n > 3 {
        n-1
    } else {
        n+1
    }
}
use std::debug::*;
assert_eq([ f(1), 2 ]);
assert_eq([ f(4), 3 ]);
```

[![test](.test/function_mixed.svg)](.test/function_mixed.log)

```µcad,function_mixed
fn f(n: Scalar) {
    if n > 3 {
        return n-1;
    }
    n+1
}
use std::debug::*;
assert_eq([ f(1), 2 ]);
assert_eq([ f(4), 3 ]);
```

[![test](.test/function_missing.svg)](.test/function_missing.log)

```µcad,function_missing#todo_fail
fn f(n: Scalar) { // error: not all paths return a value
    if n > 3 {
         n-1
    }
}
use std::debug::*;
assert_eq([ f(1), 2 ]);
assert_eq([ f(4), 3 ]);
```
