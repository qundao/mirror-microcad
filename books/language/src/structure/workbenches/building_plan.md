# Building Plan

The *building plan* is defined by a *parameter list* that follows the workbench's
*identifier*.
All *parameters* in that list will automatically become [properties](properties.md) of the
workbench when it is invoked.
These properties can be accessed within the [*building code*](building_code.md), inside functions,
or externally.

The following code demonstrates this using a `sketch` with a single parameter
in the building plan, called `radius`, of type `Length`:

[![test](.test/building_plan.svg)](.test/building_plan.log)

```µcad,building_plan
// sketch with a radius as building plan
sketch Wheel(radius: Length) {
    // access property radius from the building plan
    std::geo2d::Circle(radius);
}

// access property radius of a Wheel
r = Wheel(5cm).radius;

// check if r is 5cm
std::debug::assert_eq( [r, 5cm] );
```
