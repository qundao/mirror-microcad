# Workbench

[![test](.test/workbench_workbench.svg)](.test/workbench_workbench.log)

```µcad,workbench_workbench#fail
sketch K() {
  sketch F() {} F(); // error
} K();
```

[![test](.test/workbench_module.svg)](.test/workbench_module.log)

```µcad,workbench_module#fail
sketch K() {
  mod m {} // error
} K();
```

[![test](.test/workbench_function.svg)](.test/workbench_function.log)

```µcad,workbench_function
sketch K() {
  fn f() {} f();
} K();
```

[![test](.test/workbench_init.svg)](.test/workbench_init.log)

```µcad,workbench_init
sketch K(x: Scalar) {
  init() { x = 1; }
} K();
```

[![test](.test/workbench_use.svg)](.test/workbench_use.log)

```µcad,workbench_use
sketch K() {
  use std;
} K();
```

[![test](.test/workbench_pub_use.svg)](.test/workbench_pub_use.log)

```µcad,workbench_pub_use#fail
sketch K() {
  pub use std; // error
} K();
```

[![test](.test/workbench_return.svg)](.test/workbench_return.log)

```µcad,workbench_return#fail
sketch K() {
  return 1; // error
} K();
```

[![test](.test/workbench_if.svg)](.test/workbench_if.log)

```µcad,workbench_if
sketch K() {
  if std::math::PI == 3 { }
} K();
```

[![test](.test/workbench_marker.svg)](.test/workbench_marker.log)

```µcad,workbench_marker
sketch K() {
  @input
} K();
```

[![test](.test/workbench_assignment_const.svg)](.test/workbench_assignment_const.log)

```µcad,workbench_assignment_const
sketch K() {
  const B = 1;
} K();
```

[![test](.test/workbench_assignment_var.svg)](.test/workbench_assignment_var.log)

```µcad,workbench_assignment_var
sketch K() {
  a = 1;
} K();
```

[![test](.test/workbench_assignment_prop.svg)](.test/workbench_assignment_prop.log)

```µcad,workbench_assignment_prop
sketch K() {
  prop a = 1;
} K();
```

[![test](.test/workbench_expression.svg)](.test/workbench_expression.log)

```µcad,workbench_expression
sketch K() {
  1 + 2;
} K();
```

[![test](.test/workbench_expression_model.svg)](.test/workbench_expression_model.log)

```µcad,workbench_expression_model
sketch K() {
  __builtin::geo2d::Circle(radius=1mm);
} K();
```
