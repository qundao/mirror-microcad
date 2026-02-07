# Command attributes

Command attributes are used to control the viewer or renderer with specific commands.
They do not alter the geometry, but they can have external effects, like writing something into a file.

## `export` command

The `export` command annotates a model with a filename and optional export parameters.

Assume you have created a model and want to export it to a specific file:

[![test](.test/attributes_export.svg)](.test/attributes_export.log)

```µcad,attributes_export#todo
#[export("rect.svg")] // Will be exported to `rect.svg`
std::geo2d::Rect(42mm);

#[export("circle.svg")] // Will be exported to `circle.svg`
std::geo2d::Circle(r = 42mm);
```

When this µcad source file is exported via the command line interface, the two models will be exported to `rect.svg` and `circle.svg`.
This way, you can export multiple file at once.

The exporter is detected automatically depending on the file extension.

See [export](../export.md) for more information.
