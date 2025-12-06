# Paths

Paths are define a Line in a sketch or part.

They can be made out of arrays:

```µcad
// three point 2D path
p = [(0mm,0mm),(1mm,0mm),(1mm,1mm)];
```

Additionally a function can define the path.
In this case we need a new syntax construct which addresses the different parameters:

```µcad
path spiral(radius: Length, pitch: Scalar, turns: Scalar) {
    // will be the first `t`
    prop first = 0turns;
    // will be the last `t`
    prop last = turns * 1turn;
    // gets t and returns a coordinate
    fn eval(t: Angle) { (2mm*cos(t), 2,0mm* sin(t), pitch * t * 1,0mm) }
}
p = spiral(radius=2cm, pitch=0.5, turns=1.0);

// `p` can be used to generate points for the path at any desired resolution
p(resolution = 0.5turns);
// or just with a list of values
p([0.0, 0.5, 1.0]turns);
```

Both calls of `p` will generate an array of points which lay on the given path `p`.

This construct may need some more syntactic sugar which is open to discussion.
