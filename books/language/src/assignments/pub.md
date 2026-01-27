# Public Assignments

Public assignments provide a constant to the inner **and** the outer of a module.

[![test](.test/pub_assignment.svg)](.test/pub_assignment.log)

```µcad,pub_assignment
mod my_module {
    pub TEXT = "Hello";  // error
}

std::print(my_module::TEXT);
```

Using `pub` is not allowed within workbenches:

[![test](.test/pub_assignment_workbench.svg)](.test/pub_assignment_workbench.log)

```µcad,pub_assignment_workbench#fail
sketch MySketch() {
    pub TEXT = "Hello";  // error
    std::print(TEXT);
}

MySketch();
```

## Rules

### Not in building code

Public assignments cannot be used in building code (the code below any initializers).

[![test](.test/pub_assignment_building_code.svg)](.test/pub_assignment_building_code.log)

```µcad,pub_assignment_building_code#fail
sketch MySketch() {
    init(_: Integer) {}

    pub MY_CONST = 1;   // error: not allowed in building code
}
MySketch();
```

Public assignments must be on top of the workbench code if initializers are not used.

[![test](.test/pub_assignment_workbench_code.svg)](.test/pub_assignment_workbench_code.log)

```µcad,pub_assignment_workbench_code
sketch MySketch() {
    const MY_CONST = 1;   // allowed if no initializers and on top
    
    _i = 5;                 // any non const code
}
MySketch();
```

[![test](.test/pub_assignment_workbench_code_wrong.svg)](.test/pub_assignment_workbench_code_wrong.log)

```µcad,pub_assignment_workbench_code_wrong#todo_fail
sketch MySketch() {
    _i = 5;                 // any non const code
    
    const MY_CONST = 1;     // error: const not allowed then
}
MySketch();
```

### Not in initialization code

Public assignments cannot be used in building code (the code below any initializers).

[![test](.test/pub_assignment_init_code.svg)](.test/pub_assignment_init_code.log)

```µcad,pub_assignment_init_code#fail
sketch MySketch() {
    pub MY_CONST = 1;   // error: not allowed in init code

    init(_: Integer) {}
}
MySketch();
```

### Not in function

[![test](.test/pub_assignment_fn.svg)](.test/pub_assignment_fn.log)

```µcad,pub_assignment_fn#fail
fn f() {
    const MY_CONST = 1;     // error: not allowed in functions
}
f();

sketch MySketch() {
    fn f() {
        const MY_CONST = 1; // error: not allowed in workbench functions
    }
    f();
}
MySketch();
```

### Not in initializers

[![test](.test/pub_assignment_init.svg)](.test/pub_assignment_init.log)

```µcad,pub_assignment_init#todo_fail
sketch MySketch() {
    init(_: Integer) {
        const MY_CONST = 1;   // error: not allowed in initializers
    }
}
MySketch();
```
