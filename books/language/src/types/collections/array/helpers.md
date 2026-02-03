# Array Helpers

There are several helpers in the standard library which might worth mentioning.

## Count

You can count the number of elements in an array using `std::count`:

[![test](.test/array_expressions.svg)](.test/array_expressions.log)

```µcad,array_expressions
std::debug::assert_eq([ std::count([1,2,3]), 3 ]);
```

## Head & Tail

You can get the head and tail of an array using `std::head` and `std::tail`:

[![test](.test/array_expressions_head_tail.svg)](.test/array_expressions_head_tail.log)

```µcad,array_expressions_head_tail
std::debug::assert_eq([ std::head([1,2,3]), 1 ]);
std::debug::assert_eq([ std::tail([1,2,3]), [2,3] ]);
```
