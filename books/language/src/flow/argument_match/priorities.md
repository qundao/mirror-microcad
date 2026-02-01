# Match Priorities

A single parameter can match with an argument in several ways and every way has a
*priority* which gets important when calling [workbenches](../calls/workbench_calls.md)
that have overloading capabilities at initialization.

| Priority<br>⭣ high to low | Matches                                                       | Example Parameters                  | Example Arguments |
| ------------------------- | ------------------------------------------------------------- | ----------------------------------- | ----------------- |
| Empty List                | Empty arguments with empty parameters                         | `()`                                | `()`              |
| Identifier                | Match argument identifier with parameter identifier           | `(x: Scalar)`                       | `(x=1)`           |
| Shortened Identifier      | Match argument identifier with shortened parameter identifier | <nobr>`(max_height: Scalar)`</nobr> | `(m_h=1)`         |
| Type                      | Match argument type with parameter type                       | `(x: Length)`                       | `(1mm)`           |
| Compatible Type           | Match argument type with compatible parameter type            | `(x: Scalar)`                       | `f(1)`            |
| Default                   | Match parameter defaults                                      | `(x=1mm)`                           | `()`              |

## Match Empty List

The first case when both arguments and parameters are empty is trivial.

## Match Identifier

The following example demonstrates a call to function `f` with each argument
specified by name:

[![test](.test/argument_match_name.svg)](.test/argument_match_name.log)

```µcad,argument_match_name
fn f( width: Length, height: Length ) -> Area { width * height }

x = f(height = 2cm, width = 1cm);   // call f() with parameters in arbitrary order

std::debug::assert_eq([ x, 2cm² ]);
```

## Match Short Identifier

Parameter names can also matched by their short identifier.

The short form is every first letter of every word between any underscores (`_`).
So the short form of `width` is `w` and from `inner_radius` it is `i_r`.

[![test](.test/argument_match_short.svg)](.test/argument_match_short.log)

```µcad,argument_match_short
fn f( width: Length, height: Length ) -> Area { width * height }

// use short identifiers
f(w = 1cm, h = 2cm);
// can be mixed
f(w = 1cm, inner_radius = 2cm);
```

Here are some usual examples:

| Identifier                    | Short Identifier |
| ----------------------------- | ---------------- |
| `parameter`                   | `p`              |
| `my_parameter`                | `m_p`            |
| `my_very_long_parameter_name` | `m_v_l_p_n`      |
| `my_Parameter`                | `m_P`            |
| `MyParameter`                 | `M`              |
| `myParameter`                 | `m`              |

## Match Type

It is possible to use nameless values if all the *parameter types* of a called
function (or workbench) differ in their types.

[![test](.test/argument_match_type.svg)](.test/argument_match_type.log)

```µcad,argument_match_type
fn f( a: Scalar, b: Length, c: Area ) -> Volume{}
// Who needs names?
f(1.0, 2cm, 3cm²);
```

## Match Compatible Type

Nameless arguments may also be not equal but compatible to *parameter types*.

[![test](.test/argument_match_type_compatible.svg)](.test/argument_match_type_compatible.log)

```µcad,argument_match_type_compatible
fn f( a: Scalar, b: Length, c: Area ) {}
// giving an integer `1` to a `Scalar` parameter `a`
f(1, 2cm, 3cm²);
```

## Match Default

If an argument was not given and it's parameter has a default defined then
this will be used as argument value.

[![test](.test/argument_match_default.svg)](.test/argument_match_default.log)

```µcad,argument_match_default
fn f( a = 1mm ) {}
// a has default
f();
```

## Mix'em all

You can mix both methods if some parameters cannot be identified by type alone.

[![test](.test/argument_match_mix.svg)](.test/argument_match_mix.log)

```µcad,argument_match_mix
fn f( a: Scalar, b: Length, c=2cm, d: Area ) {}
// `a` is the only Scalar and `b` is named, so `c` does not need a name.
f(1, b=2cm, 2cm², 3cm);
```
