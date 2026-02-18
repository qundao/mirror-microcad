# Calls

```µcad,call_same_parameter_name#fail
use std::geo2d::*;
use std::ops::*;

Rect(w = 10mm, w = 10mm); // error: same parameter name
```
