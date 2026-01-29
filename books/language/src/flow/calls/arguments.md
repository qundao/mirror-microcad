# Arguments

## Argument Matching

To match *arguments* with *function* or *workbench parameters*, µcad employs
a process known as *argument matching*.

### Match by Name

The following example demonstrates a call to function `f` with each argument
specified by name:

[![test](.test/argument_match_name.svg)](.test/argument_match_name.log)

```µcad,argument_match_name
fn f( x: Length, y: Length, z: Length ) {}

f(x = 1cm, y = 2cm, z = 3cm);
```

### Match by Short Identifier

Parameter names can also matched by their short identifier.

The short form is every first letter of every word between any underscores (`_`).
So the short form of `width` is `w` and from `inner_radius` it is `i_r`.

Here are some examples:

| Identifier                    | Short Identifier |
| ----------------------------- | ---------------- |
| `parameter`                   | `p`              |
| `my_parameter`                | `m_p`            |
| `my_very_long_parameter_name` | `m_v_l_p_n`      |
| `my_Parameter`                | `m_P`            |
| `MyParameter`                 | `M`              |
| `myParameter`                 | `m`              |

[![test](.test/argument_match_short.svg)](.test/argument_match_short.log)

```µcad,argument_match_short
// short identifiers of f's parameters are `w` and `i_r`
fn f( width: Length, inner_radius: Length ) {}

// use short identifiers
f(w = 1cm, i_r = 2cm);
// can be mixed
f(w = 1cm, inner_radius = 2cm);
```

### Match by Type

It is possible to use nameless values if all the *parameter types* of a called
function (or workbench) differ in their types.

[![test](.test/argument_match_type.svg)](.test/argument_match_type.log)

```µcad,argument_match_type
fn f( a: Scalar, b: Length, c: Area ) {}
// Who needs names?
f(1.0, 2cm, 3cm²);
```

### Mix Name and Type Matching

You can mix both methods if some parameters cannot be identified by type alone.

[![test](.test/argument_match_mix.svg)](.test/argument_match_mix.log)

```µcad,argument_match_mix
fn f( a: Scalar, b: Length, c: Length ) {}
// `a` is the only Scalar and `b` is named, so `c` does not need a name.
f(1.0, b=2cm, 3cm);
```

### Named Tuple Argument Matching

The concept behind named tuple argument matching is to allow functions to accept
subsets of parameters in a bundle.
This makes it easy to pre-configure parts of arguments:

[![test](.test/tuple_match.svg)](.test/tuple_match.log)

```µcad,tuple_match#todo
// Function with three parameters: x, y, and z
fn f( x: Length, y: Length, z: Length ) {}

// Since we do not want to change x and y in the following statements,
// we prepare a tuple named plane:
plane = (x=1cm, y=2cm);

// Then we pass plane to f() three times with different z values
f( plane, z=3cm);
f( plane, z=6cm);
f( plane, z=9cm);
```

The same function can be called in various ways using named tuples:

[![test](.test/tuple_match_variants.svg)](.test/tuple_match_variants.log)

```µcad,tuple_match_variants#todo
fn f( x: Length, y: Length, z: Length ) {}

// Every parameter given by name
f( x=1cm, y=2cm, z=3cm);

// Parameters given by named tuple
f( (x=1cm, y=2cm, z=3cm) );

// Parameters given by named tuple variable
p = (x=1cm, y=2cm, z=3cm);
f( p );

// Parameters given by named tuple and a single value
f( (x=1cm, y=2cm), z=3cm );
f( y=2cm, (x=1cm, z=3cm) );

// Parameters given by named tuple variable and a single value
q = (x=1cm, y=2cm);
f( q, z=3cm );
```

As you can see, the possibilities are endless.

### Matching Errors

If you do not provide all parameters, you will get an error:

[![test](.test/tuple_match_errors.svg)](.test/tuple_match_errors.log)

```µcad,tuple_match_errors#fail
fn f( x: Length, y: Length, z: Length ) {}
f( (x=1cm, v=2cm), z=3cm); // error: y is missing here
```

When you provide all parameters but some are redundant, you will get a warning:

[![test](.test/tuple_match_warnings.svg)](.test/tuple_match_warnings.log)

```µcad,tuple_match_warnings#fail
fn f( x: Length, y: Length, z: Length ) {}
f( (x=1cm, y=2cm, v=5cm), z=3cm);  // error: Missing arguments: x,y
```

## Automatic Identifier Matching

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
