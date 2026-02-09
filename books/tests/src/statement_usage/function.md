# Function

[![test](.test/function_workbench.svg)](.test/function_workbench.log)

```µcad,function_workbench#fail
fn f() {
  sketch S() {} // error
} f();
```

[![test](.test/function_module.svg)](.test/function_module.log)

```µcad,function_module#fail
fn f() {
  mod m {} // error
} f();
```

[![test](.test/function_function.svg)](.test/function_function.log)

```µcad,function_function#fail
fn f() {
  fn f() {} // error
} f();
```

[![test](.test/function_init.svg)](.test/function_init.log)

```µcad,function_init#fail
fn f() {
  init() {} // error
} f();
```

[![test](.test/function_use.svg)](.test/function_use.log)

```µcad,function_use
fn f() {
  use std;
} f();
```

[![test](.test/function_pub_use.svg)](.test/function_pub_use.log)

```µcad,function_pub_use#fail
fn f() {
  pub use std; // error
} f();
```

[![test](.test/function_return.svg)](.test/function_return.log)

```µcad,function_return
fn f() {
  return 1;
} f();
```

[![test](.test/function_if.svg)](.test/function_if.log)

```µcad,function_if
fn f() {
  if std::math::PI == 3 { __builtin::geo2d::Circle(radius=1); }
} f();
```

[![test](.test/function_assignment_const.svg)](.test/function_assignment_const.log)

```µcad,function_assignment_const#fail
fn f() {
  const B = 1; // error
} f();
```

[![test](.test/function_assignment_pub.svg)](.test/function_assignment_pub.log)

```µcad,function_assignment_pub#fail
fn f() {
  pub p = 1; // error
} f();
```

[![test](.test/function_assignment_var.svg)](.test/function_assignment_var.log)

```µcad,function_assignment_var
fn f() {
  a = 1;
} f();
```

[![test](.test/function_assignment_prop.svg)](.test/function_assignment_prop.log)

```µcad,function_assignment_prop#fail
fn f() {
  prop a = 1; // error
} f();
```

[![test](.test/function_expression.svg)](.test/function_expression.log)

```µcad,function_expression
fn f() {
  1 + 2;
} f();
```

[![test](.test/function_expression_model.svg)](.test/function_expression_model.log)

```µcad,function_expression_model
fn f() {
  __builtin::geo2d::Circle(radius=1mm);
} f();
```
