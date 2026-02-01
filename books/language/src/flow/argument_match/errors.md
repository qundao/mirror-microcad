# Matching Errors

If you do not provide all parameters, you will get an error:

[![test](.test/tuple_match_errors.svg)](.test/tuple_match_errors.log)

```µcad,tuple_match_errors#fail
fn f( x: Length, y: Length, z: Length ) {}
f( x=1cm, z=3cm); // error: y is missing here
```

When you provide all parameters but some are redundant, you will get a error too:

[![test](.test/tuple_match_warnings.svg)](.test/tuple_match_warnings.log)

```µcad,tuple_match_warnings#fail
fn f( x: Length, y: Length, z: Length ) {}
f( x=1cm, y=2cm, v=5cm, z=3cm);  // error: Unexpected argument v
```
