# Value Assignments

A *value assignment* stores a value by a *name* on the *stack*.
They can be placed in
[source files](../structure/source_file.md),
[building code](../structure/workbenches/elements/building_code.md),
[module functions](../structure/functions/module_functions.md) or
[workbench functions](../structure/functions/module_functions.md).

The following example defines the variable `a` which from then is a reserved
name within the scope in which it was defined (*locality*).
In this case the source code file itself:

[![test](.test/assignment_value.svg)](.test/assignment_value.log)

```µcad,assignment_value
// source code file is the scope of a and b

use std::debug::*;

a = 5;
b = a * 2;
assert_eq([ a, 5  ]);
assert_eq([ b, 10 ]);
```

## Locality

If you place a value assignment into a scope (using brackets `{}`), the defined
value is only available within that specific scope:

[![test](.test/assignment_value_scope.svg)](.test/assignment_value_scope.log)

```µcad,assignment_value_scope#todo
// source code file is the topmost scope #0

use std::debug::*;

// define a within scope #0
a = 5;
assert_eq([ a, 5 ]);

// scope #1
{
    // define b within scope #1
    b = a * 2;
    // of course b is available at this point
    assert_valid( b );

    // scope #2
    {
        // b is available in scope #2 because #2 is within #1
        assert_valid( b );
    }
}

// a is still available
assert_valid(a);
// b not known here anymore
assert_invalid(b);
```

## Restrictions

### No Shadowing

So-called "shadowing" (reusing a *name*) is prohibited.
This restriction is highly intentional because µcad follows a concept of strict
immutability[^immutable] of all value definitions.

[^immutable]: Reusing names would undercut the ability to connect identifiers to values (e.g. when displaying).

[![test](.test/assignment_shadow_scope.svg)](.test/assignment_shadow_scope.log)

```µcad,assignment_shadow_scope#todo
a = 5;

{
    a = a * 2;   // this works because we are in a new scope
    std::debug::assert_eq([ a, 10 ]);
}
```

Another assignment of a variable with the same name without an additional scope is
prohibited.

[![test](.test/assignment_shadow.svg)](.test/assignment_shadow.log)

```µcad,assignment_shadow#fail
a = 5;      // warning: unused local
a = a * 2;  // error: a already defined in this scope
```

### Not in modules

Value assignments are not available in modules:

[![test](.test/assignment_module.svg)](.test/assignment_module.log)

```µcad,assignment_module#fail
mod my_module {
    a = 1; // error
}
```

### Not in initialization code

Value assignments are not available in workbenches' initialization code:

[![test](.test/assignment_workbench.svg)](.test/assignment_workbench.log)

```µcad,assignment_workbench#fail
sketch MySketch() {
    a = 1;   // error
    init(_x : Scalar) {}
}

MySketch();
```
