# scale

An operation that scales a part.

See the `init`s for examples on how to use this operation.

## Parameters

- v: Vec3

## init(s: Scalar)

Uniform scale.

* `s` - Scale factor.

Examples:
* `Rect(42mm).scale(200%);`: Scale a rectangle by 200%.

## init(x: Scalar = 1, y: Scalar = 1, z: Scalar = 1)

Non-uniform scale.

* `x` - Scale factor in X direction.
* `y` - Scale factor in Y direction.
* `z` - Scale factor in Z direction.

Examples:
* `Rect(42mm).scale(x = 2.0);`: Scale a rectangle in X direction.
