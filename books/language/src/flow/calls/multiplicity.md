# Multiplicity

When working with multiplicities, each argument can be provided as an *array* of elements of a parameter's type.
Each list element will then be evaluated for each of the array's values.
This is known as *argument multiplicity*. This way, we can intuitively express a call that is executed for each argument variant.

The following example will produce 4 rectangles at different positions:

[![test](.test/multiplicity_arrays.svg)](.test/multiplicity_arrays.log)

```µcad,multiplicity_arrays
r = std::geo2d::Rect(width = 2mm, height = 2mm);

r.std::ops::translate(x = [-4mm, 4mm], y = [-4mm, 4mm]);
```

The example results in the following calls:

[![test](.test/no_multiplicity.svg)](.test/no_multiplicity.log)

```µcad,no_multiplicity
r = std::geo2d::Rect(width = 2mm, height = 2mm);

use std::ops::translate;
r.translate(x = -4mm, y = -4mm);
r.translate(x = -4mm, y = 4mm);
r.translate(x = 4mm, y = -4mm);
r.translate(x = 4mm, y = 4mm);
```

Normally, this would require 2 nested *for loops*, which are not available in µcad.

Another example uses an array of tuples and produces the same output:

[![test](.test/multiplicity_tuple_array.svg)](.test/multiplicity_tuple_array.log)

```µcad,multiplicity_tuple_array#todo
r = std::geo2d::Rect(width = 2mm, height = 2mm);

r.std::ops::translate([(x=-4mm, y=-4mm), (x=-4mm, y=4mm), (x=4mm, y=-4mm), (x=4mm, y=4mm)]);
```

## Inline Identifiers

Argument names can be skipped if the parameter expression is a single identifier.
Like in the following example, where the variables `width` and `height` have the
exact same name as the parameters of `Circle()`.

[![test](.test/inline_identifiers.svg)](.test/inline_identifiers.log)

```µcad,inline_identifiers
width = 2mm;
height = 2mm;
std::geo2d::Rect(width, height);
```
