# Building Code

The *building code* is executed after any initialization.
Usually it produces one or many 2D or 3D objects on base of the given
*building plan*.

[![test](.test/code.svg)](.test/code.log)

```µcad,code
sketch Wheel(radius: Length) {
    // building code starts here
    std::geo2d::Circle(radius);
}

Wheel(radius = 1.0mm)
```

If *initializers* were defined the *building code* starts below them.

[![test](.test/code_post_init.svg)](.test/code_post_init.log)

```µcad,code_post_init
sketch Wheel(radius: Length) {
    // initializer
    init( diameter: Length ) { radius = diameter / 2; }

    // building code starts here
    std::geo2d::Circle(radius);
}
```

## Rules

### Illegal statements within workbenches

It's **not allowed** to use the `sketch`, `part`, `op`, `return` nor `mod` statements within workbench code:

[![test](.test/illegal_workbench_statement_sketch.svg)](.test/illegal_workbench_statement_sketch.log)

```µcad,illegal_workbench_statement_sketch#fail
sketch Wheel(radius: Length) {
    sketch Axis(length: Length) {}  // error
}
Wheel(radius = 1.0mm);
```

[![test](.test/illegal_workbench_statement_mod.svg)](.test/illegal_workbench_statement_mod.log)

```µcad,illegal_workbench_statement_mod#fail
sketch Wheel(radius: Length) {
    mod m {}                        // error
}
Wheel(radius = 1.0mm);
```

[![test](.test/illegal_workbench_statement_return.svg)](.test/illegal_workbench_statement_return.log)

```µcad,illegal_workbench_statement_return#fail
sketch Wheel(radius: Length) {
    return;                         // error
}
Wheel(radius = 1.0mm);
```
