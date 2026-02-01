# Align

Align a geometry along an axis to a certain spacing.

For example, you can align a group of geometries along a line in 2D:

[![test](.test/align_2d.svg)](.test/align_2d.log)

```µcad,align_2d
use std::geo2d::*;
use std::ops::*;
use std::math::*;

{
    Circle(radius = 0mm);
    Rect(size = 10mm);
    Circle(radius = 10mm);
    Rect(size = 10mm);
}.align(X, 4mm);
```

Align a multiplicity of geometries along a line in 2D:

[![test](.test/align_2d_multiplicity.svg)](.test/align_2d_multiplicity.log)

```µcad,align_2d_multiplicity
use std::geo2d::*;
use std::ops::*;
use std::math::*;

Rect(size = 10mm).rotate([0..10] * 90° / 10).align(X, 4mm);
```

You can also align 3D geometries:

[![test](.test/align_3d.svg)](.test/align_3d.log)

```µcad,align_3d
use std::geo3d::*;
use std::ops::*;
use std::math::*;

{
    Sphere(radius = 10mm);
    Cube(size = 10mm);
    Sphere(radius = 10mm);
    Cube(size = 10mm);
}.align(Z, 4mm);
```
