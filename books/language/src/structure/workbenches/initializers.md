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

### Building plan cannot be accessed within initializers

You cannot read building plan items from within initializers.

[![test](.test/no_building_plan_in_initializers.svg)](.test/no_building_plan_in_initializers.log)

```µcad,no_building_plan_in_initializers#todo_fail
sketch Wheel(radius: Length) {
    init( width: Length ) { 
        _ = radius;         // error: cannot read radius here
        radius = width / 2; // instead you need to set it
    }
}

Wheel(width = 1.0mm);
```

### Initializer parameters with name from building plan

If you use parameter names in an initializer which already are used in the
building plan, they will automatically become properties and cannot be set
second time.

[![test](.test/no_building_plan_same_name.svg)](.test/no_building_plan_same_name.log)

```µcad,no_building_plan_same_name#fail
sketch Wheel(radius: Length, inner: Length) {
    init( radius: Length ) {
        // radius is seta property already by building plan

        radius = radius * 2;  // error: it cannot be set a second time
        inner = radius / 2;        
    }

    use std::geo2d::Circle;
    Circle(radius) - Circle(radius = inner)
}
// Use initializer
Wheel(radius = 1.0mm);
```

Types must match when using a name from building plan in initializer parameters. 

[![test](.test/no_building_plan_same_name_different_type.svg)](.test/no_building_plan_same_name_different_type.log)

```µcad,no_building_plan_same_name_different_type#todo_fail
sketch Wheel(radius: Length, inner: Length) {
    init( radius: Scalar ) {  // error: radius is already a `Length` in building plan
        inner = radius / 2 * 1mm;
    }

    use std::geo2d::Circle;
    Circle(radius * 1mm) - Circle(radius = inner)
}
// Use initializer
Wheel(radius = 1.0);
```
