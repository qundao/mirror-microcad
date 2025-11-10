# Use Local

```µcad,use_local
fn f() {
    use std::math::abs;
    x = abs(-1.0);
    return x;
}
f();
```

```µcad,use_all_local
fn f() {
    use std::math::*;
    x = abs(-1.0);
    return x;
}
f();
```
