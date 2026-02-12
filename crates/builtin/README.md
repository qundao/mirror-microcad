# µcad Built-in Symbols

[![Crates.io](https://img.shields.io/crates/v/microcad-builtin.svg)](https://crates.io/crates/microcad-builtin)
[![Documentation](https://docs.rs/microcad-builtin/badge.svg)](https://docs.rs/microcad-builtin/)

This crate provides all modules that are built-in and implemented as Rust functions/traits and are available in the µcad language using the `__builtin` prefix.

This includes the following modules:

| Module              | Description                    | Examples                               |
| ------------------- | ------------------------------ | -------------------------------------- |
| `__builtin::math`   | Builtin mathematical functions | `sqrt`, `sin`, `cos`, `PI`             |
| `__builtin::geo2d`  | Builtin 2D primitives          | `Circle`, `Rect`                       |
| `__builtin::geo3d`  | Builtin 3D primitives          | `Sphere`, `Cube`                       |
| `__builtin::debug`  | Debugging functionality        | `assert`, `assert_eq`, `assert_valid`  |
| `__builtin::log`    | Debugging functionality        | `todo`, `error`, `info`                |

For information, please refer to the documentation for the builtin module [Language reference](http://docs.microcad.xyz/builtin/book/)

*Note: Most of this functionality will not be accessed directly in the language, but it is made accessable by the µcad standard library instead.*

## ❤️ Support the project

This crate is part of the [microcad project](https://microcad.xyz).

If you like this project, you can help us spend more time on it by donating:

<a href="https://opencollective.com/microcad/donate" target="_blank">
<img src="https://opencollective.com/microcad/donate/button@2x.png?color=blue" width=300 />
</a>
