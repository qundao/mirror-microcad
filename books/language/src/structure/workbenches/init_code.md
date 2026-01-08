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
    init(diameter: Length) { into_radius(diameter) }

    // function
    fn into_radius( diameter: Length ) {
        // use constant FACTOR from init code
        diameter / FACTOR
    }

    // set property diameter and use FACTOR from init code
    prop diameter = radius * FACTOR;
    
    // code body
    std::geo2d::Circle(radius);
}

std::debug::assert_eq([ Wheel(radius = 5cm).radius, 5cm ]);
std::debug::assert_eq([ Wheel(radius = 5cm).diameter, 10cm ]);
```

## Rules

### No code between initializers

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
