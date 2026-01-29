# Enums & Match

E.g. simple enums can be useful when functions or workbenches have different modes:

```µcad
enum Alignment {
    /// Do not align
    FIX,
    /// Align at left, top or front 
    NEAR,
    /// Align at center
    CENTER
    /// ALign at right, bottom or back
    FAR,
}
```

Those may then be used in `if` statements:

```µcad
// get X, Y, Z
use std::math::*

op align(axis: Vec3, align: Alignment ) {
    if align == Alignment::FIX {
        ...
    } else if align == Alignment::NEAR {
        ...
    } ...
}

s.align(X, CENTER).align(Y, FAR);
```

## Consequences

The enum type will be the first user defined type.
This must be integrated in our static type system.
