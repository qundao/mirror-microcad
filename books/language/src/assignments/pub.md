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
