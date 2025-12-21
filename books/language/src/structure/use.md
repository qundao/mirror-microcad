# Use Statements

When including code from other [*modules*](modules/internal_modules.md) or
other [*source files*](modules/external_modules.md) the use of
*fully qualified names* (e.g. `std:geo3d::Sphere` or `std:geo3d::Cube`) may
produce much boiler plate code when using them often.
The powerful `use` statement solves this problem and gives you an elegant method
to import code from other sources.

The following example which uses two *parts* of `std::geo3d` shows the problem
with long names:

[![test](.test/none.svg)](.test/none.log)

```µcad,none
std::geo3d::Sphere(radius = 40mm);
std::geo3d::Cube(size = 40mm);
```

There are several ways to solve this with `use`:

## Use Statement

Looking at the example below using `use` does not seem any shorter, but if we would use `Sphere` and `Cube`
more often this would shorten things quite a lot:

[![test](.test/use.svg)](.test/use.log)

```µcad,use
use std::geo3d::Sphere;
use std::geo3d::Cube;

Sphere(radius = 4mm);
Cube(size = 40mm);
```

Alternatively you can use the whole *module* `geo3d` at once and would get rid
of the `std::` part within the names:

[![test](.test/use_module.svg)](.test/use_module.log)

```µcad,use_module
use std::geo3d;

geo3d::Sphere(radius = 40mm);
```

## Use As Statement

Internally every *use statement* defines an *aliases* with an *identifier*
(e.g. `Sphere`) and a *target symbol* it points to (e.g. `std::geo3d::Sphere`).

If name conflicts occur a way to deal with this is to explicitly name the
*identifier* with `use as`:

[![test](.test/use_as.svg)](.test/use_as.log)

```µcad,use_as
use std::geo3d::Sphere as Ball;

Ball(radius = 4mm);
```

Of course you can use `use as` with a whole *module*:

[![test](.test/use_as_module.svg)](.test/use_as_module.log)

```µcad,use_as_module
use std::geo3d as space;

space::Sphere(radius = 4mm);
```

## Use All Statement

The shortest way to use many symbols from one module is to use `*` as *target*.
The following example defines aliases for **all** symbols of `std::geo3d`.

[![test](.test/use_all.svg)](.test/use_all.log)

```µcad,use_all
use std::geo3d::*;

Sphere(radius = 4mm);
Cube(size = 40mm);
Torus(major_radius = 40mm, minor_radius = 20mm);
```

The disadvantage of using `*` is that it increases the risk of name conflicts
between your code and the aliased symbols, some of which you might not even use.

## Public Use Statement

The `pub use` statement does not only make the *target symbol* visible within
the module in which it is defined, but from the outside too.

[![test](.test/use_statement_pub.svg)](.test/use_statement_pub.log)

```µcad,use_statement_pub
mod my {
    pub use std::geo3d::Sphere;
    pub use std::geo3d::Cube;
}

my::Sphere(radius = 4mm);
my::Cube(size = 40mm);
```
