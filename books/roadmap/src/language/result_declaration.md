# Result Type Declaration

For better diagnosis and better readability functions in microcad shall have return type declarations similar to Rust:

```µcad
fn f() -> Integer { 1 }
```


```µcad
fn f(n: Bool) -> Integer { "1" } // error
```
