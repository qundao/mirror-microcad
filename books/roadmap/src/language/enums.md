# Enums

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

enum Axis {
    X,
    Y,
    Z,
}
```

Those may then be used in a match statement:

```µcad
op align(axis: Axis, align: Alignment ) {
    match align {
    FIX => ...,
    NEAR => ...,
    CENTER => ...,
    FAR => ...,
    }
}

s.align(X, CENTER).align(Y, FAR);
```
