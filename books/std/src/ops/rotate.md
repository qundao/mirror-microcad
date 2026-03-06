# rotate

An operation that rotates a part.

See the `init`s for examples on how to use this operation.

## Parameters

- matrix: Matrix3x3

## init(angle: Angle, axis = __builtin::math::Z)

Rotate around an axis by an angle using a 3D vector.

The `axis` vector will be normalized.

* `angle` - An angle.
* `axis` - An axis to rotate around (Z axis is default).

Examples:
* `Rect(42mm).rotate(45°);`: 2D rotation of a square by 45°.
* `Cube(42mm).rotate(45°, (x = 1, y = 1, z = 1)) ;`: Rotates a cube by 45° around a diagonal axis.
* `Cube(42mm).rotate(30°, Y);`: Rotates a cube by 30° around Y axis.

## init(x = 0°, y = 0°, z = 0°)

Euler rotation around X, Y, Z.

* `x` - X angle.
* `y` - Y angle.
* `z` - Z angle.

Examples:
* `Cube(42mm).rotate(45°, (1, 0, 0));`: Rotates a cylinder by 45° around X axis (unnamed tuple).

## init(roll = 0°, pitch = 0°, yaw = 0°)

Euler rotation around X, Y, Z (nautical angles)

* `roll` - Roll angle, rotates around X axis.
* `pitch` - Pitch angle, rotates around Y axis.
* `yaw` - Yaw angle, rotates around Z axis.

Example:
* `Cube(42mm).rotate(roll = 30°)`: Rotates a cylinder by 45° around X axis.

## init(xyz = (x = 0°, y = 0°, z = 0°))

Euler rotation around X, Y and Z axis (with a vector of angles)

* `Cube(42mm).rotate(xyz = (30°,20°,10°))`: Euler rotation for a cube around X, Y, Z axis.

## init(zyx = (x = 0°, y = 0°, z = 0°))

Euler rotation around Z, Y and X axis (with a vector of angles)

* `Cube(42mm).rotate(zyx = (30°,20°,10°));`: Euler rotation for a cube around Z, Y, X axis.
