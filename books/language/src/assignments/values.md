# Value Assignments

A *value assignment* stores a value by a name on the *evaluation stack*.

The following example defines the variable `a` which from then is a reserved
name within the scope in which it was defined.

[![test](.test/assignment_value.svg)](.test/assignment_value.log)

```µcad,assignment_value
use std::debug::*;

a = 5;
b = a * 2;
assert_eq([ a, 5  ]);
assert_eq([ b, 10 ]);
```

## Rules

### Locality

If you place a value assignment in a scope, the defined value is only available
within that scope:

[![test](.test/assignment_value_scope.svg)](.test/assignment_value_scope.log)

```µcad,assignment_value_scope#todo
use std::debug::*;

a = 5;
assert_eq([ a, 5 ]);

{
    b = a * 2;
    assert_eq([ b, 10 ]);
}

assert_valid(a);   // a is still available
assert_invalid(b); // b not known here anymore
```

### Cannot reuse names in same scope

Another assignment of a variable with the same name is not allowed.

[![test](.test/assignment_immutable.svg)](.test/assignment_immutable.log)

```µcad,assignment_immutable#fail
a = 5;      // warning: unused local
a = a * 2;  // error: a already defined in this scope
```

### Not allowed in modules

Value assignments are not available in modules:

[![test](.test/assignment_module.svg)](.test/assignment_module.log)

```µcad,assignment_module#fail
mod my_module {
    a = 1; // error
}
```

### Not allowed in initialization code

Value assignments are not available in workbenches' initialization code:

[![test](.test/assignment_workbench.svg)](.test/assignment_workbench.log)

```µcad,assignment_workbench#fail
sketch MySketch() {
    a = 1;   // error
    init(_x : Scalar) {}
}

MySketch();
```
