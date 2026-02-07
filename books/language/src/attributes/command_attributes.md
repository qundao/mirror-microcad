# Command attributes

## Export command

If you have created a part or a sketch and want to export it to a specific file, you can add an `export` command:

[![test](.test/attributes_export.svg)](.test/attributes_export.log)

```µcad,attributes_export
#[export = "circle.svg"]
c = std::geo2d::Circle(r = 42.0mm);
```

The exporter is detected automatically depending on the file extension.

See [export](export.md) for more information.
