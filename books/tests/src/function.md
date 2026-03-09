# Functions

[![test](.test/function.svg)](.test/function.log)

```µcad,function
fn f() {}
f();
```

[![test](.test/function_value.svg)](.test/function_value.log)

```µcad,function_value
fn f(n: Integer) -> Integer {
    n+1
}
use std::debug::*;
assert_eq([ f(1), 2 ]);
assert_eq([ f(4), 5 ]);
```

[![test](.test/function_return_value.svg)](.test/function_return_value.log)

```µcad,function_return_value
fn f(n: Integer) -> Integer {
    return n+1;
}
use std::debug::*;
assert_eq([ f(1), 2 ]);
assert_eq([ f(4), 5 ]);
```

[![test](.test/function_if_result.svg)](.test/function_if_result.log)

```µcad,function_if_result
fn f(n: Integer) -> Integer {
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
fn f(n: Integer) -> Integer {
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
fn f(n: Integer) -> Integer { // error: not all paths return a value
    if n > 3 {
         n-1
    }
}
use std::debug::*;
assert_eq([ f(1), 2 ]);
assert_eq([ f(4), 3 ]);
```
