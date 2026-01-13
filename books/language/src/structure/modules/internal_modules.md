# Internal Modules

Internal modules are modules which have been defined with the `mod` keyword
followed by a name and a body as the following example shows:

[![test](.test/internal_mod.svg)](.test/internal_mod.log)

```µcad,internal_mod
mod my_module {
    // e.g. define a value
    pub VALUE = 1;
}
```

Module names are always written in
[`lower_snake_case`](./../../appendix/coding.md).

The following statements can be listed in a module:

<!-- markdownlint-disable no-inline-html -->
| Statement                                      | Purpose                                     | Example        |
| ---------------------------------------------- | ------------------------------------------- | -------------- |
| [const assignment](../../assignments/const.md) | naming constants                            | `const y = 1;` |
| [pub assignment](../../assignments/pub.md)     | exporting constants                         | `pub y = 1;`   |
| [function](../functions.md)                    | separate calculations                       | `fn f() { }`   |
| [workbench](../workbenches)                    | build or transform 2D sketches and 3D parts | `part P() { }` |
| [module](README.md)                            | modularization of complex code              | `mod m { }`    |
| [if](../../flow/conditions.md)                 | process conditions                          | `if x > 1 { }` |
| [use](../use.md)                               | use elements from other modules             | `use m;`       |
| [call](../../flow/calls)                       | use functions and workbenches               | `f();`         |
| [comment](../../doc/comments.md)               | for documentation                           | `// comment`   |
