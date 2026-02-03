# Tuple operators

Tuples support the following operators.

| Operator | Description               | Example                   |
| -------- | ------------------------- | ------------------------- |
| `+`      | add each element          | `(x=1, y=2) + (x=3, y=4)` |
| `-`      | subtract each element     | `(x=1, y=2) - (x=3, y=4)` |
| `*`      | multiply each element     | `(x=1, y=2) * (x=3, y=4)` |
| `/`      | divide each element       | `(x=1, y=2) / (x=3, y=4)` |
| `-`      | negation of each element  | `-(x=1, y=2)`             |
| `!`      | inversion of each element | `!( true, false )`        |

[![test](.test/tuple_operations.svg)](.test/tuple_operations.log)

```µcad,tuple_operations
std::debug::assert_eq([ (x=1, y=2) + (x=3, y=4), (x=4, y=6) ]);
std::debug::assert_eq([ (x=2, y=3) - (x=1, y=4), (x=1, y=-1) ]);
std::debug::assert_eq([ (x=1.0, y=2.0) * 2, (x=2.0, y=4.0) ]);
std::debug::assert_eq([ (x = 1.0, y = 2.0) / 2, (x = 0.5, y = 1.0)]);
std::debug::assert_eq([ -(x = 1.0, y = 2.0), (x = -1.0, y = -2.0)]);
```

## Errors

### Name mismatch

[![test](.test/tuple_error_name_mismatch.svg)](.test/tuple_error_name_mismatch.log)

```µcad,tuple_error_name_mismatch#fail
(x=1, y=2) + (x=3, z=4); // error: type mismatch: lhs=(x: Integer, y: Integer), rhs=(x: Integer, z: Integer)
```

### Type mismatch

[![test](.test/tuple_error_type_mismatch.svg)](.test/tuple_error_type_mismatch.log)

```µcad,tuple_error_type_mismatch#fail
(x=1, y=2) + (x=3mm, y=4mm); // error: type mismatch: (x: Integer, y: Integer) + (x: Length, z: Length)
```
