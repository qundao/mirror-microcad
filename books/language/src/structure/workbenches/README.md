# Workbenches

## Workbench Types

*Workbenches* are µcad code constructs used to:

- create 2D [*sketches*](sketches.md) using `sketch`,
- build 3D [*parts*](parts.md) using `part`, or
- apply [*operations*](operations.md) to them using `op` workbenches.

Essentially, a *workbench* is initialized with a set of *parameters* and possibly some *initialization code*, then executes *code* that generates 2D and 3D *objects*.

## Workbench Declaration

### Workbench Elements

A *workbench* consists of the following elements:

- A leading keyword: `part`, `sketch`, or `op`,
- an **identifier** that names the workbench,
- a **building plan** defined by a *parameter list* following the identifier,
- optional **init code**, which is executed before any *initializer*,
- optional **initializers**, offering alternative ways to initialize the *building plan*,
- optional **functions**, acting as subroutines with their own parameters and code bodies,
- optional **properties**, accessible from outside and set from the inside,
- and typically some **building code**, which runs after all initialization steps and generates the final *objects*.

The following code demonstrates most of these elements:

[![test](.test/part_declaration.svg)](.test/part_declaration.log)

```µcad,part_declaration
// Sketch with a `radius` as building plan.
// Which will automatically become a property.
sketch Wheel(radius: Length) {
    
    // Init code...
    const FACTOR = 2;

    // Initializer #1
    init(diameter: Length) {
        // must set `radius`
        radius = diameter / FACTOR;
    }

    // No code in between!

    // Initializer #2
    init(r: Length) {
        // must set `radius`
        radius = r;
    }

    // Function (sub routine)
    fn into_diameter(r: Length) {
        return r * FACTOR;
    }

    // Building code...

    // Set a property which can be seen from outside.
    prop diameter = into_diameter(radius);
    
    // Local variable `r`
    r = radius;
    
    // Create a circle.
    std::geo2d::Circle(r);
}

use std::debug::*;

// Call sketch with diameter.
d = Wheel(diameter = 2cm);
// Check radius property.
assert_eq([d.radius, 1cm]);

// Call sketch with radius.
r = Wheel(radius = 2cm);
// Check diameter property.
assert_eq([r.diameter, 4cm]);

d - r;
```

![test](.test/part_declaration-out.svg)

### Building Plan

The *building plan* is defined by a *parameter list* that follows the workbench's
*identifier*.
All *parameters* in that list become *properties* of the workbench when it is invoked.
These properties can be accessed within the *building code*, inside functions,
or externally.

[![test](.test/building_plan.svg)](.test/building_plan.log)

```µcad,building_plan
// sketch with a `radius` as building plan
sketch Wheel(radius: Length) {
    // access property `radius` from the building plan
    std::geo2d::Circle(radius);
}

std::debug::assert_eq([Wheel(5cm).radius, 5cm]);
```

### Initializers

*Initializers* are defined with the keyword `init` and a following *parameter list*.
One may define multiple initializers which must have different parameter lists.

[![test](.test/initializers.svg)](.test/initializers.log)

```µcad,initializers#todo_fail
sketch Wheel(radius: Length) {
    init( radius: Length ) {} // error: same parameters as in building plan
    std::geo2d::Circle(1mm);
}

Wheel(radius = 1.0mm);
```

However, if an initializer is used, all properties from the building plan must
be initialized (except those with *default values*).

[![test](.test/init_property.svg)](.test/init_property.log)

```µcad,init_property
sketch Wheel(radius: Length, thickness: Length) {
    // initializer with diameter
    init( diameter: Length, thickness: Length ) {
        // must set `radius` in code 
        radius = diameter / 2;

        // thickness (from the building plan) does not need 
        // to be set, because it was automatically set by 
        // parameter of this initializer
    }

    // Now radius and thickness can be used
    std::geo2d::Circle(radius) - std::geo2d::Circle(radius = radius - thickness);
}

// call with building plan
Wheel(radius=5cm, thickness=1cm);
// call with initializer
Wheel(diameter=10cm, thickness=1cm);
```

If the *building plan* is not fully initialized by an initializer
you will get an error:

[![test](.test/missed_property.svg)](.test/missed_property.log)

```µcad,missed_property#fail
sketch Wheel(radius: Length) {
    init( width: Length ) { _ = width; } // error: misses to set `radius` from building plan
}

Wheel(width = 1.0mm);
```

### Init Code

If you use *initializers* you might write some *init code*
which must be placed on top of the workbench's body (before any *initializers*).

The *init code* is just allowed to define some *constants* which then can be used
in all following code (including code within *initializers* and *functions*).

[![test](.test/pre_init_code.svg)](.test/pre_init_code.log)

```µcad,pre_init_code
sketch Wheel(radius: Length) {
    // init code
    const FACTOR = 2.0;

    // initializer
    init(diameter: Length) { radius = into_radius(diameter); }

    // function
    fn into_radius( diameter: Length ) {
        // use constant FACTOR from init code
        return diameter / FACTOR;
    }

    // set property diameter and use FACTOR from init code
    prop diameter = radius * FACTOR;
    
    // code body
    std::geo2d::Circle(radius);
}

__builtin::debug::assert(Wheel(radius = 5cm).radius == 5cm);
__builtin::debug::assert(Wheel(radius = 5cm).diameter == 10cm);
__builtin::debug::assert(Wheel(diameter = 10cm).diameter == 10cm);
```

### Init Code Rules

It's **not allowed** to write any code between *initializers*.

[![test](.test/code_between_initializers.svg)](.test/code_between_initializers.log)

```µcad,code_between_initializers#fail
sketch Wheel(radius: Length) {
    init( width:Length ) { radius = width / 2; }
    
    radius = 1; // error: code between initializers not allowed

    init( height:Length ) { radius = height / 2; }
}

Wheel(radius = 1.0mm);
```

### Building Code

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

### Building Code Rules

It's **not allowed** to use the `sketch`, `part`, `op`, `return` nor `mod` statements within workbench code:

[![test](.test/illegal_workbench_statement.svg)](.test/illegal_workbench_statement.log)

```µcad,illegal_workbench_statement#fail
sketch Wheel(radius: Length) {
    sketch Axis(length: Length) {}  // error
    std::geo2d::Circle(radius);
}

Wheel(radius = 1.0mm);
```

## Properties

There are two ways to declare *Properties*:

- as parameter of the building plan or
- in an assignment within the build code by using the keyword `prop`.

In the following example we declare a building plan which consists of a `radius` which will automatically be a property:

[![test](.test/property.svg)](.test/property.log)

```µcad,property
// `outer` will automatically become a property because
// it is declared in the building plan:
sketch Wheel(outer: Length) {
    use std::geo2d::Circle;

    // `inner` is declared as property and maybe read from 
    // outside this workbench
    prop inner = outer / 2;

    // generate wheel (and use property inner)
    Circle(r = outer) - Circle(r = inner);
}

// evaluate wheel
t = Wheel(1cm);

// extract and display `outer` and `inner` from generated wheel
std::print("outer: {t.outer}");
std::print("inner: {t.inner}");
```

If you remove the `prop` keyword you will fail at accessing `inner`:

[![test](.test/property_wrong.svg)](.test/property_wrong.log)

```µcad,property_wrong#fail
sketch Wheel(outer: Length) {
    use std::geo2d::Circle;

    // `inner` is declared as variable and may not be read
    // from outside this workbench
    inner = outer / 2;

    Circle(r = outer) - Circle(r = inner);
}

t = Wheel(1cm);

// you can still extract and display `outer`
std::print("outer: {t.outer}");
// but you cannot access `inner` anymore
std::print("inner: {t.inner}"); // error
```
