# Builtin Library `ops`

## `ops`

### `subtract`

[![test](.test/builtin_subtract_2d.svg)](.test/builtin_subtract_2d.log)

```µcad,builtin_subtract_2d
use __builtin::*;

#[color = "red"]
{
    geo2d::Circle(radius = 3.0).ops::translate(x = 1.0, y = 1.0, z = 0.0);
    geo2d::Rect(x = 0.0, y = 0.0, width = 3.0, height = 3.0);
}.ops::subtract();
```

[![test](.test/builtin_subtract_3d.svg)](.test/builtin_subtract_3d.log)

```µcad,builtin_subtract_3d
use __builtin::*;

{
    geo3d::Sphere(radius = 3.0);
    geo3d::Sphere(radius = 2.0).__builtin::ops::translate(x = 3.0);    
}.ops::subtract();
```

### `union`

### `intersect`

### `hull`

### `extrude`

[![test](.test/builtin_extrude.svg)](.test/builtin_extrude.log)

```µcad,builtin_extrude
use __builtin::*;

a = geo2d::Circle(radius = 9.0) - geo2d::Circle(radius = 2.0).ops::translate(x = [-3.0, 3.0], y = [-3.0, 3.0], z = 0.0);

a.ops::extrude(height = 4.0, scale_x = 100%, scale_y = 100%, twist = 0°);
```

### `orient`

### `revolve`

[![test](.test/builtin_revolve.svg)](.test/builtin_revolve.log)

```µcad,builtin_revolve
use __builtin::*;

std::geo2d::Circle(10mm)
    .ops::translate(x = 20.0, y = 0.0, z = 0.0)
    .ops::revolve(revolve_degrees = 360.0);
```

### `rotate`

### `translate`

use __builtin::*;

[![test](.test/builtin_translate.svg)](.test/builtin_translate.log)

```µcad,builtin_translate
op translate(x = 0.0mm, y = 0.0mm, z = 0.0mm) {
    @input.__builtin::ops::translate(x = x / 1mm, y = y / 1mm, z = z / 1mm);
}

r = __builtin::geo2d::Rect(x = 0, y = 0, width = 3, height = 3);

#[color = "red"]
r.translate(x = 0.0mm, y = 0.0mm);

#[color = "blue"]
r.translate(x = 4.0mm, y = 0.0mm);

#[color = "green"]
r.translate(x = 0.0mm, y = 4.0mm);

#[color = "gray"]
r.translate(x = 4.0mm, y = 4.0mm);
```

[![test](.test/builtin_translate_twice.svg)](.test/builtin_translate_twice.log)

```µcad,builtin_translate_twice
use __builtin::*;

#[color = color::RED]
geo2d::Rect(x = 0.0, y = 0.0, width = 10.0, height = 10.0)
    .ops::translate(x = 0.0, y = 15.0, z = 0.0)
    .ops::translate(x = 15.0, y = 0.0, z = 0.0);
```
