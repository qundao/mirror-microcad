# Types

The µcad type system consists of a group of builtin types.
The type system is *static*, which means a every value has a fixed type
which cannot be changed or [overwritten](../assignments/value.md#no-shadowing).

Here is a complete list of the builtin types:

| Type                               | Description                            | Type Declarations                                                  | Example Values                                   |
| ---------------------------------- | -------------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------ |
| [*Boolean*](primitives.md#bool)    | Boolean value                          | `Bool`                                                             | `true`, `false`                                  |
| [*Integer*](primitives.md#integer) | Integer value without unit             | `Integer`                                                          | `4`, `-1`                                        |
| [*Scalar*](primitives.md#scalar)   | Floating point value without unit      | `Scalar`                                                           | `0.5`, `50%`, `-1.23e10`                         |
| [*Quantity*](quantities.md)        | Floating point value with unit         | `Length`, `Area`, `Volume`, `Density`, `Angle`, `Weight`           | `-4mm`, `1.3m2`, `2cm²`, `23.0e5deg`             |
| [*String*](primitives.md#string)   | UTF-8 text string                      | `String`                                                           | `"Hello, World!"`                                |
| [*Array*](collections/arrays.md)   | List of values with *common type*      | `[Integer]`                                                        | `[1,2,3]`, `[1m,2cm,3µm]`                        |
| [*Tuple*](collections/tuples/)     | List of named values or distinct types | `(Length,Scalar,Bool)`, `(x:Scalar,y:Length)`, `(x:Scalar,Length)` | `(4mm,4.0,true)`, `(x=4.0,y=4mm)`, `(x=4.0,4mm)` |
| [*Model*](models/)                 | Geometric 2D or 3D model               | `Model`                                                            | `std::geo3d::Cube(2mm)`                          |

## Declaration

The examples in the table above declare the type explicitly.
However, we can use units to declare the implicitly.
Using *units* is recommended and what you get in return is that declarations
are quite handy:

[![test](.test/types_def_vs_decl.svg)](.test/types_def_vs_decl.log)

```µcad,types_def_vs_decl
x: Length = 4mm;   // explicit type declaration
y = 4mm;           // implicit type declaration via units.
```

Declarations without any initializations are *not allowed* in µcad.
Hence, the following example will fail:

[![test](.test/types_no_declaration.svg)](.test/types_no_declaration.log)

```µcad,types_no_declaration#fail
x: Length;         // parse_error
```

However, for [parameter lists](../flow/calls/args_params.md#parameters) in
[functions](../structure/functions/) and [workbenches](../structure/workbenches/elements/),
you can declare the type only but also pass a default value:

[![test](.test/types_bundles_functions.svg)](.test/types_bundles_functions.log)

```µcad,types_bundles_functions
fn f(x = 4mm) {}       // use unit (with default)
fn f(x: Length) {}     // use type (without default)
```

> [!NOTE]
> Find out more about what types are used for in the sections about
> [*argument matching*](../flow/argument_match/) and [*assignments*](../assignments/).