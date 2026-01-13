# Parts

Parts are *workbenches* that are used to create 3D *models*.

[![test](.test/part_basic.svg)](.test/part_basic.log)

```µcad,part_basic
part MyPart( radius: Length ) {
    use std::geo3d::*;
    Sphere(radius) - Cube(radius)
}

MyPart(1cm);
```

Like all workbenches a part can have several [workbench elements](../elements/).
