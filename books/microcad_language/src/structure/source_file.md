# Source Files

*Source files* are files that contain microcad code.

A source file can include the following types of *statements* which we will all
discuss in this book:

- [Expressions](expressions/README.md)
- [Assignments](assignments.md)
- [Functions](functions.md)
- [Workbenches](workbenches/README.md)
- [Modules](modules/README.md)
- [If Statements](conditions.md)
- [Use Statements](use.md)
- [Calls](../flow/calls/README.md)
- [Comments](comments.md)

In the simplest case, a microcad program consists of a single file containing
one or more statements.  
These statements generate, for example, a 2D graphic or a 3D model — as shown
in the examples below, where a circle and a sphere are created, each with a
radius of one centimeter.

[![test](.test/source_file_2D.svg)](.test/source_file_2D.log)

```µcad,source_file_2D
// simply draw a circle
std::geo2d::Circle(radius = 1cm);
```

[![test](.test/source_file_3D.svg)](.test/source_file_3D.log)

```µcad,source_file_3D
// simply draw a sphere
std::geo3d::Sphere(radius = 1cm);
```

Statements within a source file can represent either a 2D or a 3D model — but
not both at the same time.
Mixing 2D and 3D statements in the same file will result in an error:

[![test](.test/source_file_mixed.svg)](.test/source_file_mixed.log)

```µcad,source_file_mixed#fail
std::geo2d::Circle(radius = 1cm);
std::geo3d::Sphere(radius = 1cm);  // error: can't mix 2D and 3D
```

However, microcad programs can also be split across multiple files.
To include other files, the [`mod`](modules/external_modules.md)
statement is used — but more on that later.
