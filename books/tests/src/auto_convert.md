# Automatic conversion of values when matching arguments

[![test](.test/auto_convert.svg)](.test/auto_convert.log)

```µcad,auto_convert
fn f( x: Scalar ) { }
f(x=1);
f(x="");
f("");
f(1);   // error

fn g( x: Integer ) { }
g(x=1.0);
g(x="");
g("");
g(1.0);  // error

fn h( x: String ) { }
h(x=1.0);
h(x="");
h("");
h(1.0);  // error
```
