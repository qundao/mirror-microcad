# Operations

*Operations* process 2D or 3D geometries into 2D or 3D geometries.

Actual operations are workbenches that process input models into output models.

So the following `nop` operation would be a neutral operation:

[![test](.test/op_example.svg)](.test/op_example.log)

```µcad,op_example
// define operation nop without parameters
op nop() { @input }

// use operation nop on a circle
std::geo2d::Circle(radius = 1cm).nop();
```

## `@input`

`@input` is a placeholder to tell where the input nodes of the operation shall
be inserted.

An operation can have multiple children like in this example:

[![test](.test/input.svg)](.test/input.log)

```µcad,input#todo
// define operation which takes multiple items
op punched_disk() { 
    // check number of input models
    if @input.count() == 2 {
        // make hole by subtracting both inputs
        @input.subtract(); 
    } else {
        std::error("punched_disk must get exactly two objects");
    }
}

// use operation punch_disk with two circles
{
    std::geo2d::Circle(radius = 1cm);
    std::geo2d::Circle(radius = 2cm);
}.punched_disk();
```

Like other workbenches operations can have parameters too:

[![test](.test/parameters.svg)](.test/parameters.log)

```µcad,parameters#todo
// define operation which takes multiple items
op punch_disk(radius: Length) {
    if @input.count() == 1 {
        { 
            @input;
            std::geo2d::Circle(radius);
        }.subtract();
    } else {
        std::error("punch_disk must get one object");
    }
}

// use operation punch_disk on a circle
{
    std::geo2d::Circle(radius = 2cm);
}.punch_disk(radius = 1cm);
```
