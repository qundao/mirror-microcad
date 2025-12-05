# Addressing of Corners & Edges

The idea behind this is that we can identify corners and edges by the viewer which
automatically displays stable names for all edges or corners of a model so you
can use them within the code.

```µcad
use std::geo2d::*;
Rect(10cm).corner(a).round(1mm);  // round one corner
Rect(10cm).edge(a).fillet(1mm); // create a fillet at one edge
```

This might also be done with multiplicity:

```µcad
use std::geo2d::*;
Rect(10cm).corner([a,b]).round(1mm);  // round two corners
Rect(10cm).edge([a..c]).fillet(1mm); // create a fillet at three edges
```
