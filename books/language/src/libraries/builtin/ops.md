# Builtin Library `ops`

## `ops`

### `subtract`

[![test](.test/builtin_subtract_2d.svg)](.test/builtin_subtract_2d.log)

```µcad,builtin_subtract_2d
use __builtin::*;

#[color = "red"]
{
    geo2d::Circle(radius = 3.0mm).ops::translate(x = 1.0mm, y = 1.0mm, z = 0.0mm);
    geo2d::Rect(x = 0.0mm, y = 0.0mm, width = 3.0mm, height = 3.0mm);
}.ops::subtract();
```

[![test](.test/builtin_subtract_3d.svg)](.test/builtin_subtract_3d.log)

```µcad,builtin_subtract_3d
use __builtin::*;

{
    geo3d::Sphere(radius = 3.0mm);
    geo3d::Sphere(radius = 2.0mm).__builtin::ops::translate(x = 3.0mm);    
}.ops::subtract();
```

### `union`

### `intersect`

### `hull`

### `extrude`

[![test](.test/builtin_extrude.svg)](.test/builtin_extrude.log)

```µcad,builtin_extrude
use __builtin::*;

a = geo2d::Circle(radius = 9.0mm) - geo2d::Circle(radius = 2.0mm).ops::translate(x = [-3.0, 3.0]mm, y = [-3.0, 3.0]mm);

a.ops::extrude(height = 4.0mm, scale_x = 100%, scale_y = 100%, twist = 0°);
```

### `orient`

### `revolve`

[![test](.test/builtin_revolve.svg)](.test/builtin_revolve.log)

```µcad,builtin_revolve
use __builtin::*;

std::geo2d::Circle(10mm)
    .ops::translate(x = 20.0mm, y = 0.0mm, z = 0.0mm)
    .ops::revolve(angle = 360.0°);
```

### `rotate`

### `translate`

use __builtin::*;

[![test](.test/builtin_translate.svg)](.test/builtin_translate.log)

```µcad,builtin_translate
op translate(x = 0.0mm, y = 0.0mm, z = 0.0mm) {
    @input.__builtin::ops::translate(x, y, z);
}

r = __builtin::geo2d::Rect(x = 0mm, y = 0mm, width = 3mm, height = 3mm);

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
geo2d::Rect(x = 0.0mm, y = 0.0mm, width = 10.0mm, height = 10.0mm)
    .ops::translate(x = 0.0mm, y = 15.0mm, z = 0.0mm)
    .ops::translate(x = 15.0mm, y = 0.0mm, z = 0.0mm);
```
