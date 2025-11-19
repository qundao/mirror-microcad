# Arrays

An array is an ordered collection of values with a common type.

## Arrays as list: `[1, 2, 3]`

[![test](.test/arrays_and_comments.svg)](.test/arrays_and_comments.log)

```µcad,arrays_and_comments
[
    // First element
    1,

    // Second element
    2
];
```

You can count the number of elements in an array using `std::count`:

[![test](.test/array_expressions.svg)](.test/array_expressions.log)

```µcad,array_expressions
std::debug::assert_eq([std::count([1,2,3]), 3]);
```

## Arrays as range: `[1..5]`

You can generate an array via range expressions: `[1..5]`.

A µcad range includes both of it's end points.

[![test](.test/range_expressions.svg)](.test/range_expressions.log)

```µcad,range_expressions
std::debug::assert_eq([[1..5], [1,2,3,4,5]]);
std::debug::assert_eq([[-2..2], [-2,-1,0,1,2]]);
```

The order of the endpoints of a range is important:

[![test](.test/range_expressions_bad_order.svg)](.test/range_expressions_bad_order.log)

```µcad,range_expressions_bad_order#fail
[6..1];  // error
[2..-2];  // error
```

Only `Integer` can be used as endpoint:

[![test](.test/range_expressions_bad_type.svg)](.test/range_expressions_bad_type.log)

```µcad,range_expressions_bad_type#fail
[1.0..2.0];  // parse_error
```

## Array operations

### Unit bundling

Array support unit bundling, which means the you can write the unit after the `[]` brackets.

[![test](.test/array_unit_bundling.svg)](.test/array_unit_bundling.log)

```µcad,array_unit_bundling
std::debug::assert_eq([ [1mm, 2mm, 3mm], [1, 2, 3]mm ]);
```

### Addition `+`

#### Adding a quantity

[![test](.test/array_add.svg)](.test/array_add.log)

```µcad,array_add
std::debug::assert_eq([ [1,2]mm + 2mm, [3,4]mm ]);
```

### Subtraction `-`

#### Subtracting a quantity

[![test](.test/array_sub.svg)](.test/array_sub.log)

```µcad,array_sub
std::debug::assert_eq([ [1,2]mm - 2mm, [-1,0]mm ]);
```

### Multiplication `*`

#### Scaling an array

[![test](.test/array_mul_scale.svg)](.test/array_mul_scale.log)

```µcad,array_mul_scale
std::debug::assert_eq([ [-0.5mm,0.5mm]*2, [-1,1]mm ]);
```

### Division `/`

#### Dividing an array by a value

[![test](.test/array_div.svg)](.test/array_div.log)

```µcad,array_div
std::debug::assert_eq([ [-1.0mm,1.0mm]/2, [-0.5, 0.5]mm ]);
```

### Negation `-`

[![test](.test/array_neg.svg)](.test/array_neg.log)

```µcad,array_neg
std::debug::assert_eq([ [-1.0mm,1.0mm], -[1.0mm, -1.0mm] ]);
```
