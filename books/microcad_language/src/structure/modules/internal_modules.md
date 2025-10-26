# Internal Modules

Internal modules are modules which are defined with the `mod` keyword followed
by a name and a body as the following example shows:

```µcad,internal_mod
mod my_module {
    /// statements
}
```

Module names are always written in 
[`lower_snake_case`](./../../appendix/coding.md).

The following statements can be listed in a module:

| Statement                                       | Purpose                                     | Example        |
| ----------------------------------------------- | ------------------------------------------- | -------------- |
| [const assignment](../../values/assignments.md) | store values                                | `const y = x;` |
| [function](../functions.md)                     | separate calculations                       | `fn f() { }`   |
| [workbench](../../workbenches)                  | build or transform 2D sketches and 3D parts | `part P() { }` |
| [module](../)                                   | modularization of complex code              | `mod m { }`    |
| [if](../../flow/conditions.md)                  | process conditions                          | `if x > 1 { }` |
| [use](../use.md)                                | use elements from other modules             | `use m;`       |
| [call](../../flow/calls)                        | use functions and workbenches               | `f();`         |
| [comment](../../doc/comments.md)                | for documentation                           | `// comment`   |
