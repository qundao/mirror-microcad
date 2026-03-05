# Cylinder

Cylinder definition.

Examples:
* with radius `r` and height `h`: `Cylinder(r = 10.0mm, h = 5.0mm)`

## Parameters

- radius_bottom: Length
- radius_top: Length
- height: Length
- offset: Length = 0mm

## init(radius: Length, height: Length)

Initialize by `radius` and `height`. Cylinder will be centered.

## init(radius: Length, height: Length, offset: Length)

Initialize by `radius`, `height` and `offset`.

## init(radius: Length, bottom: Length, top: Length)

Initialize by `radius`, `bottom` and `top`.

## init(diameter: Length, height: Length)

Initialize by `diameter` and `height`. Cylinder will be centered.
