# Workbench Functions

A [workbench](../workbenches/README.md) can contain functions that are accessible
within the module only.

Here is an example which generates a punched disk of a given radius using a
function `inner()`:

[![test](.test/workbench_example.svg)](.test/workbench_example.log)

```µcad,workbench_example
sketch PunchedDisk(radius: Length) {
    use std::geo2d::Circle;

    // calculate inner from radius in a method
    fn inner() { return radius/2; }

    // generate donut (and call inner)
    Circle(radius) - Circle(radius = inner());
}

PunchedDisk(radius = 1cm);
```

## Restrictions

There are some restrictions for *workbench functions*:

### No public workbench functions

Trying to make them public with the keyword `pub` will result into an error:

[![test](.test/workbench_pub.svg)](.test/workbench_pub.log)

```µcad,workbench_pub#fail
part PunchedDisk(radius: Length) {
    pub fn inner() { return radius/2; }   // error: cant use pub fn inside workbench
}

PunchedDisk(4.0mm);
```

### No `prop` in workbench functions nor initializers

You cannot create *workbench properties* in *function bodies*.

[![test](.test/workbench_fn_prop.svg)](.test/workbench_fn_prop.log)

```µcad,workbench_fn_prop#fail
part PunchedDisk(radius: Length) {
    fn inner() {
        prop hole = radius/2;  // error: prop not allowed in function
        return hole;
    }
    inner();
}

PunchedDisk(1cm);
```

Also the `prop` keyword is not allowed in *initializers*.
Instead, the properties of the building plan must be set directly, without
using the `prop` keyword.

[![test](.test/workbench_init_prop.svg)](.test/workbench_init_prop.log)

```µcad,workbench_init_prop#fail
part PunchedDisk(radius: Length) {
    init(diameter: Length) { 
        prop radius = diameter/2; // error: prop not allowed in init
    }
    init(d: Length) { 
        radius = d/2;
    }

    // right place to set additional properties
    prop hole = radius/2;

    fn inner() { 
        return hole;
    }
    inner();
}

PunchedDisk(diameter=1cm);
PunchedDisk(d=1cm);
```
