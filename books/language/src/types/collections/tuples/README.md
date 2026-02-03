# Tuples

A *tuple* is a collection of *values*, each of which can be of different *type*.
Typically, each value is paired with an identifier, allowing a tuple to function
similarly to a key-value store.

[![test](.test/named_tuple_access.svg)](.test/named_tuple_access.log)

```µcad,named_tuple_access
use std::debug::assert_eq;

tuple = (width=10cm, depth=10cm, volume=1l);

assert_eq([ tuple.width, 10cm ]);
assert_eq([ tuple.depth, 10cm ]);
assert_eq([ tuple.volume, 1l ]);

assert_eq([ tuple, (width=10cm, depth=10cm, volume=1l) ]);
```

## Partially Named Tuples

A tuple may lack identifiers for some or even all of its values.
In such cases, these *unnamed values* within the tuple must all be of different types.

[![test](.test/unnamed_tuple.svg)](.test/unnamed_tuple.log)

```µcad,unnamed_tuple
(10cm, 10cm², 1l);
```

Otherwise, they would be indistinguishable since the values in a tuple do not adhere
to a specific order.

## Arbitrary Order

[![test](.test/unnamed_tuple_order.svg)](.test/unnamed_tuple_order.log)

```µcad,unnamed_tuple_order
// these tuples are equal
std::debug::assert_eq([ (1l, 10cm, 10cm²), (10cm, 10cm², 1l) ]);
```

A partially or fully unnamed tuple can only be utilized through
[argument matching](../structure/arguments.md#argument-matching) or *tuple assignment*.

## Ambiguous Elements

Either names or types must be unique in a tuple.

[![test](.test/unnamed_tuple_ambiguous.svg)](.test/unnamed_tuple_ambiguous.log)

```µcad,unnamed_tuple_ambiguous#todo_fail
(10cm, 10mm, 1m);  // error: ambiguous type Length
```
