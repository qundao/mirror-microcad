# Start Code

A µcad program consists of one or multiple files.

Files can be added by the [`mod` statement](../structure/modules/external_modules.md)
or by adding library paths of external modules with command line options.

Currently[^no_project] every µcad program starts with the original input file
that was given at running `microcad`.
The **top level code** within this file is where microcad starts processing.

[![test](.test/start.svg)](.test/start.log)

```µcad,start
// start code begins here
use std::geo2d::*;

mod my_inner {
    pub RADIUS = 10mm;
}

Circle( radius = my_inner::RADIUS );
// start ends begins here
```

[^no_project]: In future µcad will get a package management and will have projects and toml files.
