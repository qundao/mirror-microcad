# Align

Align a geometry along an axis to a certain spacing.

For example, you can align a group of geometries along a line in 2D:

[![test](.test/align_2d.svg)](.test/align_2d.log)

```µcad,align_2d
use std::geo2d::*;
use std::ops::*;
use std::math::*;

{
    Circle(10mm);
    Rect(10mm);
    Circle(10mm);
    Rect(10mm);
}.align(X, 4mm);
```

Align a multiplicity of geometries along a line in 2D:

[![test](.test/align_2d_multiplicity.svg)](.test/align_2d_multiplicity.log)

```µcad,align_2d_multiplicity
use std::geo2d::*;
use std::ops::*;
use std::math::*;

Rect(10mm).rotate([0..10] * 90° / 10).align(X, 4mm);
```

You can also align 3D geometries:

[![test](.test/align_3d.svg)](.test/align_3d.log)

```µcad,align_3d
use std::geo3d::*;
use std::ops::*;
use std::math::*;

{
    Sphere(10mm);
    Cube(10mm);
    Sphere(10mm);
    Cube(10mm);
}.align(Z, 4mm);
```
