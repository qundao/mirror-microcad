# InvoluteGearProfile

A parametric **involute spur gear** profile.

This type defines the geometric parameters for generating a 2D involute gear shape.


## See also

- [`involute_gear_tooth`](#method.involute_gear_tooth): Generates a single gear tooth.


## References

- [Involute Gear Geometry (Wikipedia)](https://en.wikipedia.org/wiki/Involute_gear)


## Arguments

- `module: Length`: Gear module (mm per tooth), controlling the overall gear size.
Defines the ratio between the pitch diameter and the number of teeth:
`pitch_diameter = module * teeth`.
- `teeth: Integer`: Total number of teeth on the gear. Must be a positive integer.
- `pressure_angle: Angle`: Pressure angle (radians or degrees), defining the shape of the involute flank.
The standard pressure angle of the gear, typically
`20°` or `25°`, which determines the shape of the involute profile and
the base circle radius.
