# Assignments

Whenever you use a more complex [*expression*](../expressions/README.md), it is often worthwhile to store it in a variable so that it can
be used once or multiple times elsewhere.
In µcad, **variables are always immutable** which means that once they are set, their value cannot be reset in the same context.
Therefore, they differ from the variables known in other programming languages.

[![test](.test/assignment.svg)](.test/assignment.log)

```µcad,assignment
use std::math::sqrt;

a = 2cm;
b = 5cm; 
c = sqrt(a*a + b*b);

std::print("{c}");
```

There are several kinds of assignments for different purposes.
A property for example is always related to a *workbench* so it can only be
defined within one.
Here is a complete list of all assignments available in µcad and where they can
be defined:

| Target           | Keyword | Source File | Module | Building | Function | Initialization & Initializers |
| ---------------- | ------- | :---------: | :----: | :------: | :------: | :---------------------------: |
| Value            | -       |      ✅      |   ❌    |    ✅     |    ✅     |               ❌               |
| Model            | -       |      ✅      |   ❌    |    ✅     |    ❌     |               ❌               |
| Private Constant | `const` |      ✅      |   ✅    |    ❌     |    ❌     |               ✅               |
| Public Constant  | `pub`   |      ✅      |   ✅    |    ❌     |    ❌     |               ❌               |
| Property         | `prop`  |      ❌      |   ❌    |    ✅     |    ❌     |               ❌               |

