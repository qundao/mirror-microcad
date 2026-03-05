# distribute_grid

Distribute geometries within a 2D rectangular grid.

The `distribute_grid` operator arranges elements evenly inside a rectangular region,
subdivided into a specified number of rows and columns. Each grid cell receives
one element, allowing for structured placement of objects, points, or geometries.

The grid is defined by its position (`x`, `y`), total dimensions (`width`, `height`),
and subdivision counts (`rows`, `columns`).

## Parameters

- x: Length
- y: Length
- width: Length
- height: Length
- rows: Integer
- columns: Integer

## init(width: Length, height: Length, rows: Integer, columns: Integer)

Initialize a centered rectangular grid.

## init(size: Length, rows: Integer, columns: Integer)

Initialize a centered square grid.

The `init` helper simplifies grid creation by specifying a single `size`
value instead of separate width and height. It automatically centers
the grid around the origin and defines its overall dimensions.

## init(cell_size: Length, rows: Integer, columns: Integer)

Initialize a centered grid with a squared cell size.

The `init` helper simplifies grid creation by specifying a single `cell_size`
value instead of separate width and height. It automatically centers
the grid around the origin and defines its overall dimensions.
