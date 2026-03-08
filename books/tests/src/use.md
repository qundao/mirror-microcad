# Use statement tests

[![test](.test/use_test.svg)](.test/use_test.log)

```µcad,use_test
// use debug from `std/module.µcad`
use std::debug::assert;
use std::debug::is_valid;
assert(true);

// use all symbols from std::debug for test checks
use std::debug::*;

// use symbol `circle` in file `geo2d.µcad`
use std::geo2d::Circle;
assert(is_valid("Circle"));

// use all symbols in file `geo3d.µcad`
use std::geo3d::*;
assert(is_valid("Sphere"));
assert(is_valid("Cube"));

// alias `Circle` in `std/geo2d/mod.µcad` into `foo`
use std::geo2d::Circle as foo;
assert(is_valid("foo"));

// use print from `std/module.µcad`
use std::print;
assert(is_valid("print"));
print("Hello");

// public use operation from `std/module.µcad`
pub use std::ops;
assert(is_valid("ops"));
assert(is_valid("use_test::ops"));

part MyPart() {
    Circle(radius=1);
    Sphere(radius=1);
}

assert(is_valid("MyPart"));
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

part MyPart3d() { Sphere(radius=1mm); }
sketch MySketch2d() { Circle(radius=1mm); }

x = MySketch2d();
y = MyPart3d();
z = baz(-1.0);
```

[![test](.test/use_local.svg)](.test/use_local.log)

```µcad,use_local
fn f() {
    use std::math::abs;
    x = abs(-1.0);
    return x;
}
f();
```

[![test](.test/use_all_local.svg)](.test/use_all_local.log)

```µcad,use_all_local
fn f() {
    use std::math::*;
    x = abs(-1.0);
    return x;
}
f();
```

[![test](.test/use_statement_pub_in_module.svg)](.test/use_statement_pub_in_module.log)

```µcad,use_statement_pub_in_module
mod my {
    mod name {
        pub mod space {
            pub use std::geo2d::*;
        }
    }
    pub use name::space::*;
}

my::Circle(radius = 4mm);
my::Rect(size = 40mm);
```
