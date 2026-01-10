# Sketches

Sketches are similar to *parts* but in two dimensions only.
They can be extruded into three-dimensional parts.

[![test](.test/sketch_basic.svg)](.test/sketch_basic.log)

```µcad,sketch_basic
use std::geo2d::*;
sketch MySketch( radius: Length) {
    Circle(radius) - Rect(size = radius);
}

MySketch(1cm);
```

The output is a 2D sketch:

![test](.test/sketch_basic-out.svg)

If you generate a 3D model within a sketch you will get an error:

[![test](.test/sketch_3d.svg)](.test/sketch_3d.log)

```µcad,sketch_3d#todo_fail
use std::geo3d::*;
sketch MySketch( radius: Length) {
    Sphere(radius) - Cube(size = radius);  // error
}

MySketch(1cm);
```
