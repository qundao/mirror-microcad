# Scope tests

## Scopes

[![test](.test/scopes.svg)](.test/scopes.log)

```µcad,scopes
a = 1;
use __builtin::debug::assert;
use __builtin::debug::is_valid;

assert(is_valid("a"));
assert(!is_valid("b"));
assert(!is_valid("c"));

{
    assert(is_valid("a"));
    assert(!is_valid("b"));
    assert(!is_valid("c"));

    b = 2;

    assert(is_valid("a"));
    assert(is_valid("b"));
    assert(!is_valid("c"));

    c = 3;

    assert(is_valid("a"));
    assert(is_valid("b"));
    assert(is_valid("c"));
};

assert(is_valid("a"));
assert(!is_valid("b"));
assert(!is_valid("c"));
```
