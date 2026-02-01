# Arguments

## Argument Matching

To match *call arguments* with *function* or *workbench parameters*, µcad employs
a process known as *argument matching*.

 > [!IMPORTANT]
 > Parameters in µcad are **not positional** (which means their order is irrelevant)!

A parameter can match with an argument in several ways and every way has a
priority which will get important with workbenches that have overloading
capabilities at initialization.

| Priority<br>⭣ high to low | Matches                                                       | Example Parameters                  | Example Arguments |
| ------------------------- | ------------------------------------------------------------- | ----------------------------------- | ----------------- |
| Empty List                | Empty arguments with empty parameters                         | `()`                                | `()`              |
| Identifier                | Match argument identifier with parameter identifier           | `(x: Scalar)`                       | `(x=1)`           |
| Shortened Identifier      | Match argument identifier with shortened parameter identifier | <nobr>`(max_height: Scalar)`</nobr> | `(m_h=1)`         |
| Type                      | Match argument type with parameter type                       | `(x: Length)`                       | `(1mm)`           |
| Compatible Type           | Match argument type with compatible parameter type            | `(x: Scalar)`                       | `f(1)`            |
| Default                   | Match parameter defaults                                      | `(x=1mm)`                           | `()`              |

### Match Empty List

The first case when both arguments and parameters are empty is trivial.

### Match Identifier

The following example demonstrates a call to function `f` with each argument
specified by name:

[![test](.test/argument_match_name.svg)](.test/argument_match_name.log)

```µcad,argument_match_name
fn f( x: Length, y: Length, z: Length ) {}

f(x = 1cm, y = 2cm, z = 3cm);
```

### Match Short Identifier

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

### Match Type

It is possible to use nameless values if all the *parameter types* of a called
function (or workbench) differ in their types.

[![test](.test/argument_match_type.svg)](.test/argument_match_type.log)

```µcad,argument_match_type
fn f( a: Scalar, b: Length, c: Area ) {}
// Who needs names?
f(1.0, 2cm, 3cm²);
```

### Match Compatible Type

Nameless arguments may also be not equal but compatible to *parameter types*.

[![test](.test/argument_match_type_compatible.svg)](.test/argument_match_type_compatible.log)

```µcad,argument_match_type_compatible
fn f( a: Scalar, b: Length, c: Area ) {}
// giving an integer `1` to a `Scalar` parameter `a`
f(1, 2cm, 3cm²);
```

### Match Default

If an argument was not given and it's parameter has a default defined then
this will be used as argument value.

[![test](.test/argument_match_default.svg)](.test/argument_match_default.log)

```µcad,argument_match_default
fn f( a = 1mm ) {}
// a has default
f();
```

### Mix'em all

You can mix both methods if some parameters cannot be identified by type alone.

[![test](.test/argument_match_mix.svg)](.test/argument_match_mix.log)

```µcad,argument_match_mix
fn f( a: Scalar, b: Length, c=2cm, d: Area ) {}
// `a` is the only Scalar and `b` is named, so `c` does not need a name.
f(1, b=2cm, 2cm², 3cm);
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
