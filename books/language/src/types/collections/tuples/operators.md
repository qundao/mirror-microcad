# Tuple operators

## Addition `*`

### Adding two tuples of the same type

[![test](.test/tuple_add_same.svg)](.test/tuple_add_same.log)

```µcad,tuple_add_same
a = (x = 1.0mm, y = 2.0mm);
b = (x = 3.0mm, y = 4.0mm);
std::debug::assert_eq([a + b, (x = 4.0mm, y = 6.0mm)]);
```

[![test](.test/tuple_add_different.svg)](.test/tuple_add_different.log)

```µcad,tuple_add_different#fail
a = (x = 1.0mm, y = 2.0mm);
b = (x = 3.0mm, z = 4.0mm);
c = a + b; // error: Tuple type mismatch for +: lhs=(x: Length, y: Length), rhs=(x: Length, z: Length)
std::debug::assert_eq([c, (x = 4.0mm, y = 6.0mm)]); // error: Array elements have different types: [<INVALID TYPE>, (x: Length, y: Length)]
```

### Subtraction `-`

#### Subtracting a quantity

[![test](.test/tuple_sub.svg)](.test/tuple_sub.log)

```µcad,tuple_sub
a = (x = 2.0mm, y = 3.0mm);
b = (x = 1.0mm, y = 4.0mm);
std::debug::assert_eq([a - b, (x = 1.0mm, y = -1.0mm)]);
```

### Multiplication `*`

#### Scaling a tuple

[![test](.test/tuple_mul_scale.svg)](.test/tuple_mul_scale.log)

```µcad,tuple_mul_scale
v = (x = 1.0mm, y = 2.0mm);
std::debug::assert_eq([v*2, (x = 2.0mm, y = 4.0mm)]);
```

### Division `/`

#### Dividing a tuple by a value

[![test](.test/tuple_div.svg)](.test/tuple_div.log)

```µcad,tuple_div
v = (x = 1.0mm, y = 2.0mm);
std::debug::assert_eq([v/2, (x = 0.5mm, y = 1.0mm)]);
```

### Negation `-`

[![test](.test/tuple_neg.svg)](.test/tuple_neg.log)

```µcad,tuple_neg
v = (x = 1.0mm, y = 2.0mm);
std::debug::assert_eq([-v, (x = -1.0mm, y = -2.0mm)]);
```
