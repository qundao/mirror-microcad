# Buffer

The buffer operation creates polygons which represent all points within a specified distance from the input geometry. For example, buffering a point creates a circle, buffering a line creates a “pill” shape,
and buffering a polygon creates a larger polygon (or a smaller one if a negative distance is requested).

[![test](.test/buffer.svg)](.test/buffer.log)

```µcad,std_geo2d_buffer
std::geo2d::Rect(20mm)
    .std::ops::buffer(distance = 20mm);
```
