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
