# Tuple Assignments

Tuple syntax is also employed on the left side of *tuple assignments*.

[![test](.test/tuple_assignment.svg)](.test/tuple_assignment.log)

```µcad,tuple_assignment#todo
(width, height) = (1m,2m);
// check values of width and height
assert_eq([width,1m]);
assert_eq([height,2m]);
```

Occasionally, it's practical to *group units* together, but this cannot be done directly
with a tuple.
Instead, you must use an *array*, which will be converted into a tuple during the assignment.

[![test](.test/tuple_assignment_bundle.svg)](.test/tuple_assignment_bundle.log)

```µcad,tuple_assignment_bundle#todo
(width, height) = [1,2]m;
assert_eq([width,1m]);
assert_eq([height,2m]);
```

This method can generally be used to convert an *array* into a *tuple*:

[![test](.test/tuple_assignment_convert.svg)](.test/tuple_assignment_convert.log)

```µcad,tuple_assignment_convert#todo
array = [1,2]m;
(width, height) = array;
tuple = (width, height);

assert_eq([tuple,(1m,2m)]);
assert_eq([tuple.width,1m]);
assert_eq([tuple.height,2m]);
```
