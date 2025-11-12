# Automatic conversion of values when matching arguments

[![test](.test/auto_convert.svg)](.test/auto_convert.log)

```µcad,auto_convert#fail
fn f( x: Scalar ) { }
f(x=1);
f(x="");    // error
f("");      // error
f(1.0);
f(1);

fn g( x: Integer ) { }
g(x=1.0);   // error
g(x="");    // error
g("");      // error
g(1.0);     // error
g(1);

fn h( x: String ) { }
h(x=1.0);   // error
h(x="");
h("");
h(1.0);     // error
```
