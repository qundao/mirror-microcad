# Distribute in grid

Align a geometry along an axis to a certain spacing.

For example, you can align a group of geometries along a line in 2D:

[![test](.test/distribute_grid_2d.svg)](.test/distribute_grid_2d.log)

```µcad,distribute_grid_2d
use std::geo2d::*;
use std::ops::*;

{
    Circle(10mm);
    Rect(10mm);
    Rect(10mm);
    Circle(10mm);
}.distribute_grid(30mm, rows = 2, columns = 2);
```

[![test](.test/distribute_grid_3d.svg)](.test/distribute_grid_3d.log)

```µcad,distribute_grid_3d
use std::geo3d::*;
use std::ops::*;

{
    Sphere(10mm);
    Cube(10mm);
    Cube(10mm);
    Sphere(10mm);
}.distribute_grid(30mm, rows = 2, columns = 2);
```
