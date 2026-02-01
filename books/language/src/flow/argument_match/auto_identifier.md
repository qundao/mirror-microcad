# Automatic Identifier Matching

In some cases the name of the parameter is already included in an argument expression.
So if there is only one (or multiple identical) identifiers within an expression
and it (those) match an argument of the same type, this argument will be matched.

[![test](.test/argument_match_auto.svg)](.test/argument_match_auto.log)

```µcad,argument_match_auto
fn f(x: Integer, y: Integer) -> Integer { x*y }
x = 1;
f(x, y=2); // matches because argument `x` matches the name of parameter `x`
```

Even when using a more complex expression a unique identifier can match an argument:

[![test](.test/argument_match_single_identifier.svg)](.test/argument_match_single_identifier.log)

```µcad,argument_match_single_identifier
fn f(x: Integer, y: Integer) -> Integer { x*y }
x = 1;
y = 2;
f(x * 2, y * y); // matches because `x` and `y` match parameter names `x` and `y`
```

[![test](.test/argument_match_auto_err.svg)](.test/argument_match_auto_err.log)

```µcad,argument_match_auto_err#fail
fn f(x: Integer, y: Integer) -> Integer { x*y }
x = 1;
y = 2;
f(x * y, y * x); // error: `x` and `y` cannot be matched because they are not unique within the arguments.
```
