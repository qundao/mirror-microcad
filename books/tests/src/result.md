# Result

- if is an expression
- semicolon...
  - eliminates a value of an expression
  - DOES NOT eliminate a model of an expression
- statement lists are only in...
  - module body
  - source files
  - bodies:
    - function body
    - workbench body
    - expression:
      - if body
      - model group body

## Statements

Statements can be put into groups:

1) Within functions, workbenches, inits and expression bodies `use` and `const` put symbols onto the evaluation stack.
2) Within modules or source files `use` and `const` as well as `pub` and `mod` put symbols into the symbol table.

## Module Body

No building code within module bodies - so no statements

## Source Files & Workbenches

A source file is a module, but all top-level statements which are not `use`, `const`, `pub` or `mod` are interpreted as building code.

[![test](.test/source_file.svg)](.test/source_file.log)

```µcad,source_file
use std::geo2d::*;
1;
Circle(1mm);
2;
Circle(2mm);
3;
```

They can have mixed code which shall not end in a value result.

[![test](.test/source_file_value.svg)](.test/source_file_value.log)

```µcad,source_file_value
use std::geo2d::*;
1;
Circle(1mm);
2;
Circle(2mm);
3       // error
```

## Workbenches

Same with workbenches but models are not returned but accumulated into a Group:

[![test](.test/sketch.svg)](.test/sketch.log)

```µcad,sketch
sketch S() {
    use std::geo2d::*;
    1;
    Circle(1mm);
    2;
    Circle(2mm);
    3;
}
S();
```

Workbenches cannot end with a value result.

[![test](.test/sketch_value.svg)](.test/sketch_value.log)

```µcad,sketch_value#fail
sketch S() {
    1 // error
}
```

Workbenches cannot have `return` statements.

[![test](.test/sketch_return.svg)](.test/sketch_return.log)

```µcad,sketch_return#fail
sketch T() {
    return 1; // error
}
```

## Functions

Functions can return values.

[![test](.test/function_ok.svg)](.test/function_ok.log)

```µcad,function_ok
fn f() -> Length { 1mm }
fn g() {}
f();
g();
```

Functions can NOT generate models.

[![test](.test/function_model.svg)](.test/function_model.log)

```µcad,function_model#fail
fn f() {
    std::geo2d::Circle(1mm); // error
}
f();
```

Functions can NOT result in models.

[![test](.test/function_model_result.svg)](.test/function_model_result.log)

```µcad,function_model_result#fail
fn f() -> Model  {  // error: Model cannot be used as return type of a function
    std::geo2d::Circle(1mm) // error
}
f();
```

Functions can NOT return models.

[![test](.test/function_model_return.svg)](.test/function_model_return.log)

```µcad,function_model_return#fail
fn f() -> Model  {  // error: Model cannot be used as return type of a function
    return std::geo2d::Circle(1mm); // error
}
f();
```

Functions can read model properties.

[![test](.test/function_model_property.svg)](.test/function_model_property.log)

```µcad,function_model_property
fn f() -> Length {
    Circle(2mm).radius
}
f();
```

Functions can return model properties.

[![test](.test/function_model_property_return.svg)](.test/function_model_property_return.log)

```µcad,function_model_property_return
fn f() -> Length {
    return Circle(2mm).radius;
}
f();
```

Functions must return a result if one is declared.

[![test](.test/function_missing_result.svg)](.test/function_missing_result.log)

```µcad,function_missing_result#fail
fn f() -> Length {} // error
f();
```

Functions MUST NOT have a result if none is declared.

[![test](.test/function_unexpected_result.svg)](.test/function_unexpected_result.log)

```µcad,function_unexpected_result#fail
fn f() { 
    1 // error
} 
f();
```

Functions results must be of correct type.

[![test](.test/function_wrong_result.svg)](.test/function_wrong_result.log)

```µcad,function_wrong_result#fail
fn f() -> Length {
    1 // error
}
f();
```

Functions must return a value of the declared result type.

[![test](.test/function_wrong_return.svg)](.test/function_wrong_return.log)

```µcad,function_wrong_return#fail
fn f() -> Length {
    return 1; // error
}
f();
```
