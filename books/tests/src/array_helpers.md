# Array Helpers

There are several helpers in the standard library which might worth mentioning.

## `len`

You can count the number of elements in an array using `std::array::len()`.

[![test](.test/array_helper_len.svg)](.test/array_helper_len.log)

```µcad,array_helper_len
std::debug::assert_eq([ std::array::len([1,2,3]), 3 ]);
```

## `first`

You can get the first element of an array using `std::array::first()`.

[![test](.test/array_helper_first.svg)](.test/array_helper_first.log)

```µcad,array_helper_first
std::debug::assert_eq([ std::array::first([1,2,3]), 1 ]);
```

## `tail`

You can get the tail of an array using `std::array::first()` and `std::array::tail()`.

[![test](.test/array_helper_tail.svg)](.test/array_helper_tail.log)

```µcad,array_helper_tail
std::debug::assert_eq([ std::array::tail([1,2,3]), [2,3] ]);
```

## `rev`

You can reverse the order the elements in an array using `std::array::rev()`.

[![test](.test/array_helper_rev.svg)](.test/array_helper_rev.log)

```µcad,array_helper_rev
std::debug::assert_eq([ std::array::rev([1,2,3]), [3,2,1] ]);
```

## `sorted`

You can sort the elements in an array using `std::array::sorted()`.

[![test](.test/array_helper_sorted.svg)](.test/array_helper_sorted.log)

```µcad,array_helper_sorted
std::debug::assert_eq([ std::array::sorted([3,1,2]), [1,2,3] ]);
```

## `contains`

You can sort the elements in an array using `std::array::contains(x)`.

[![test](.test/array_helper_contains.svg)](.test/array_helper_contains.log)

```µcad,array_helper_contains
std::debug::assert_eq([ std::array::contains(arr = [3,1,2], x = 1), true ]);
std::debug::assert_eq([ std::array::contains(arr = [3,1,2], x = 4), false ]);
```
