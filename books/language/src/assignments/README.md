# Assignments

Whenever you use a more complex [*expression*](../expressions/), it is often
useful to store it behind a name so that it can be used once or multiple times
elsewhere.
In µcad, stored **values are always immutable**, meaning that once a value has
been stored behind a name, it cannot be reset in the same context.
This is different from the variables known in other programming languages.

[![test](.test/assignment.svg)](.test/assignment.log)

```µcad,assignment
use std::math::sqrt;

a = 2cm;
b = 5cm; 
c = sqrt(a*a + b*b);

std::print("{c}");
```

>[!TIP]
> If a name starts with an underscore (like `_this_name`) that suppresses
> any warning about if it is not in use.

There are several kinds of assignments for different purposes.
A property for example is always related to a *workbench* so it can only be
defined within one.
Here is a complete list of all assignments available in µcad and where they can
be defined:

| Target           | Key-word | [Source File](../structure/source_file.md) | [Module](../structure/modules/) | [Building Code](../structure/workbenches/elements/building_code.md) | [Func-tion](../structure/functions/) | [Initial-ization](../structure/workbenches/elements/init_code.md) | [Initial-izers](../structure/workbenches/elements/initializers.md) |
| ---------------- | :------: | :----------------------------------------: | :-----------------------------: | :-----------------------------------------------------------------: | :----------------------------------: | :---------------------------------------------------------------: | :----------------------------------------------------------------: |
| Value            |    -     |                     ✅                      |                ❌                |                                  ✅                                  |                  ✅                   |                                 ❌                                 |                                 ✅                                  |
| Model            |    -     |                     ✅                      |                ❌                |                                  ✅                                  |                  ❌                   |                                 ❌                                 |                                 ❌                                  |
| Private Constant | `const`  |                     ✅                      |                ✅                |                                  ❌                                  |                  ❌                   |                                 ✅                                 |                                 ❌                                  |
| Public Constant  |  `pub`   |                     ✅                      |                ✅                |                                  ❌                                  |                  ❌                   |                                 ❌                                 |                                 ❌                                  |
| Property         |  `prop`  |                     ❌                      |                ❌                |                                  ✅                                  |                  ❌                   |                                 ❌                                 |                                 ❌                                  |
