# Initializers

*Initializers* are a way to define alternative parameters to create the
building plan.
An Initializer is defined with the keyword `init` and a following *parameter list*.
One may define multiple initializers which must have different parameter lists.

However, if an initializer is used, all properties from the building plan must
be initialized (except those with *default values*).

[![test](.test/init_property.svg)](.test/init_property.log)

```µcad,init_property
sketch Wheel(radius: Length, thickness: Length) {
    // initializer with diameter
    init( diameter: Length, thickness: Length ) {
        // must set property `radius` from building plan
        radius = diameter / 2;

        // thickness (from the building plan) does not need 
        // to be set, because it was automatically set by 
        // parameter of this initializer
    }

    // Now radius and thickness can be used
    std::geo2d::Circle(radius) - std::geo2d::Circle(radius = radius - thickness);
}

// call with building plan
Wheel(radius=1.5cm, thickness=2mm);
// call with initializer
Wheel(diameter=1.5cm, thickness=5mm);
```

The output of this code shows the two concentric wheels:

![test](.test/init_property-out.svg)

## Rules

If the *building plan* is not fully initialized by an initializer
you will get an error:

[![test](.test/missed_property.svg)](.test/missed_property.log)

```µcad,missed_property#fail
sketch Wheel(radius: Length) {
    init( width: Length ) { _ = width; } // error: misses to set `radius` from building plan
}

Wheel(width = 1.0mm);
```
