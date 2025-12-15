# Argument matching

```µcad,argument_match_auto
fn f(x: Integer, y: Integer) -> Integer { x*y }
x = 1;
f(x, y=2); // matches because `x` and `y` match parameter names
```

```µcad,argument_match_single_identifier#todo
fn f(x: Integer, y: Integer) -> Integer { x*y }
x = 1;
y = 2;
f(x * 2, y * 2); // matches because `x` and `y` match parameter names
```
