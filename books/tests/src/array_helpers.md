# Array Helpers

There are several helpers in the standard library which might worth mentioning.

## Count

You can count the number of elements in an array using `std::count()`.

[![test](.test/array_helper_count.svg)](.test/array_helper_count.log)

```µcad,array_helper_count
std::debug::assert_eq([ std::count([1,2,3]), 3 ]);
```

## Head

You can get the head of an array using `std::head()`.

[![test](.test/array_helper_head.svg)](.test/array_helper_head.log)

```µcad,array_helper_head
std::debug::assert_eq([ std::head([1,2,3]), 1 ]);
```

## Tail

You can get the tail of an array using `std::head` and `std::tail()`.

[![test](.test/array_helper_tail.svg)](.test/array_helper_tail.log)

```µcad,array_helper_tail
std::debug::assert_eq([ std::tail([1,2,3]), [2,3] ]);
```

## Reverse

You can reverse the order the elements in an array using `std::rev()`.

[![test](.test/array_helper_reverse.svg)](.test/array_helper_reverse.log)

```µcad,array_helper_reverse#todo
std::debug::assert_eq([ std::count([1,2,3]), [3,2,1] ]);
```

## Sort

You can sort the elements in an array using `std::sort()`.

[![test](.test/array_helper_sort.svg)](.test/array_helper_sort.log)

```µcad,array_helper_sort#todo
std::debug::assert_eq([ std::sore([3,1,2]), [1,2,3] ]);
```
