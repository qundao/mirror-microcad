# Arithmetic

Quantity types can use operators:

[![test](.test/types_quantity_arithmetic.svg)](.test/types_quantity_arithmetic.log)

```µcad,types_quantity_arithmetic#todo
use std::debug::assert_eq;

a = 6cm;
b = 2cm;
assert_eq([ a * b, 12cm²  ]);
assert_eq([ a / b, 3 ]);
assert_eq([ a + b, 80mm ]);
assert_eq([ a - b, 40mm ]);
```
