# Statement Usage

The following table shows which elements (rows) may occur in which Context (columns):

| Context/Element        | Source File       | Module            | Initialization Code | Initializer     | Workbench            | Building code   | Function code       |
| ---------------------- | ----------------- | ----------------- | ------------------- | --------------- | -------------------- | --------------- | ------------------- |
| `sketch`, `part`, `op` | yes               | yes               | -                   | -               | -                    | -               | -                   |
| `mod`                  | yes               | yes               | -                   | -               | -                    | -               | -                   |
| `fn`                   | yes               | yes               | -                   | -               | -                    | -               | -                   |
| `init`                 | -                 | -                 | yes                 | -               | yes                  | -               | -                   |
| `use`                  | yes               | yes               | yes                 | yes             | yes                  | yes             | yes                 |
| `pub use`              | yes               | yes               | -                   | -               | -                    | -               | -                   |
| `return`               | -                 | -                 | -                   | -               | -                    | -               | yes                 |
| `if`                   | yes               | -                 | -                   | -               | yes                  | yes             | yes                 |
| `@input`               | -                 | -                 | -                   | -               | yes                  | ?yes?           | -                   |
| `x = 1`                | yes               | yes               | -                   | yes             | yes                  | yes             | yes                 |
| `const`                | yes               | yes               | -                   | -               | -                    | -               | -                   |
| `prop`                 | -                 | -                 | -                   | -               | yes                  | -               | -                   |
| *expression*           | yes               | -                 | -                   | -               | yes                  | yes             | yes                 |
|                        | [Test](source.md) | [Test](module.md) | [Test](pre-init.md) | [Test](init.md) | [Test](workbench.md) | [Test](body.md) | [Test](function.md) |