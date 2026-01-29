# Parts

Parts are *workbenches* that are used to create 3D *models*.
They are named in `PascalCase`:

[![test](.test/part_basic.svg)](.test/part_basic.log)

```µcad,part_basic
part MyPart( radius: Length ) {
    use std::geo3d::*;
    Sphere(radius) - Cube(radius)
}

MyPart(1cm);
```

Like all workbenches parts can have several [workbench elements](../elements/).

## Restrictions

### Parts cannot generate 2D models

You will get an error if you generate a 2D model with a part:

[![test](.test/part_2d.svg)](.test/part_2d.log)

```µcad,part_2d#todo_fail
sketch MyPart( radius: Length) {
    use std::geo2d::*;
    Circle(radius) - Rect(radius);  // error: Circle and Rect are 2D
}

MyPart(1cm);
```
