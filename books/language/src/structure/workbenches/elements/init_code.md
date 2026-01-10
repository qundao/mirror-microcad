# Initialization Code

If you use *initializers* you might place some *initialization code*  on top of
the workbench's body (before the first *initializer*).

The *initialization code* is just allowed to define some *constants* which
then can be used in all following code (including code within *initializers*
and *functions*).

[![test](.test/pre_init_code.svg)](.test/pre_init_code.log)

```µcad,pre_init_code
sketch Wheel(radius: Length) {
    // init code
    const FACTOR = 2.0;

    // initializer
    init(diameter: Length) { into_radius(radius); }

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

__builtin::debug::assert(Wheel(5cm).radius == 5cm);
__builtin::debug::assert(Wheel(5cm).diameter == 10cm);
```

If there are no initializers, the initialization code is just part of the
[building code](building_code.md).

## Rules

### Cannot access building plan in initialization code

[![test](.test/init_code_no_building_plan.svg)](.test/init_code_no_building_plan.log)

```µcad,init_code_no_building_plan#todo_fail
sketch Wheel(radius: Length, thickness = 5mm) {
    use std::geo2d::Circle;

    const _ = radius * 2;   // error: cannot use radius from building plan

    init( diameter: Length ) { radius = diameter / 2; }
    Circle(radius + thickness) - Circle(radius)
}

Wheel(radius = 1cm);
```
