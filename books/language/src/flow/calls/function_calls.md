# Function Calls

A function is simply called by appending a parameter list to it's name:

[![test](.test/function_call.svg)](.test/function_call.log)

```µcad,function_call
// function definition
fn f() {}

// function call
f();
```

[![test](.test/function_return.svg)](.test/function_return.log)

```µcad,function_return
// function definition
fn f() {
    // return statement
    return 1; 
}

// function call (and result check)
std::debug::assert_eq([ f(), 1 ]);
```

[![test](.test/function_param_return.svg)](.test/function_param_return.log)

```µcad,function_param_return
// function definition with parameter
fn f(n: Integer) {
    return n * 2; 
}

// function calls with parameter (and result checks)
std::debug::assert_eq([ f(1), 2 ]);
std::debug::assert_eq([ f(2), 4 ]);
```
