# Use statement tests

[![test](.test/use_test.svg)](.test/use_test.log)

```µcad,use_test
// use debug from `std/module.µcad`
use std::debug::assert;
assert(true);

// use all symbols from std::debug for test checks
use std::debug::*;

// use symbol `circle` in file `geo2d.µcad`
use std::geo2d::Circle;
assert_valid(Circle);

// use all symbols in file `geo3d.µcad`
use std::geo3d::*;
assert_valid(Sphere);
assert_valid(Cube);

// alias `Circle` in `std/geo2d/mod.µcad` into `foo`
use std::geo2d::Circle as foo;
assert_valid(foo);

// use print from `std/module.µcad`
use std::print;
assert_valid(print);
print("Hello");

// public use operation from `std/module.µcad`
pub use std::ops;
assert_valid(ops);
assert_valid(use_test::ops);

part MyPart() {
    Circle(radius=1);
    Sphere(radius=1);
}

assert_valid(MyPart);
```

[![test](.test/use_as_test.svg)](.test/use_as_test.log)

```µcad,use_as_test
use std::geo2d::*; 
use Rect as Cap;

Cap(width=1mm,height=1mm);
```

[![test](.test/use_statement_test.svg)](.test/use_statement_test.log)

```µcad,use_statement_test
// use symbol `circle` in file `geo2d.µcad`
use std::geo2d::Circle;
// use all symbols in file `geo3d.µcad`
use std::geo3d::*;
// alias `bar` in `std/text/foo.µcad` into `baz`
use std::math::abs as baz;
// use print from `std/module.µcad`
use std::print;
// public use operation from `std/module.µcad`
pub use std::ops;

// use debug from `std/module.µcad`
use std::debug;
debug::assert(true);

part my_part3d() { Sphere(radius=1mm); }
part my_part2d() { Circle(radius=1mm); }

x = my_part2d();
y = my_part3d();
```

```µcad,use_local
fn f() {
    use std::math::abs;
    x = abs(-1.0);
    return x;
}
f();
```

```µcad,use_all_local
fn f() {
    use std::math::*;
    x = abs(-1.0);
    return x;
}
f();
```
