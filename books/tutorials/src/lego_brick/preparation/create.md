# Create new µcad file

Before we design any geometry, we use the `microcad` command line tool to create a new µcad project:

```sh
microcad create lego_brick
```

This will create a file `lego_brick.µcad`.

Let's open this file in VSCode:

[![test](.test/create.svg)](.test/create.log)

```µcad,create
use std::geo2d::*;
use std::ops::*;

sketch Hello(height = 42mm) {
    RoundedRect(width = 4 * height, height, radius = 10mm) - Text(height, "Hello µcad!").center();
}

// create a 3D text
Hello().extrude(23mm);
```

We can export the file using the following command:

```sh
microcad export lego_brick
```

Nothing will be exported because the sketch does not contain any output geometry.
Therefore, let's add some!
