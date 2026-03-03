# Function Results

Of course, functions can also return a value.
To do so, you need to define the *return type* (using `->`) as shown in the following example:

[![test](.test/function_result.svg)](.test/function_result.log)

```µcad,function_result
fn square( x: Scalar ) -> Scalar {
    x * x
}

std::debug::assert_eq([ square(8.0), 64.0 ]);
```

You may also use `if` to decide between different results.

[![test](.test/function_conditional_result.svg)](.test/function_conditional_result.log)

```µcad,function_conditional_result#todo
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

Of course returning a value twice is not allowed.

[![test](.test/result_twice.svg)](.test/result_twice.log)

```µcad,result_twice#fail
fn f( n: Integer ) -> Scalar {
    if n > 1 {
        1.0
    } else if n < 1 {
        -1.0
    }
    0.0 // error: without else this line would return a second value
}

std::debug::assert_eq([ f(32), 1.0 ]);
std::debug::assert_eq([ f(0), 0.0 ]);
std::debug::assert_eq([ f(-2), -1.0 ]);
```

[![test](.test/result_wrong.svg)](.test/result_wrong.log)

```µcad,result_wrong#fail
fn f(b: Bool, x: Length) -> Length {
    if b {
        x
    } else {   // error: results in Aera
        x * x
    }
}

f(true, 1mm);
```

[![test](.test/result_wrong_next_if.svg)](.test/result_wrong_next_if.log)

```µcad,result_wrong_next_if#fail
fn f(b: Bool, x: Length) -> Length {
    if b {
        x
    } else if x < 1 {   // error: results in Aera
        x * x
    }
}

f(true, 1mm);
```

## Early Return

It is also possible to implement an *early return* pattern with the `return`
statement.

[![test](.test/function_return.svg)](.test/function_return.log)

```µcad,function_return#todo
fn pow( x: Scalar, n: Integer ) -> Scalar {
    if n > 1 {
        return x * pow(x, n-1);
    }
    if n < 1 {
        return 1.0 / x / pow(x, -n);
    }
    1.0
}

std::debug::assert_eq([ pow(8.0,2), 64.0 ]);
std::debug::assert_eq([ pow(8.0,-2), 0.015625 ]);
```
