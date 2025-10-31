# Pre-Init

[![test](.test/pre_init_workbench.svg)](.test/pre_init_workbench.log)

```µcad,pre_init_workbench#fail
sketch K() { 
  sketch F() {} K();  // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_module.svg)](.test/pre_init_module.log)

```µcad,pre_init_module#fail
sketch K() { 
  mod m {}   // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_function.svg)](.test/pre_init_function.log)

```µcad,pre_init_function#fail
sketch K() { 
  fn f() {} f();   // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_init.svg)](.test/pre_init_init.log)

```µcad,pre_init_init
sketch K() { 
  init() {}
init(l:Length) {} } K();
```

[![test](.test/pre_init_use.svg)](.test/pre_init_use.log)

```µcad,pre_init_use
sketch K() { 
  use std;
init(l:Length) {} } K();
```

[![test](.test/pre_init_pub_use.svg)](.test/pre_init_pub_use.log)

```µcad,pre_init_pub_use#fail
sketch K() { 
  pub use std; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_return.svg)](.test/pre_init_return.log)

```µcad,pre_init_return#fail
sketch K() { 
  return 1; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_if.svg)](.test/pre_init_if.log)

```µcad,pre_init_if#fail
sketch K() { 
  if std::math::PI == 3 { } // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_assignment_const.svg)](.test/pre_init_assignment_const.log)

```µcad,pre_init_assignment_const#fail
sketch K() { 
  const B = 1; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_assignment_var.svg)](.test/pre_init_assignment_var.log)

```µcad,pre_init_assignment_var
sketch K() { 
  a = 1;
init(l:Length) {} } K();
```

[![test](.test/pre_init_assignment_prop.svg)](.test/pre_init_assignment_prop.log)

```µcad,pre_init_assignment_prop#fail
sketch K() {
  prop a = 1; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_expression.svg)](.test/pre_init_expression.log)

```µcad,pre_init_expression#fail
sketch K() { 
  1 + 2; // error
init(l:Length) {} } K();
```

[![test](.test/pre_init_expression_model.svg)](.test/pre_init_expression_model.log)

```µcad,pre_init_expression_model#fail
sketch K() { 
  __builtin::geo2d::Circle(radius=1); // error
init(l:Length) {} }
```
