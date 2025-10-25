# Ambiguous Look Up for properties/locals and operations

```µcad,method_call
op scale() {}
sketch Sketch(scale: Scalar) { std::geo2d::Rect(size = scale * 40mm) }
Sketch(4.0);
```
