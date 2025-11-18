# Assignments

Whenever you use a more complex *expression*, it is often worthwhile to store it in a variable so that it can
be used once or multiple times elsewhere.
In µcad, variables are always immutable which means that once they are set, their value cannot be reset in the same context.
Therefore, they differ from the variables known in other programming languages.

A *value assignment* stores a value by a name on the *evaluation stack*.

The following example defines the variable `a` which from then is a reserved
name within the scope in which it was defined.

[![test](.test/assignment.svg)](.test/assignment.log)

```µcad,assignment
a = 5;
b = a * 2;
std::debug::assert_eq([a,5]);
std::debug::assert_eq([b,10]);
```

Another assignment of a variable with the same name is not allowed.

[![test](.test/assignment_immutable.svg)](.test/assignment_immutable.log)

```µcad,assignment_immutable#fail
a = 5; // warning: unused local
a = a * 2; // error: a already defined in this scope
```

Value assignments may only be used within code bodies (scopes) - and not within 
modules or in init code for example:

[![test](.test/assignment_module.svg)](.test/assignment_module.log)

```µcad,assignment_module#fail
mod my_module {
    a = 1; // error
}

sketch MySketch() {
    a = 1;   // error
    init(_x : Scalar) {}
}

MySketch();
```

## Prefix Underscore

If a value name starts with an underscore (like `_this_name`) that suppresses any warning about if it is not in use.
