# Modules

A module is a way to organize and group code into logical units.
Modules help manage scope and control visibility (e.g., using `pub` to make
items public available).

**In short:**

- Modules define a namespace for your code.
- They can be nested to create hierarchies.
- Declared with the `mod` keyword, either in separate files or directly in
  your code.
- Control what is exposed to the outside world using `pub`.

## Example

The following example shows two nested modules carrying a public constant and
a public function which then are used from outside the modules:

[![test](.test/mod_example.svg)](.test/mod_example.log)

```µcad,mod_example
// define private module
mod my {
    // define public module
    pub mod math {
        // define PI as a public constant value
        pub const PI = 3.14159;

        // define some public calculation function
        pub fn abs(x: Scalar) -> Scalar {
            if x < 0 { return -x; } else { return x; }
        }
    }
}

// use the function and the constant
x = my::math::abs(-1.0) * my::math::PI;
```
