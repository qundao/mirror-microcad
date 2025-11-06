# Attributes

Attributes are syntax elements that can be used to attach arbitrary data to model nodes.

The attributes will not change the model geometry itself, but might change its appearance when if they are used for viewers or exporters.
There can be multiple attributes for a node, but each attribute needs to have a unique ID.

## Simple example

Let's define a model node `c`.

When viewed or exported, model node `c` will have a red color, because the `color` attribute will be set:

[![test](.test/attributes_simple_example.svg)](.test/attributes_simple_example.log)

```µcad,attributes_simple_example
#[color = "#FF0000"]
c = std::geo2d::Circle(42.0mm);

std::debug::assert_eq([c#color, (r = 1.0, g = 0.0, b = 0.0, a = 1.0)]);
```

## Syntax

Syntactically, an attribute consists of `#` prefix and an item.

* *Name-value pair*: `#[color = "#FF00FF"]`, `#[resolution = 200%]`. Store and retrieve arbitrary values. The name has to be unique.

* *Calls*: `#[svg("style = fill:none")]`. Control the output for a specific exporter.

* *Commands*: `#[export = "test.svg"]`, `#[measure = width, height]`. A certain command to execute on a model, e.g. for export and measurement. Multiple commands are possible.

## Color attribute

The `color` attribute attaches a color to a model node.

In viewer and when exported, the model will be drawn in the specified color.

[![test](.test/attributes_color.svg)](.test/attributes_color.log)

```µcad,attributes_color
#[color = "#FFFFFF"]
c = std::geo2d::Circle(42.0mm);

std::debug::assert_eq([c#color, (r = 1.0, g = 1.0, b = 1.0, a = 1.0)]);
```

## Resolution attribute

The `resolution` attribute sets the rendering resolution of the model.
The model will be rendered in with 200% resolution than the default resolution of `0.1mm`.
This means the circle will be rendered with a resolution `0.05mm`.

[![test](.test/attributes_precision.svg)](.test/attributes_precision.log)

```µcad,attributes_precision
#[resolution = 200%]
c = std::geo2d::Circle(42.0mm);

std::debug::assert_eq([c#resolution, 200%]);
```

## Export command

If you have created a part or a sketch and want to export it to a specific file, you can add an `export` command:

[![test](.test/attributes_export.svg)](.test/attributes_export.log)

```µcad,attributes_export
#[export = "circle.svg"]
c = std::geo2d::Circle(42.0mm);
```

The exporter is detected automatically depending on the file extension.

See [export](export.md) for more information.
